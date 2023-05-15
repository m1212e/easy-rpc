use std::convert::Infallible;
use std::{collections::HashMap, net::SocketAddr, pin::Pin, sync::Arc};

use erpc::protocol::error::Error;
use erpc::protocol::{self, socket::SocketMessage};
use futures_util::{Future, SinkExt, StreamExt};
use http_body_util::Full;
use hyper::body::{Bytes, Incoming};
use hyper::header::UPGRADE;
use hyper::http::HeaderValue;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::upgrade::Upgraded;
use hyper::Response;
use hyper::StatusCode;
use log::error;
use parking_lot::RwLock;
use serde::{de::DeserializeOwned, Serialize};
use tokio::{net::TcpListener, sync::oneshot};
use tokio_tungstenite::WebSocketStream;

//TODO: check the channels for optimal tool for the problem (e.g. swithc to broadcast, mpsc where applicable)

use crate::{handler, Socket};

pub type InternalHandler = Box<
    dyn Fn(protocol::Request) -> Pin<Box<dyn Future<Output = protocol::Response> + Send + Sync>>
        + Send
        + Sync,
>;

type HandlerMap = Arc<RwLock<HashMap<String, InternalHandler>>>;

//TODO: check where rwlock/mutex is necessary
#[derive(Clone)]
pub struct Server {
    /**
      Shutdown signal to exit the webserver gracefully
    */
    shutdown_signal: Arc<RwLock<Option<oneshot::Sender<()>>>>,
    enabled_sockets: bool,
    allowed_cors_origins: Vec<String>,
    port: u16,
    handler_map: HandlerMap,
    socket_broadcaster: (flume::Sender<Socket>, flume::Receiver<Socket>),
}

impl Server {
    pub fn new(port: u16, allowed_cors_origins: Vec<String>, enabled_sockets: bool) -> Self {
        Self {
            shutdown_signal: Arc::new(RwLock::new(None)),
            enabled_sockets,
            allowed_cors_origins,
            port,
            handler_map: Arc::new(RwLock::new(HashMap::new())),
            socket_broadcaster: flume::unbounded(),
        }
    }

    pub fn socket_broadcaster(&self) -> &flume::Receiver<Socket> {
        &self.socket_broadcaster.1
    }

    //TODO implement a way to enable compile time handler registration
    #[allow(dead_code)]
    pub fn register_raw_handler(&self, handler: InternalHandler, identifier: String) {
        self.handler_map.write().insert(identifier, handler);
    }

    #[allow(dead_code)]
    pub fn register_handler<H, P>(&mut self, handler: H, identifier: &str)
    where
        H: handler::Handler<P> + 'static,
        P: DeserializeOwned + Send + Sync,
        H::Output: Serialize,
        H::Future: Future<Output = H::Output> + Send + Sync,
    {
        let v: InternalHandler = Box::new(move |request| {
            let handler = handler.clone();
            Box::pin(async move {
                //TODO there should be a macro for this
                let parameters = match serde_json::to_value(request.parameters) {
                    Ok(v) => v,
                    Err(err) => return Error::from(err).into(),
                };

                let parameters = match serde_json::from_value::<P>(parameters) {
                    Ok(v) => v,
                    Err(err) => return Error::from(err).into(),
                };

                let result = handler.call(parameters).await;

                let serialized = match serde_json::to_value(&result) {
                    Ok(v) => v,
                    Err(err) => return Error::from(err).into(),
                };

                protocol::Response {
                    body: Ok(serialized),
                }
            })
        });

        self.handler_map
            //TODO check if blocking here is a problem
            .write()
            .insert(identifier.to_string(), v);
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let addr = SocketAddr::from(([127, 0, 0, 1], self.port));
        let listener = TcpListener::bind(addr).await?;

        let handler_map = self.handler_map.clone();
        let enabled_sockets = self.enabled_sockets;
        let socket_broadcaster = self.socket_broadcaster.0.clone();
        let allowed_cors_origins = self.allowed_cors_origins.clone();
        loop {
            let handler_map = handler_map.clone();
            let socket_broadcaster = socket_broadcaster.clone();
            let (stream, _) = listener.accept().await?;
            let allowed_cors_origins = allowed_cors_origins.clone();

            tokio::task::spawn(async move {
                if let Err(err) = http1::Builder::new()
                    .serve_connection(
                        stream,
                        service_fn(move |req| {
                            let handler_map = handler_map.clone();
                            let socket_broadcaster = socket_broadcaster.clone();
                            let allowed_cors_origins = allowed_cors_origins.clone();
                            async move {
                                let headers = req.headers().clone();
                                let mut response = Server::http_handler(
                                    req,
                                    handler_map,
                                    enabled_sockets,
                                    socket_broadcaster,
                                )
                                .await;

                                // TODO put CORS code somewhere else
                                response.headers_mut().append(
                                    "Access-Control-Allow-Credentials",
                                    HeaderValue::from_static("true"),
                                );
                                response.headers_mut().append(
                                    "Access-Control-Allow-Methods",
                                    HeaderValue::from_static("POST"),
                                );
                                if allowed_cors_origins.contains(&"*".to_string()) {
                                    response.headers_mut().append(
                                        "Access-Control-Allow-Origin",
                                        HeaderValue::from_static("*"),
                                    );
                                } else if let Some(origin) = headers.get("Origin") {
                                    if let Ok(origin) = origin.to_str() {
                                        if allowed_cors_origins.contains(&origin.to_string()) {
                                            if let Ok(header_value) = HeaderValue::from_str(origin)
                                            {
                                                response.headers_mut().append(
                                                    "Access-Control-Allow-Origin",
                                                    header_value,
                                                );
                                            }
                                        }
                                    }
                                }

                                Ok::<_, Infallible>(response)
                            }
                        }),
                    )
                    .with_upgrades()
                    .await
                {
                    error!("Error serving http1 connection: {:?}", err);
                }
            });
        }
    }

    pub fn stop(&self) -> Result<(), String> {
        let mut w = self.shutdown_signal.write();
        let sender = match w.take() {
            Some(v) => v,
            None => {
                return Err("Server is not running".to_string());
            }
        };

        match sender.send(()) {
            Ok(_) => {}
            Err(err) => {
                return Err(format!(
                    "Server can't be stopped because of error: {:#?}",
                    err
                ));
            }
        };

        Ok(())
    }

    async fn http_handler(
        request: hyper::Request<Incoming>,
        handlers: HandlerMap,
        enabled_sockets: bool,
        socket_broadcaster: flume::Sender<Socket>,
    ) -> hyper::Response<Full<Bytes>> {
        if enabled_sockets && request.uri().path().starts_with("/ws/") {
            //TODO check if role exists
            let mut role = request.uri().path().strip_prefix("/ws/").unwrap();
            if role.ends_with('/') {
                role = role.strip_suffix('/').unwrap();
            }
            let role = role.to_string();

            if !request.headers().contains_key(UPGRADE) {
                return Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(Full::new(Bytes::new()))
                    .unwrap();
            }

            tokio::task::spawn(async move {
                match hyper::upgrade::on(request).await {
                    Ok(upgraded) => {
                        let socket = match tokio_tungstenite::accept_async(upgraded).await {
                            Ok(v) => v,
                            Err(err) => {
                                error!("Could not accept upgraded connection: {}", err);
                                return;
                            }
                        };

                        Server::handle_socket(socket, handlers, role, socket_broadcaster).await
                    }
                    Err(e) => error!("upgrade error: {}", e),
                }
            });

            return Response::builder()
                .header(UPGRADE, HeaderValue::from_static("websocket"))
                .status(StatusCode::SWITCHING_PROTOCOLS)
                .body(Full::new(Bytes::new()))
                .unwrap();
        }

        let request = match protocol::Request::try_from_hyper_request(request).await {
            Ok(v) => v,
            Err(err) => {
                return err.into();
            }
        };

        let handler_result = Server::handle_request(request, handlers).await;
        match handler_result.try_into() {
            Ok(v) => v,
            Err(err) => err.into(),
        }
    }

    async fn handle_socket(
        socket: WebSocketStream<Upgraded>,
        handlers: HandlerMap,
        role: String,
        socket_broadcaster: flume::Sender<Socket>,
    ) {
        //TODO this section definitely needs an overhaul to reduce complexity

        // the actual native socket channel
        let (mut socket_send, mut socket_recieve) = socket.split();

        // the channels to pass into the socket struct for use outside of this function, e.g. for targets to send requests
        let (requests_sender, requests_reciever) = flume::unbounded::<protocol::socket::Request>();
        let (responses_sender, responses_reciever) =
            flume::unbounded::<protocol::socket::Response>();

        //TODO there must be a more elegant way to do this
        // since the WebSocketSteam is not cloneable an we need to send to the socket from multiple places, this wrapper channel and
        // a worker process exist which allows us to pass messages into the socket from multiple senders
        let (socket_outgoing_sender, socket_outgoing_reciever) =
            flume::unbounded::<protocol::socket::SocketMessage>();
        tokio::spawn(async move {
            while let Ok(message) = socket_outgoing_reciever.recv_async().await {
                socket_send
                    .send(match message.try_into() {
                        Ok(v) => v,
                        Err(err) => {
                            error!("Could not send message via socket: {:?}", err);
                            continue;
                        }
                    })
                    .await
                    .unwrap();
            }
        });

        // send requests to the client
        tokio::spawn({
            let socket_outgoing_sender = socket_outgoing_sender.clone();
            async move {
                while let Ok(request) = requests_reciever.recv_async().await {
                    socket_outgoing_sender
                        .send(SocketMessage::Request(request))
                        .unwrap();
                }
            }
        });

        // broadcast the creation of a socket (e.g. to create a new target which can wrap around the socket)
        match socket_broadcaster
            .send_async(Socket {
                responses: responses_reciever,
                requests: requests_sender,
                role,
            })
            .await
        {
            Ok(_) => {}
            Err(err) => {
                error!("Could not broadcast socket: {:#?}", err);
                return;
            }
        }

        while let Some(message) = socket_recieve.next().await {
            let message = match message {
                Ok(v) => v,
                Err(err) => {
                    error!("Error while recieving websocket message: {:?}", err);
                    continue;
                }
            };
            let message = match protocol::socket::SocketMessage::try_from_socket_message(message) {
                Ok(v) => match v {
                    Some(v) => v,
                    None => continue,
                },
                Err(err) => {
                    error!("Could not parse websocket message: {:?}", err);
                    continue;
                }
            };

            match message {
                SocketMessage::Request(req) => {
                    let response = Server::handle_request(req.request, handlers.clone()).await;

                    match socket_outgoing_sender.send(SocketMessage::Response(
                        protocol::socket::Response {
                            id: req.id,
                            response,
                        },
                    )) {
                        Ok(_) => {}
                        Err(err) => {
                            error!("Could not send response: {:?}", err);
                        }
                    }
                }
                SocketMessage::Response(response) => match responses_sender.send(response) {
                    Ok(_) => {}
                    Err(err) => {
                        error!("Could not send response on response channel: {:?}", err);
                    }
                },
            }
        }
    }

    async fn handle_request(
        request: protocol::Request,
        handlers: HandlerMap,
    ) -> protocol::Response {
        let handler_result_future = {
            let lock = handlers.read();
            let handler = match lock.get(&request.identifier).ok_or(Error::NotFound) {
                Ok(v) => v,
                Err(err) => return err.into(),
            };

            handler(request)
        };

        handler_result_future.await
    }
}

use std::{collections::HashMap, net::SocketAddr, pin::Pin, sync::Arc};

use erpc::protocol::{self, socket::SocketMessage};
use futures_util::{Future, SinkExt, StreamExt};
use hyper::client::conn::http2;
use hyper::server::conn::http1;
use log::{error, warn};
use parking_lot::RwLock;
use serde::{de::DeserializeOwned, Serialize};
use tokio::{net::TcpListener, sync::oneshot};
use hyper::service::service_fn;

use crate::{handler, Socket};

pub type InternalHandler = Box<
    dyn Fn(
            protocol::Request,
        )
            -> Pin<Box<dyn Future<Output = Result<protocol::Response, String>> + Send + Sync>>
        + Send
        + Sync,
>;

type SocketChannel = (flume::Sender<Socket>, flume::Receiver<Socket>);
type HandlerMap = Arc<RwLock<HashMap<String, InternalHandler>>>;

//TODO: check where rwlock/mutex is necessary
#[derive(Clone)]
pub struct Server {
    /**
      Shutdown signal to exit the webserver gracefully
    */
    shutdown_signal: Arc<RwLock<Option<oneshot::Sender<()>>>>,
    /**
      Channel to broadcast connected sockets
    */
    socket_channel: SocketChannel,
    enabled_sockets: bool,
    allowed_cors_origins: Vec<String>,
    port: u16,
    handlers: HandlerMap,
}

impl Server {
    pub fn new(port: u16, allowed_cors_origins: Vec<String>, enabled_sockets: bool) -> Self {
        Self {
            shutdown_signal: Arc::new(RwLock::new(None)),
            socket_channel: flume::unbounded(),
            enabled_sockets,
            allowed_cors_origins,
            port,
            handlers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    //TODO implement a way to enable compile time handler registration
    #[allow(dead_code)]
    pub fn register_raw_handler(&self, handler: InternalHandler, identifier: String) {
        self.handlers.write().insert(identifier, handler);
    }

    #[allow(dead_code)]
    pub fn register_handler<H, P>(&mut self, handler: H, identifier: &str)
    where
        H: handler::Handler<P> + 'static,
        P: DeserializeOwned + Send + Sync,
        H::Output: Serialize,
        H::Future: Future<Output = H::Output> + Send + Sync,
    {
        let v: InternalHandler = Box::new(move |parameters| {
            let handler = handler.clone();
            Box::pin(async move {
                let parameters = serde_json::to_value(parameters)
                    .map_err(|err| format!("Could not convert to value: {err}"))?;

                let parameters = serde_json::from_value::<P>(parameters)
                    .map_err(|err| format!("Could not parse parameters: {err}"))?;

                let result = handler.call(parameters).await;

                let serialized = serde_json::to_value(&result)
                    .map_err(|err| format!("Could not serialize response: {err}"))?;

                Ok(protocol::Response { body: serialized })
            })
        });

        self.handlers
            //TODO: remove unwrap
            //TODO check if blocking here is a problem
            .write()
            .insert(identifier.to_string(), v);
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let addr = SocketAddr::from(([127, 0, 0, 1], self.port));
        let listener = TcpListener::bind(addr).await?;
        loop {
            let (stream, _) = listener.accept().await?;

            tokio::task::spawn(async move {
                if let Err(err) = http1::Builder::new()
                    .serve_connection(stream, service_fn(hello))
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

    pub fn get_socket_notifier(&self) -> &flume::Receiver<Socket> {
        &self.socket_channel.1
    }

    async fn http_handler(
        handlers: HandlerMap,
        path: Peek,
        parameters: Bytes,
    ) -> warp::reply::Response {
        let mut response = warp::reply::Response::default();
        *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;

        // introduce extra scope to drop the lock handle before await is called
        let handler_fut = {
            let lock = handlers.read();

            let handler = match lock.get(path.as_str()) {
                Some(v) => v,
                None => {
                    error!("Could not find a handler for path: {}", path.as_str());
                    *response.status_mut() = StatusCode::NOT_FOUND;
                    return response;
                }
            };

            handler(protocol::Request {
                identifier: path.as_str().to_string(),
                parameters: match serde_json::from_slice(&parameters) {
                    Ok(v) => v,
                    Err(err) => {
                        error!("Could not parse request parameters: {}", err);
                        *response.status_mut() = StatusCode::BAD_REQUEST;
                        return response;
                    }
                },
            })
        };

        let result = match handler_fut.await {
            Ok(v) => v,
            Err(err) => {
                error!("Handler call errored: {err}");
                return response;
            }
        };

        *response.status_mut() = StatusCode::OK;
        *response.body_mut() = Body::from(match serde_json::to_vec(&result.body) {
            Ok(v) => v,
            Err(err) => {
                error!("Could not serialize response body: {err}");
                return response;
            }
        });

        response
    }

    fn socket_handler(
        role: String,
        enabled_sockets: bool,
        socket_channel: SocketChannel,
        ws: warp::ws::Ws,
    ) -> warp::reply::Response {
        //TODO ideally check what roles are allowed here
        if !enabled_sockets {
            warn!("Tried to connect to disabled websocket server");
            return warp::reply::with_status(
                "Tried to connect to disabled websocket server",
                StatusCode::NOT_IMPLEMENTED,
            )
            .into_response();
        }

        ws.on_upgrade(|socket| async move {
            let (mut socket_sender, mut socket_reciever) = socket.split();
            let (incoming_sender, incoming_reciever) = flume::unbounded::<SocketMessage>();
            let (outgoing_sender, outgoing_reciever) = flume::unbounded::<SocketMessage>();

            tokio::spawn(async move {
                while let Some(message) = socket_reciever.next().await {
                    let message = match message {
                        Ok(v) => {
                            let m: SocketMessage = match serde_json::from_slice(v.as_bytes()) {
                                Ok(v) => v,
                                Err(err) => {
                                    eprintln!("Websocket message parse error: {err}");
                                    break;
                                }
                            };
                            m
                        }
                        Err(err) => {
                            eprintln!("Websocket message error: {err}");
                            break;
                        }
                    };

                    match incoming_sender.send(message) {
                        Ok(_) => {}
                        Err(err) => {
                            eprintln!("Could not broadcast incoming socket message: {err}")
                        }
                    };
                }
            });

            tokio::spawn(async move {
                loop {
                    let message = match outgoing_reciever.recv_async().await {
                        Ok(v) => v,
                        Err(err) => {
                            eprintln!("Error while processing outgoing socket message: {err}");
                            break;
                        }
                    };

                    let text = match serde_json::to_string(&message) {
                        Ok(v) => v,
                        Err(err) => {
                            eprintln!("Could not serialize ws message: {err}");
                            break;
                        }
                    };
                    socket_sender
                        .send(warp::ws::Message::text(text))
                        .await
                        .unwrap();
                }
            });

            socket_channel
                .0
                .send_async(Socket {
                    sender: outgoing_sender.clone(),
                    reciever: incoming_reciever.clone(),
                    role: role.clone(),
                })
                .await
                .unwrap();
        })
        .into_response()
    }
}

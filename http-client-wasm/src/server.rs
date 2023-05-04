use std::sync::{Arc, RwLock};

use erpc::protocol::socket::SocketMessage;
use futures_util::{Future, SinkExt, StreamExt};
use log::{error, warn};
use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::oneshot;
use warp::{
    hyper::{Body, Method, StatusCode},
    path::Peek,
    Filter,
};

use crate::Socket;

//TODO: include in docs that credentials are sent by default

type SocketChannel = (flume::Sender<Socket>, flume::Receiver<Socket>);

//TODO: check where rwlock/mutex is necessary
#[derive(Clone)]
pub struct Server {
    /**
     * The erpc server
     */
    server: erpc::Server,
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
}

impl Server {
    pub fn new(port: u16, allowed_cors_origins: Vec<String>, enabled_sockets: bool) -> Self {
        Self {
            server: erpc::Server::new(),
            shutdown_signal: Arc::new(RwLock::new(None)),
            socket_channel: flume::unbounded(),
            enabled_sockets,
            allowed_cors_origins,
            port,
        }
    }

    #[allow(dead_code)]
    pub fn register_raw_handler(
        &mut self,
        handler: erpc::server::InternalHandler,
        identifier: &str,
    ) {
        self.server.register_raw_handler(handler, identifier)
    }

    #[allow(dead_code)]
    pub fn register_handler<H, P>(&mut self, handler: H, identifier: &str)
    where
        H: erpc::Handler<P> + 'static,
        P: DeserializeOwned + Send + Sync,
        H::Output: Serialize,
        H::Future: Future<Output = H::Output> + Send + Sync,
    {
        self.server.register_handler(handler, identifier)
    }

    pub fn run(&self) -> Result<impl futures_util::Future<Output = ()> + Send + Sync, String> {
        let enabled_sockets = self.enabled_sockets;
        let socket_channel = self.socket_channel.clone();
        let server = self.server.clone();

        let socket_channel = warp::any().map(move || socket_channel.clone());
        let enabled_sockets = warp::any().map(move || enabled_sockets);

        let mut cors = warp::cors()
            .allow_methods(vec![Method::GET, Method::POST])
            .allow_credentials(true);

        if self.allowed_cors_origins.contains(&"*".to_string()) {
            cors = cors.allow_any_origin();
        } else {
            for origin in &self.allowed_cors_origins {
                cors = cors.allow_origin(origin.as_str());
            }
        }

        let http = warp::path!("handlers" / ..)
            .and(warp::any().map(move || server.clone()))
            .and(warp::path::peek())
            .and(warp::body::json())
            .and(warp::body::content_length_limit(1024 * 64))
            .then(Self::http_handler)
            .with(cors.clone());

        let ws = warp::path!("ws" / String)
            .and(enabled_sockets)
            .and(socket_channel)
            .and(warp::ws())
            .map(|role, enabled_sockets, socket_channel, ws| {
                Self::socket_handler(role, enabled_sockets, socket_channel, ws)
            })
            .with(cors.clone());

        let (sender, reciever) = oneshot::channel::<()>();
        self.shutdown_signal
            .write()
            .map_err(|err| format!("Could not set shutdown signal: {err}"))?
            .replace(sender);

        let (_, server) = warp::serve(http.or(ws).with(cors)).bind_with_graceful_shutdown(
            ([127, 0, 0, 1], self.port),
            async {
                reciever.await.ok();
            },
        );

        Ok(server)
    }

    pub fn stop(&self) -> Result<(), String> {
        let mut w = self
            .shutdown_signal
            .write()
            .map_err(|err| format!("Could not set shutdown signal: {err}"))?;
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

    /**
      A channel containing all previously connected sockets and broadcasting new socket connections
    */
    pub fn get_socket_notifier(&self) -> &flume::Receiver<Socket> {
        &self.socket_channel.1
    }

    //TODO remove return type of Box<dyn Reply> and replace with static types
    async fn http_handler(
        server: erpc::Server,
        path: Peek,
        parameters: Vec<serde_json::Value>,
    ) -> warp::reply::Response {
        let result = server
            .process_request(erpc::protocol::Request {
                identifier: path.as_str().to_string(),
                parameters,
            })
            .await
            .and_then(|v| {
                serde_json::to_vec(&v)
                    .map_err(|err| format!("Could not serialize response: {}", err))
            });

        let mut response = warp::reply::Response::default();
        match result {
            Ok(v) => {
                *response.status_mut() = StatusCode::OK;
                *response.body_mut() = Body::from(v);
            }
            Err(err) => {
                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                error!("Result errored: {err}");
            }
        }

        response
    }

    fn socket_handler(
        role: String,
        enabled_sockets: bool,
        socket_channel: SocketChannel,
        ws: warp::ws::Ws,
    ) -> warp::reply::Response {
        let mut response = warp::reply::Response::default();
        if !enabled_sockets {
            *response.status_mut() = StatusCode::NOT_IMPLEMENTED;
            warn!("Tried to connect to disabled websocket server");
            return response;
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
                                    return;
                                }
                            };
                            m
                        }
                        Err(err) => {
                            eprintln!("Websocket message error: {err}");
                            return;
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
                            return;
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
        });

        *response.status_mut() = StatusCode::OK;
        response
    }
}

use std::convert::Infallible;
use std::{collections::HashMap, net::SocketAddr, pin::Pin, sync::Arc};

use erpc::protocol::{self, socket::SocketMessage};
use futures_util::{Future, SinkExt, StreamExt};
use http_body_util::Full;
use hyper::body::{Bytes, Incoming};
use hyper::http::Error;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{client::conn::http2, Response};
use log::{error, warn};
use parking_lot::RwLock;
use serde::{de::DeserializeOwned, Serialize};
use tokio::{net::TcpListener, sync::oneshot};
use tokio_tungstenite::accept_async;

use crate::{handler, Socket};

pub type InternalHandler = Box<
    dyn Fn(protocol::Request) -> Pin<Box<dyn Future<Output = protocol::Response> + Send + Sync>>
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
    handler_map: HandlerMap,
}

impl Server {
    pub fn new(port: u16, allowed_cors_origins: Vec<String>, enabled_sockets: bool) -> Self {
        Self {
            shutdown_signal: Arc::new(RwLock::new(None)),
            socket_channel: flume::unbounded(),
            enabled_sockets,
            allowed_cors_origins,
            port,
            handler_map: Arc::new(RwLock::new(HashMap::new())),
        }
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
                let parameters = match serde_json::to_value(request.parameters) {
                    Ok(v) => v,
                    Err(err) => {
                        return protocol::Response {
                            body: Err(err.into()),
                        }
                    }
                };

                let parameters = match serde_json::from_value::<P>(parameters) {
                    Ok(v) => v,
                    Err(err) => {
                        return protocol::Response {
                            body: Err(err.into()),
                        }
                    }
                };

                let result = handler.call(parameters).await;

                let serialized = match serde_json::to_value(&result) {
                    Ok(v) => v,
                    Err(err) => {
                        return protocol::Response {
                            body: Err(err.into()),
                        };
                    }
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
        loop {
            let handler_map = handler_map.clone();
            let (stream, _) = listener.accept().await?;
            tokio::task::spawn(async move {
                if let Err(err) = http1::Builder::new()
                    .serve_connection(
                        stream,
                        service_fn(|req| {
                            let handler_map = handler_map.clone();
                            async move {
                                Ok::<_, Infallible>(Server::http_handler(req, handler_map).await)
                            }
                        }),
                    )
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
        request: hyper::Request<Incoming>,
        handlers: HandlerMap,
    ) -> hyper::Response<Full<Bytes>> {
        if request.uri().path().starts_with("/ws/") {
            let ws_stream = match accept_async(request.).await {
                Ok(ws_stream) => ws_stream,
                Err(e) => {
                    eprintln!("Failed to accept WebSocket connection: {}", e);
                    return Ok(Response::new(Body::empty()));
                }
            };

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

    async fn handle_request(
        request: protocol::Request,
        handlers: HandlerMap,
    ) -> protocol::Response {
        let handler_result_future = {
            let handlers_lock = handlers.read();
            let handler = match handlers_lock.get(&request.identifier) {
                Some(v) => v,
                None => {
                    return protocol::Response {
                        body: Err(protocol::error::Error::NotFound),
                    };
                }
            };

            handler(request)
        };

        handler_result_future.await
    }
}

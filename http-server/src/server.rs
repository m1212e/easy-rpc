//TODO: check the channels for optimal tool for the problem (e.g. swithc to broadcast, mpsc where applicable)

use std::{collections::HashMap, pin::Pin, sync::Arc};

use erpc::protocol::{self, socket::SocketMessage, SendableError};
use futures_util::Future;
use log::error;
use parking_lot::RwLock;
use reqwest::Method;
use salvo::{catcher::Catcher, cors::Cors, prelude::*};
use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::oneshot;

use crate::handler;

pub type InternalHandler = Box<
    dyn Fn(protocol::Request) -> Pin<Box<dyn Future<Output = protocol::Response> + Send + Sync>>
        + Send
        + Sync,
>;

type HandlerMap = Arc<RwLock<HashMap<String, InternalHandler>>>;
type SocketBroadcaster = (flume::Sender<Socket>, flume::Receiver<Socket>);

#[derive(Clone, Debug)]
pub struct Socket {
    pub requests: flume::Sender<erpc::protocol::socket::Request>,
    pub responses: flume::Receiver<erpc::protocol::socket::Response>,
    pub role: String,
}

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
    socket_broadcaster: SocketBroadcaster,
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

    pub fn get_socket_broadcaster(&self) -> &flume::Receiver<Socket> {
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
                let parameters = match serde_json::to_value(request.parameters) {
                    Ok(v) => v,
                    Err(err) => return SendableError::from(err).into(),
                };
                let parameters = match serde_json::from_value::<P>(parameters) {
                    Ok(v) => v,
                    Err(err) => return SendableError::from(err).into(),
                };
                let result = handler.call(parameters).await;
                let serialized = match serde_json::to_value(&result) {
                    Ok(v) => v,
                    Err(err) => return SendableError::from(err).into(),
                };
                serialized.into()
            })
        });

        self.handler_map
            //TODO check if blocking here is a problem
            .write()
            .insert(identifier.to_string(), v);
    }

    pub async fn run(&self) -> impl Future<Output = ()> {
        let (tx, rx) = oneshot::channel::<()>();
        self.shutdown_signal.write().replace(tx);

        let mut cors_handler = Cors::new()
            .allow_methods(vec![Method::POST, Method::OPTIONS])
            .allow_headers("*");

        if self.allowed_cors_origins.contains(&"*".to_string()) {
            cors_handler = cors_handler.allow_origin("*");
        } else {
            cors_handler = cors_handler.allow_origin(&self.allowed_cors_origins.clone());
        }

        let mut router = Router::with_hoop(affix::inject(self.handler_map.clone())).push(
            Router::with_hoop(cors_handler.into_handler())
                .options(salvo::handler::empty())
                .path(format!(
                    "{}/<**identifier>",
                    protocol::routes::HANDLERS_ROUTE
                ))
                .post(request_handler),
        );

        if self.enabled_sockets {
            router = router.push(
                Router::with_hoop(affix::inject(self.socket_broadcaster.clone()))
                    .path(format!("{}/<*role>", protocol::routes::WEBSOCKETS_ROUTE))
                    .handle(socket_handler),
            );
        }

        // let config = RustlsConfig::new(None);
        // let listener = TcpListener::new(("0.0.0.0", self.port));
        // let acceptor = QuinnListener::new(config, ("0.0.0.0", self.port))
        //     .join(listener)
        //     .bind()
        //     .await;

        let acceptor = TcpListener::new(("0.0.0.0", self.port)).bind().await;

        salvo::Server::new(acceptor).serve_with_graceful_shutdown(
            Service::new(router).catcher(Catcher::default().hoop(error_handler)),
            async {
                rx.await.ok();
            },
            None,
        )
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
}

#[handler]
async fn request_handler(
    req: &mut Request,
    depot: &mut Depot,
) -> Result<protocol::Response, protocol::SendableError> {
    let response = {
        let identifier = req
            .param::<String>("**identifier")
            .ok_or("Could not read identifier from path")?;

        let req = protocol::Request::try_from_salvo_request(req, identifier).await?;

        let handlers = depot
            .obtain::<HandlerMap>()
            .ok_or("Could not obtain handler map")?
            .read();

        let handler = handlers
            .get(&req.identifier)
            .ok_or(protocol::SendableError::NotFound)?;

        handler(req)
    };

    Ok(response.await)
}

#[handler]
async fn socket_handler(
    req: &mut Request,
    res: &mut Response,
    depot: &mut Depot,
) -> Result<(), protocol::SendableError> {
    let role = req
        .param::<String>("*role")
        .ok_or("Could not read role from path")?;

    let handlers = depot
        .obtain::<HandlerMap>()
        .ok_or("Could not obtain handler map")?
        .clone();

    let socket_broadcaster = depot
        .obtain::<SocketBroadcaster>()
        .ok_or("Could not obtain socket broadcaster")?
        .clone();

    // interfaces for this socket, they mirror requests of this socket 1:1
    let (requests_sender, requests_reciever) = flume::unbounded::<protocol::socket::Request>();
    let (responses_sender, responses_reciever) = flume::unbounded::<protocol::socket::Response>();

    let socket = Socket {
        responses: responses_reciever,
        requests: requests_sender.clone(),
        role,
    };

    socket_broadcaster
        .0
        .send_async(socket)
        .await
        .map_err(|err| format!("Could not broadcast socket: {}", err))?;

    WebSocketUpgrade::new()
        .upgrade(req, res, |mut ws| async move {
            loop {
                tokio::select! {
                    Some(msg) = ws.recv() => {
                        let msg = if let Ok(msg) = msg {
                            msg
                        } else {
                            return;
                        };

                        let msg = match SocketMessage::try_from(msg) {
                            Ok(v) => v,
                            Err(err) => {
                                error!("Could not parse incoming socket request: {:?}", err);
                                return;
                            }
                        };

                        match msg {
                            SocketMessage::Request(r) => {
                                let response = {
                                    let handlers = handlers.read();
                                    let handler = match handlers.get(&r.request.identifier) {
                                        Some(v) => v,
                                        None => {
                                            if let Err(err) = responses_sender.send(
                                                protocol::socket::Response::from_response(protocol::SendableError::NotFound.into(), &r.id)
                                            ) {
                                                error!("Could not send response: {:?}", err);
                                            };
                                            return;
                                        },
                                    };

                                    handler(r.request)
                                };

                                if let Err(err) = responses_sender.send_async(protocol::socket::Response::from_response(response.await, &r.id)).await {
                                    error!("Could not send response: {:?}", err);
                                };
                            },
                            SocketMessage::Response(r) => {
                                if let Err(err) = responses_sender.send_async(r).await {
                                    error!("Could not send response: {:?}", err);
                                };
                            },
                        };
                    }
                    Ok(req) = requests_reciever.recv_async() => {
                        let message: salvo::websocket::Message =
                            match protocol::socket::SocketMessage::Request(req).try_into() {
                                Ok(v) => v,
                                Err(err) => {
                                    error!("Could not convert request to websocket message: {:?}", err);
                                    return;
                                }
                            };

                        if ws.send(message).await.is_err() {
                            return;
                        }
                    }
                }
            }
        })
        .await?;

    Ok(())
}

// this is used to remove the default error page, which is salvo branded
#[handler]
async fn error_handler(res: &mut Response, ctrl: &mut FlowCtrl) {
    ctrl.skip_rest();
}

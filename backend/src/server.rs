use super::{protocol::socket::SocketMessage, Socket};
use futures_util::{Future, SinkExt, StreamExt};
use reqwest::{Method, StatusCode};
use serde::{de::DeserializeOwned, Serialize};
use std::{
  collections::HashMap,
  pin::Pin,
  sync::{Arc, RwLock},
};
use tokio::sync::oneshot;
use warp::{path::Peek, Filter, Reply};

//TODO: include in docs that credentials are sent by default
//TODO: ensure conversion to a protocol struct on each recieved call

type Handler = Box<
  dyn Fn(
      Vec<serde_json::Value>,
    ) -> Pin<Box<dyn Future<Output = Result<serde_json::Value, String>> + Send + Sync>>
    + Send
    + Sync,
>;

type SocketChannel = (flume::Sender<Socket>, flume::Receiver<Socket>);

//TODO: check where rwlock/mutex is necessary
#[derive(Clone)]
pub struct ERPCServer {
  /**
    The port the server runs on
  */
  port: u16,
  /**
    Whether the server should accept websocket connections
  */
  enabled_sockets: bool,
  /**
    List of the allowed origins
  */
  allowed_cors_origins: Vec<String>,
  /**
    Request handlers for incoming requests to this server
  */
  handlers: Arc<tokio::sync::RwLock<HashMap<String, Handler>>>,
  /**
    Shutdown signal to exit the webserver gracefully
  */
  shutdown_signal: Arc<RwLock<Option<oneshot::Sender<()>>>>,
  /**
    Channel to broadcast connected sockets
  */
  socket_channel: SocketChannel,
}

impl ERPCServer {
  pub fn new(port: u16, allowed_cors_origins: Vec<String>, enabled_sockets: bool) -> Self {
    ERPCServer {
      handlers: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
      shutdown_signal: Arc::new(RwLock::new(None)),
      port,
      allowed_cors_origins,
      enabled_sockets,
      socket_channel: flume::unbounded(),
    }
  }

  #[allow(dead_code)]
  pub fn register_raw_handler(&mut self, handler: Handler, identifier: &str) {
    self
      .handlers
      //TODO: should this become async and not use blocking:write?
      .blocking_write()
      .insert(identifier.to_owned(), handler);
  }

  #[allow(dead_code)]
  pub fn register_handler<H, P>(&mut self, handler: H, identifier: &str)
  where
    H: super::handler::Handler<P> + 'static,
    P: DeserializeOwned + Send + Sync,
    H::Output: Serialize,
    H::Future: Future<Output = H::Output> + Send + Sync,
  {
    let v: Handler = Box::new(move |v| {
      let handler = handler.clone();
      Box::pin(async move {
        //TODO this could be more elegant?
        let v =
          serde_json::to_value(v).map_err(|err| format!("Could not convert to value: {err}"))?;

        let parameters = match serde_json::from_value::<P>(v) {
          Ok(v) => v,
          Err(err) => {
            return Err(format!("Failed to parse parameters: {}", err));
          }
        };

        let result = handler.call(parameters).await;

        let serialized = match serde_json::to_value(result) {
          Ok(v) => v,
          Err(err) => {
            return Err(format!("Failed to serialize result: {}", err));
          }
        };

        Ok(serialized)
      })
    });

    self
      .handlers
      //TODO: should this become async and not use blocking:write?
      .blocking_write()
      .insert(identifier.to_owned(), v);
  }

  pub fn run(&self) -> Result<impl futures_util::Future<Output = ()> + Send + Sync, String> {
    let handlers = self.handlers.clone();
    let enabled_sockets = self.enabled_sockets;
    let socket_channel = self.socket_channel.clone();

    let socket_channel = warp::any().map(move || socket_channel.clone());
    let handlers = warp::any().map(move || handlers.clone());
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
      .and(handlers)
      .and(warp::path::peek())
      .and(warp::body::json())
      .and(warp::body::content_length_limit(1024 * 64))
      .then(Self::http_handler)
      .with(cors.clone());

    let handlers = self.handlers.clone();
    let request_handlers = warp::any().map(move || handlers.clone());
    let ws = warp::path!("ws" / String)
      .and(enabled_sockets)
      .and(request_handlers)
      .and(socket_channel)
      .and(warp::ws())
      .map(
        |role, enabled_sockets, request_handlers, socket_channel, ws| {
          Self::socket_handler(role, enabled_sockets, request_handlers, socket_channel, ws)
        },
      )
      .with(cors.clone());

    let (sender, reciever) = oneshot::channel::<()>();
    self
      .shutdown_signal
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
    request_handlers: Arc<tokio::sync::RwLock<HashMap<String, Handler>>>,
    path: Peek,
    parameters: Vec<serde_json::Value>,
  ) -> Box<dyn Reply> {
    let lock = request_handlers.read().await;
    let handler = match lock.get(path.as_str()) {
      Some(v) => v,
      None => {
        eprintln!("Could not find a registered handler for {}", path.as_str());
        return Box::new(warp::reply::with_status(
          "Internal server error. Please see server logs",
          StatusCode::NOT_FOUND,
        ));
      }
    };

    let result = match handler(parameters).await {
      Ok(v) => v,
      Err(err) => {
        eprintln!("Error while running handler {}: {err}", path.as_str());
        return Box::new(warp::reply::with_status(
          "Internal server error. Please see server logs",
          StatusCode::INTERNAL_SERVER_ERROR,
        ));
      }
    };

    Box::new(warp::reply::json(&result))
  }

  fn socket_handler(
    role: String,
    enabled_sockets: bool,
    _request_handlers: Arc<tokio::sync::RwLock<HashMap<String, Handler>>>, // in the future we might also handle requests incoming via sockets
    socket_channel: SocketChannel,
    ws: warp::ws::Ws,
  ) -> Box<dyn Reply> {
    if enabled_sockets {
      Box::new(ws.on_upgrade(|socket| async move {
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
              Err(err) => eprintln!("Could not broadcast incoming socket message: {err}"),
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
      }))
    } else {
      Box::new(warp::reply::with_status(
        "Websockets are disabled",
        StatusCode::NOT_IMPLEMENTED,
      ))
    }
  }
}

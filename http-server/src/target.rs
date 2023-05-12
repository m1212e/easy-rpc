use crate::Socket;
use erpc::{
    protocol::{self, socket::SocketMessage},
    target::TargetType,
};
use nanoid::nanoid;
use std::{
    collections::HashMap,
    fmt::Debug,
    sync::{Arc, Mutex},
};
use tokio::sync::oneshot;

lazy_static::lazy_static! {
  static ref REQWEST_CLIENT: reqwest::Client = reqwest::Client::new();
}

//TODO find a better/faster way to store open requests
#[derive(Debug, Clone)]
pub struct Target {
    address: String,
    target_type: TargetType,
    socket: Arc<Mutex<Option<Socket>>>,
    requests: Arc<Mutex<HashMap<String, oneshot::Sender<protocol::socket::Response>>>>,
}

impl Target {
    pub fn new(mut address: String, target_type: TargetType) -> Self {
        if address.ends_with('/') {
            address.pop();
        }

        Target {
            address,
            target_type,
            socket: Arc::new(Mutex::new(None)),
            requests: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn call(&self, request: protocol::Request) -> protocol::Response {
        match self.target_type {
            TargetType::HTTPServer => {
                let r = REQWEST_CLIENT
                    .post(format!("{}/handlers/{}", self.address, request.identifier))
                    .body(
                        serde_json::to_vec(&request.parameters)
                            .expect("Vec of json::Value should be ok"),
                    );

                let response = match r.send().await {
                    Ok(v) => v,
                    Err(err) => {
                        return protocol::Response {
                            body: Err(err.into()),
                        }
                    }
                };

                let bytes = match response.bytes().await {
                    Ok(v) => v,
                    Err(err) => {
                        return protocol::Response {
                            body: Err(err.into()),
                        }
                    }
                };

                protocol::Response {
                    body: serde_json::from_slice(&bytes).map_err(|err| err.into()),
                }
            }
            TargetType::Browser => {
                let socket = {
                    let socket = self
                        .socket
                        .lock()
                        .map_err(|err| format!("Could not lock socket mutex: {err}"))?;

                    match &*socket {
                        Some(v) => v.clone(),
                        None => return Err("Socket not set for this target".to_string()),
                    }
                };

                let id = nanoid!();
                let (sender, reciever) = oneshot::channel::<protocol::socket::Response>();
                {
                    // scope to drop the requests lock
                    let mut requests = self
                        .requests
                        .lock()
                        .map_err(|err| format!("Could not access sockets: {err}"))?;

                    requests.insert(id.clone(), sender);
                }

                socket
                    .sender
                    .send(SocketMessage::Request(protocol::socket::Request {
                        id,
                        request,
                    }))
                    .unwrap();

                let response = reciever
                    .await
                    .map_err(|err| format!("RecvError in socket response channel: {err}"))?;

                let response = response.body;

                //TODO this unwrap
                serde_json::from_value(response.body.unwrap())
                    .map_err(|err| format!("Could not parse socket response: {err}"))?
            }
        }
    }

    pub async fn set_socket(&mut self, socket: Socket) {
        match self.socket.lock() {
            Ok(mut v) => {
                *v = Some(socket.clone());
            }
            Err(err) => {
                eprintln!("Socket lock error: {err}");
                return;
            }
        }

        loop {
            let msg = match socket.reciever.recv_async().await {
                Ok(v) => v,
                Err(err) => {
                    eprintln!("Socket stream error: {err}");
                    return;
                }
            };

            match msg {
                SocketMessage::Request(_) => {
                    eprintln!("Requests via websocket not supported yet!");
                    break;
                }
                SocketMessage::Response(res) => {
                    let mut requests = match self.requests.lock() {
                        Ok(v) => v,
                        Err(err) => {
                            eprintln!("Could not access requests (1): {err}");
                            break;
                        }
                    };

                    let return_channel = match requests.remove(&res.id) {
                        Some(v) => v,
                        None => {
                            eprintln!("Could not find open request for id {}", res.id);
                            break;
                        }
                    };

                    match return_channel.send(res) {
                        Ok(_) => {}
                        Err(ret_res) => eprintln!("Could not send response for {}", ret_res.id),
                    };
                }
            };
        }
    }
}

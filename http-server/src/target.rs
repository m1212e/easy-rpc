use crate::Socket;
use erpc::{
    protocol::{self, error::Error},
    target::TargetType,
};
use log::error;
use nanoid::nanoid;
use parking_lot::Mutex;
use std::{collections::HashMap, fmt::Debug, sync::Arc};
use tokio::sync::oneshot;

lazy_static::lazy_static! {
  static ref REQWEST_CLIENT: reqwest::Client = reqwest::Client::new();
}

//TODO find a better/faster way to store open requests
#[derive(Debug, Clone)]
pub struct Target {
    address: String,
    target_type: TargetType,
    socket: Option<Socket>,
    //TODO check if this is optimal
    open_socket_requests: Arc<Mutex<HashMap<String, oneshot::Sender<protocol::socket::Response>>>>,
}

impl Target {
    pub fn new(mut address: String, target_type: TargetType) -> Self {
        if address.ends_with('/') {
            address.pop();
        }

        Target {
            address,
            target_type,
            socket: None,
            open_socket_requests: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn call(&self, request: protocol::Request) -> protocol::Response {
        match self.target_type {
            TargetType::HttpServer => {
                let r = REQWEST_CLIENT
                    .post(format!("{}/handlers/{}", self.address, request.identifier))
                    .body(
                        serde_json::to_vec(&request.parameters)
                            .expect("Vec of json::Value should be ok"),
                    );

                let response = match r.send().await {
                    Ok(v) => v,
                    Err(err) => return Error::from(err).into(),
                };

                let bytes = match response.bytes().await {
                    Ok(v) => v,
                    Err(err) => return Error::from(err).into(),
                };

                protocol::Response {
                    body: serde_json::from_slice(&bytes).map_err(|err| Error::from(err).into()),
                }
            }
            TargetType::Browser => {
                let request_over_socket_channel = match &self.socket {
                    Some(v) => v.requests.clone(),
                    None => return Error::from("Socket not set for this target").into(),
                };

                let id = nanoid!();
                let (sender, reciever) = oneshot::channel::<protocol::socket::Response>();
                {
                    // scope to drop the requests lock
                    let mut requests = self.open_socket_requests.lock();

                    requests.insert(id.clone(), sender);
                }

                match request_over_socket_channel.send(protocol::socket::Request {
                    id: id.clone(),
                    request,
                }) {
                    Ok(_) => {}
                    Err(err) => {
                        let mut requests = self.open_socket_requests.lock();

                        requests.remove(&id);

                        return Error::from(format!("Could not send request on socket: {err}"))
                            .into();
                    }
                }

                let response = match reciever.await {
                    Ok(v) => v,
                    Err(err) => {
                        return Error::from(format!("Could not await response channel: {err}"))
                            .into()
                    }
                };

                response.response
            }
        }
    }

    pub async fn set_socket(&mut self, socket: Socket) {
        while let Ok(response) = socket.responses.recv_async().await {
            let mut requests = self.open_socket_requests.lock();

            let return_channel = match requests.remove(&response.id) {
                Some(v) => v,
                None => {
                    error!("Could not find open request for id {}", response.id);
                    continue;
                }
            };

            match return_channel.send(response) {
                Ok(_) => {}
                Err(ret_res) => error!("Could not send response for {}", ret_res.id),
            };
        }
    }
}

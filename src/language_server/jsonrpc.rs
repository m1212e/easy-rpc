use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    collections::HashMap,
    io::Read,
    num::ParseIntError,
    str::Utf8Error,
    sync::{Arc, RwLock},
    time::Duration,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    sync::{
        mpsc,
        oneshot::{self, error::RecvError},
    },
    time::{error::Elapsed, timeout},
};

use crate::unwrap_oneshot;
use nanoid::nanoid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    jsonrpc: String,
    method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    id: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<Error>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    pub code: i32,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error {
            code: -32700,
            data: None,
            message: format!("Could not serialize JSON message: {}", err),
        }
    }
}

impl From<RecvError> for Error {
    fn from(err: RecvError) -> Self {
        Error {
            code: -32000,
            data: None,
            message: format!("Recv error in oneshot: {}", err.to_string()),
        }
    }
}

impl From<Elapsed> for Error {
    fn from(_: Elapsed) -> Self {
        Error {
            code: -32001,
            data: None,
            message: format!("Timed out"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error {
            code: -32002,
            data: None,
            message: format!("IO Error {}", err.to_string()),
        }
    }
}

impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Self {
        Error {
            code: -32003,
            data: None,
            message: format!("String conversion error {}", err.to_string()),
        }
    }
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Error {
            code: -32004,
            data: None,
            message: format!("Int conversion error {}", err.to_string()),
        }
    }
}

#[derive(Clone)]
pub struct JSONRPCServer {
    pending_responses: Arc<RwLock<HashMap<String, oneshot::Sender<Response>>>>,
}

impl JSONRPCServer {
    pub fn send_request(
        &self,
        method: &str,
        params: Option<serde_json::Value>,
        is_notification: bool,
    ) -> oneshot::Receiver<Result<Response, Error>> {
        let message = Request {
            jsonrpc: "2.0".to_string(),
            id: match is_notification {
                true => None,
                false => Some(serde_json::Value::from(nanoid!())),
            },
            method: method.to_string(),
            params,
        };

        let (ret_send, ret_recieve) = oneshot::channel::<Result<Response, Error>>();

        let serialized = unwrap_oneshot!(serde_json::to_string(&message), ret_send, ret_recieve);
        let mut payload = format!(
            "Content-Length: {len}\r\n\r\n",
            len = serialized.as_bytes().len()
        );

        payload.push_str(&serialized);
        payload.push('\n');

        let pending_responses = self.pending_responses.clone();
        tokio::spawn(async move {
            tokio::io::stdout()
                .write_all(payload.as_bytes())
                .await
                .unwrap();

            if is_notification {
                ret_send
                    .send(Ok(Response {
                        error: None,
                        id: json!(0u32),
                        result: None,
                    }))
                    .unwrap();
                return;
            }

            let (callback_send, callback_recieve) = oneshot::channel::<Response>();
            {
                let mut w = pending_responses.write().unwrap();
                (*w).insert(message.id.as_ref().unwrap().to_string(), callback_send);
            }

            match timeout(Duration::from_secs(10), callback_recieve).await {
                Ok(res) => match res {
                    Ok(val) => ret_send.send(Ok(val)),
                    Err(err) => ret_send.send(Err(err.into())),
                },
                Err(err) => ret_send.send(Err(err.into())),
            }
            .unwrap();

            {
                let mut w = pending_responses.write().unwrap();
                (*w).remove(&message.id.unwrap().to_string());
            }
        });

        ret_recieve
    }

    pub fn new() -> JSONRPCServer {
        JSONRPCServer {
            pending_responses: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn run(
        &mut self,
        handlers: Arc<
            RwLock<
                HashMap<
                    String,
                    Box<
                        dyn Fn(Option<serde_json::Value>) -> Result<serde_json::Value, Error>
                            + Send
                            + Sync,
                    >,
                >,
            >,
        >,
    ) -> Result<(), Error> {
        let (sender, reciever) = mpsc::channel::<Request>(10);
        let err = tokio::select! {
            err = incoming(sender) => {
                err
            }
            err = responder(reciever, handlers) => {
                err
            }
        };

        err
    }
}

async fn incoming(incoming_request_sender: mpsc::Sender<Request>) -> Result<(), Error> {
    loop {
        let mut buffer = [0u8; 15];
        tokio::io::stdin().read_exact(&mut buffer).await?;

        if std::str::from_utf8(&buffer)?.to_string().to_lowercase() != "content-length:" {
            return Err(Error {
                code: -32700,
                data: None,
                message: "Could not find message size".to_string(),
            });
        }

        //TODO: Handle Content-Type header part accordingly
        let mut read = String::new();
        loop {
            let char = tokio::io::stdin().read_u8().await? as char;
            if char == '{' {
                break;
            }
            read.push(char);
        }

        read.retain(|c| !c.is_whitespace());

        let length = read.parse::<usize>()? - 1; // -1 for the { which is already consumed

        let mut buffer: Vec<u8> = Vec::new();
        buffer.resize(length, 0);
        if tokio::io::stdin().read(&mut buffer[..]).await? != length {
            return Err(Error {
                code: -32700,
                data: None,
                message: format!("Could not read message of length {length}"),
            });
        };

        let raw_message = format!("{}{}", "{", std::str::from_utf8(&buffer)?.to_string());

        match serde_json::from_str::<Request>(&raw_message) {
            Ok(request) => {
                incoming_request_sender.send(request).await.unwrap();
            }
            Err(err) => {
                return Err(Error::from(err));
            }
        }
    }
}

async fn responder(
    mut incoming_request_reciever: mpsc::Receiver<Request>,
    handlers: Arc<
        RwLock<
            HashMap<
                String,
                Box<
                    dyn Fn(Option<serde_json::Value>) -> Result<serde_json::Value, Error>
                        + Send
                        + Sync,
                >,
            >,
        >,
    >,
) -> Result<(), Error> {
    loop {
        let request = match incoming_request_reciever.recv().await {
            Some(val) => val,
            None => break,
        };

        let result = {
            match handlers.read() {
                Ok(handlers) => match handlers.get(&request.method) {
                    Some(handler) => handler(request.params),
                    None => Err(Error {
                        code: -32601,
                        message: "The requested method could not be found".to_string(),
                        data: Some(serde_json::to_value(request.method).unwrap()),
                    }),
                },
                Err(_) => Err(Error {
                    code: 0,
                    message: "Handlers RW Lock is poisoned".to_string(),
                    data: None,
                }),
            }
        };

        // a notification doesnÄt need a response
        if request.id.is_none() {
            continue;
        }

        let response = match result {
            Ok(val) => serde_json::to_string(&Response {
                id: request.id.unwrap(), // this is guaranteed
                result: Some(val),
                error: None,
            })?,
            Err(error) => serde_json::to_string(&Response {
                id: request.id.unwrap(), // this is guaranteed
                result: None,
                error: Some(error),
            })?,
        };

        let mut payload = format!("Content-Length: {}\r\n\r\n", response.as_bytes().len());
        payload.push_str(&response);
        payload.push('\n');

        tokio::io::stdout().write_all(payload.as_bytes()).await?;
    }

    Ok(())
}

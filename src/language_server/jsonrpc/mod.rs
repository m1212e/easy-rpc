use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    collections::HashMap,
    future::Future,
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

#[derive(Serialize, Deserialize)]
pub struct Request {
    jsonrpc: String,
    method: String,
    params: serde_json::Value,
    id: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    id: serde_json::Value,
    result: Option<serde_json::Value>,
    error: Option<Error>,
}

#[derive(Serialize, Deserialize)]
pub struct Error {
    code: i32,
    message: String,
    data: Option<serde_json::Value>,
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error {
            code: -32700,
            data: None,
            message: "Could not serialize JSON message".to_string(),
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
    fn from(err: Elapsed) -> Self {
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

pub struct JSONRPCServer<F, Fut>
where
    F: Fn(String, serde_json::Value) -> Fut,
    Fut: Future<Output = (Option<serde_json::Value>, Option<Error>)>,
{
    input: tokio::io::Stdin,
    output: tokio::io::Stdout,
    pending_responses: Arc<RwLock<HashMap<String, oneshot::Sender<Response>>>>,
    incoming_requests: (mpsc::Sender<Request>, mpsc::Receiver<Request>),
    id_counter: u128,
    on_request_callback: F,
}

impl<F, Fut> JSONRPCServer<F, Fut>
where
    F: Fn(String, serde_json::Value) -> Fut,
    Fut: Future<Output = (Option<serde_json::Value>, Option<Error>)>,
{
    pub async fn send_request(
        &mut self,
        method: String,
        params: serde_json::Value,
        is_notification: bool,
    ) -> Result<Response, Error> {
        let message = Request {
            jsonrpc: "2.0".to_string(),
            id: match is_notification {
                true => None,
                false => Some(serde_json::Value::from(self.id_counter.to_string())),
            },
            method,
            params,
        };
        self.id_counter += 1;

        let serialized = serde_json::to_string(&message)?;
        let prefix = format!(
            "Content-Length: {len}\r\n\r\n",
            len = serialized.as_bytes().len()
        );

        self.output
            .write_all(&format!("{prefix}{serialized}").as_bytes())
            .await?;

        if is_notification {
            return Ok(Response {
                error: None,
                id: json!(0u32),
                result: None,
            });
        }

        let (sender, reciever) = oneshot::channel::<Response>();
        {
            let mut w = self.pending_responses.write().unwrap();
            (*w).insert(message.id.as_ref().unwrap().to_string(), sender);
        }

        let ret = timeout(Duration::from_secs(10), reciever).await??;

        {
            let mut w = self.pending_responses.write().unwrap();
            (*w).remove(&message.id.unwrap().to_string());
        }

        Ok(ret)
    }

    pub fn new(
        input: tokio::io::Stdin,
        output: tokio::io::Stdout,
        on_request: F,
    ) -> JSONRPCServer<F, Fut> {
        JSONRPCServer {
            input,
            output,
            pending_responses: Arc::new(RwLock::new(HashMap::new())),
            incoming_requests: mpsc::channel(10),
            id_counter: 0,
            on_request_callback: on_request,
        }
    }

    pub async fn run(&mut self) -> Result<(), Error> {
        tokio::select! {
            err = incoming(&mut self.input, &mut self.incoming_requests.0) => {
                err
            }
            err = responder(&mut self.output, &mut self.incoming_requests.1, &self.on_request_callback) => {
                err
            }
        }
    }
}

async fn incoming(
    input: &mut tokio::io::Stdin,
    incoming_request_sender: &mut mpsc::Sender<Request>,
) -> Result<(), Error> {
    loop {
        let mut buffer = [0; 15];
        if input.read(&mut buffer[..]).await? != 15 {
            break; // reader ended
        };

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
            let char = input.read_u8().await? as char;
            if char == '{' {
                break;
            }
            read.push(char);
        }

        read.retain(|c| !c.is_whitespace());

        let length = read.parse::<usize>()?;

        let mut buffer: Vec<u8> = Vec::new();
        if input.read(&mut buffer[..]).await? != length - 1 {
            return Err(Error {
                code: -32700,
                data: None,
                message: format!("Could not read message of length {length}"),
            });
        };

        let raw_message = format!("{}{}", "{", std::str::from_utf8(&buffer)?.to_string());

        match serde_json::from_str::<Request>(&raw_message) {
            Ok(request) => {
                incoming_request_sender.send(request).await;
            }
            Err(_) => {}
        }
    }
    Ok(())
}

async fn responder<F, Fut>(
    output: &mut tokio::io::Stdout,
    incoming_request_reciever: &mut mpsc::Receiver<Request>,
    onRequest: &F,
) -> Result<(), Error>
where
    F: Fn(String, serde_json::Value) -> Fut,
    Fut: Future<Output = (Option<serde_json::Value>, Option<Error>)>,
{
    loop {
        let request = match incoming_request_reciever.recv().await {
            Some(val) => val,
            None => break,
        };

        let result = onRequest(request.method, request.params).await;

        let response = serde_json::to_string(&Response {
            id: request.id.unwrap(), // this is guaranteed
            result: result.0,
            error: result.1,
        })?;

        let prefix = format!("Content-Length: {}\r\n\r\n", response.as_bytes().len());

        output.write(format!("{}{}", prefix, response).as_bytes()).await?;
    }

    Ok(())
}

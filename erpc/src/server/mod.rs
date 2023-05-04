pub mod handler;

use crate::protocol::{self, Request, Response};
use futures_util::Future;
use serde::{de::DeserializeOwned, Serialize};
use std::{collections::HashMap, pin::Pin, sync::Arc};
use tokio::sync::RwLock;

pub type InternalHandler = Box<
    dyn Fn(Request) -> Pin<Box<dyn Future<Output = Result<Response, String>> + Send + Sync>>
        + Send
        + Sync,
>;

#[derive(Clone)]
pub struct Server {
    handlers: Arc<RwLock<HashMap<String, InternalHandler>>>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    #[allow(dead_code)]
    pub fn register_raw_handler(&mut self, handler: InternalHandler, identifier: &str) {
        self.handlers
            //TODO: should this become async and not use blocking:write?
            .blocking_write()
            .insert(identifier.to_string(), handler);
    }

    #[allow(dead_code)]
    pub fn register_handler<H, P>(&mut self, handler: H, identifier: &str)
    where
        H: crate::server::handler::Handler<P> + 'static,
        P: DeserializeOwned + Send + Sync,
        H::Output: Serialize,
        H::Future: Future<Output = H::Output> + Send + Sync,
    {
        let v: InternalHandler = Box::new(move |request| {
            let handler = handler.clone();
            Box::pin(async move {
                let parameters = serde_json::to_value(request.parameters)
                    .map_err(|err| format!("Could not convert to value: {err}"))?;

                let parameters = serde_json::from_value::<P>(parameters)
                    .map_err(|err| format!("Could not parse parameters: {err}"))?;

                let result = handler.call(parameters).await;

                let serialized = serde_json::to_value(&result)
                    .map_err(|err| format!("Could not serialize response: {err}"))?;

                Ok(Response { body: serialized })
            })
        });

        self.handlers
            //TODO: should this become async and not use blocking:write?
            .blocking_write()
            .insert(identifier.to_string(), v);
    }

    pub async fn process_request(
        &self,
        request: protocol::Request,
    ) -> Result<protocol::Response, String> {
        let handlers = self.handlers.read().await;
        let handler = handlers.get(&request.identifier).ok_or(format!(
            "Could not find handler for request identifier {}",
            request.identifier
        ))?;

        handler(request).await
    }
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}

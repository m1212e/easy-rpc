#![deny(clippy::all)]

//TODO: remove unwraps
//TODO: refactoring

use std::convert::Infallible;

use erpc::protocol;
use http_server::Socket;
use napi::{
    bindgen_prelude::{FromNapiValue, Promise},
    Env, JsFunction, JsUnknown, NapiRaw,
};
use tokio::sync::oneshot;

use crate::INITIALIZED;

#[napi(object)]
pub struct ServerOptions {
    pub port: u16,
    pub allowed_cors_origins: Vec<String>,
}

#[napi(js_name = "ERPCServer")]
pub struct ERPCServer {
    server: http_server::Server,
}

#[napi]
impl ERPCServer {
    #[napi(constructor)]
    pub fn new(
        options: ServerOptions,
        _server_type: String, // exists for consistency reasons but isn't actually needed
        enable_sockets: bool,
        _role: String, // might become handy in the future
    ) -> Self {
        if *INITIALIZED {}

        ERPCServer {
            server: http_server::Server::new(
                options.port,
                options.allowed_cors_origins,
                enable_sockets,
            ),
        }
    }

    #[napi(skip_typescript, js_name = "registerERPCHandler")]
    pub fn register_erpc_handler(
        &self,
        env: Env,
        func: JsFunction,
        identifier: String,
    ) -> Result<(), napi::Error> {
        let tsf = crate::threadsafe_function::ThreadsafeFunction::create(
            env.raw(),
            unsafe { func.raw() },
            0,
            |ctx: crate::threadsafe_function::ThreadSafeCallContext<(
                Vec<serde_json::Value>,
                oneshot::Sender<serde_json::Value>,
            )>| {
                let args = ctx
                    .value
                    .0
                    .iter()
                    .map(|v| ctx.env.to_js_value(v))
                    .collect::<Result<Vec<JsUnknown>, napi::Error>>()?;

                let response = ctx.callback.call(None, args.as_slice())?;
                let response_channel = ctx.value.1;

                if !response.is_promise()? {
                    let response: serde_json::Value = ctx.env.from_js_value(response)?;
                    ctx.env
                        .execute_tokio_future(
                            async move {
                                match response_channel.send(serde_json::to_value(&response)?) {
                                    Ok(_) => {}
                                    Err(err) => {
                                        return Err(napi::Error::from_reason(format!(
                                            "Could not send response: {err}"
                                        )))
                                    }
                                };
                                Ok(())
                            },
                            |_, _| Ok(()),
                        )
                        .unwrap();
                } else {
                    unsafe {
                        let prm: Promise<serde_json::Value> =
                            Promise::from_napi_value(ctx.env.raw(), response.raw())?;
                        ctx.env.execute_tokio_future(
                            async move {
                                let result = prm.await?;
                                match response_channel.send(serde_json::to_value(result)?) {
                                    Ok(_) => {}
                                    Err(err) => {
                                        return Err(napi::Error::from_reason(format!(
                                            "Could not send response: {err}"
                                        )))
                                    }
                                };
                                Ok(())
                            },
                            |_, _| Ok(()),
                        )?;
                    }
                };

                Ok(())
            },
        )?;

        self.server.register_raw_handler(
            Box::new(move |input| {
                let (sender, reciever) = oneshot::channel::<serde_json::Value>();
                let r = tsf.call(
                    (input.parameters, sender),
                    crate::threadsafe_function::ThreadsafeFunctionCallMode::Blocking,
                );

                Box::pin(async move {
                    match r {
                        napi::Status::Ok => {}
                        _ => {
                            return erpc::protocol::Response {
                                body: Err(protocol::error::Error::from(format!(
                                    "Threadsafe function status not ok: {r}"
                                ))
                                .into()),
                            }
                        }
                    };

                    let v = match reciever.await {
                        Ok(v) => v,
                        Err(err) => {
                            return erpc::protocol::Response {
                                body: Err(protocol::error::Error::from(format!(
                                    "Could not receive response: {err}"
                                ))
                                .into()),
                            }
                        }
                    };

                    erpc::protocol::Response { body: Ok(v) }
                })
            }),
            identifier,
        );
        Ok(())
    }

    #[napi(skip_typescript)]
    pub fn on_socket_connection(&mut self, env: Env, func: JsFunction) -> Result<(), napi::Error> {
        let tsf = crate::threadsafe_function::ThreadsafeFunction::create(
            env.raw(),
            unsafe { func.raw() },
            0,
            |ctx: crate::threadsafe_function::ThreadSafeCallContext<Socket>| {
                let role = ctx.env.create_string_from_std(ctx.value.role.clone())?;
                let mut socket = ctx.env.create_object()?;
                ctx.env.wrap(&mut socket, ctx.value)?;

                ctx.callback
                    .call(None, &[role.into_unknown(), socket.into_unknown()])?;
                Ok(())
            },
        )?;

        let socket_notifier_channel = self.server.socket_broadcaster().clone();
        env.execute_tokio_future(
            async move {
                loop {
                    let socket = match socket_notifier_channel.recv_async().await {
                        Ok(v) => v,
                        Err(err) => {
                            return Err(napi::Error::from_reason(format!(
                                "Error while recieving from socket notifier channel: {err}"
                            )))
                        }
                    };

                    let r = tsf.call(
                        (socket).to_owned(),
                        crate::threadsafe_function::ThreadsafeFunctionCallMode::Blocking,
                    );

                    match r {
                        napi::Status::Ok => {}
                        _ => {
                            return Err(napi::Error::from_reason(format!(
                                "Threadsafe function status not ok: {r}"
                            )))
                        }
                    }
                }
            },
            |_, _: Infallible| Ok(()),
        )
        .unwrap();

        Ok(())
    }

    /**
      Starts the server as configured
    */
    #[napi]
    pub async fn run(&self) -> Result<(), napi::Error> {
        self.server
            .run()
            .await
            .map_err(|err| napi::Error::from_reason(format!("Could not start server: {err}")))
    }

    /**
     * Stops the server
     */
    #[napi]
    pub fn stop(&self) -> Result<(), napi::Error> {
        match self.server.stop() {
            Ok(_) => Ok(()),
            Err(err) => Err(napi::Error::from_reason(format!(
                "Could not stop server: {err}"
            ))),
        }
    }
}

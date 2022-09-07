use serde::{Serialize};

use super::{LanguageServer};

#[derive(Serialize)]
pub struct ServerCapabilities {}

impl LanguageServer {
    pub fn on_initialized<F: Fn() + Send + Sync + 'static>(&mut self, handler: F) {
        self.register_handler("initialized".to_string(), move |_| {
            handler();
            Ok(serde_json::to_value("{}").unwrap())
        });
    }
}

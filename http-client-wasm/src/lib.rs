use std::sync::Arc;

use target::Target;
use tokio::sync::RwLock;

mod server;
mod target;
mod tests;

lazy_static::lazy_static! {
    static ref CREATED_TARGETS: StorageChannel<Target> = StorageChannel::new();
}

//TODO make this its own crate
struct StorageChannel<T: Clone> {
    stored_values: Arc<RwLock<Vec<T>>>,
    senders: Arc<RwLock<Vec<flume::Sender<T>>>>,
}

impl<T: Clone> StorageChannel<T> {
    fn new() -> Self {
        Self {
            stored_values: Arc::new(RwLock::new(Vec::new())),
            senders: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn send(&self, value: T) {
        for sender in self.senders.read().await.iter() {
            //TODO
            sender.send_async(value.to_owned()).await;
        }

        self.stored_values.write().await.push(value);
    }

    pub async fn reciever(&self) -> Result<flume::Receiver<T>, String> {
        let (rx, tx) = flume::unbounded();

        for stored_value in self.stored_values.read().await.iter() {
            rx.send_async(stored_value.to_owned()).await;
        }

        self.senders.write().await.push(rx);

        Ok(tx)
    }
}

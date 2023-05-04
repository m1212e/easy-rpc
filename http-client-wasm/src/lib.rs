use std::sync::{Arc, RwLock};

use target::Target;

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

    pub async fn send(&self, value: T) -> Result<(), String> {
        let mut r = self
            .stored_values
            .write()
            .map_err(|err| format!("The storage channel lock is poisoned"))?;

        let senders = self
            .senders
            .read()
            .map_err(|err| format!("The senders lock is poisoned"))?;

        for sender in senders {
            sender.send_async(value.to_owned()).await;
        }

        r.push(value);
        Ok(())
    }

    pub async fn reciever(&self) -> Result<flume::Receiver<T>, String> {
        let (rx, tx) = flume::unbounded();

        let mut senders = self
            .senders
            .write()
            .map_err(|err| format!("The senders lock is poisoned"))?;

        let stored_values = self
            .stored_values
            .read()
            .map_err(|err| format!("The storage channel lock is poisoned"))?;

        for stored_value in stored_values {
            rx.send_async(stored_value.to_owned()).await
        }

        senders.push(rx);

        Ok(tx)
    }
}

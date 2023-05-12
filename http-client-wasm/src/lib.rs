use std::fmt::Debug;
use std::sync::Arc;

mod server;
mod target;
mod tests;

use parking_lot::RwLock;
pub use server::Server;
pub use target::Target;

lazy_static::lazy_static! {
    static ref CREATED_TARGETS: StorageChannel<Target> = StorageChannel::new();
}

//TODO make this its own crate
struct StorageChannel<T: Clone + Debug> {
    stored_values: Arc<RwLock<Vec<T>>>,
    senders: Arc<RwLock<Vec<flume::Sender<T>>>>,
}

impl<T: Clone + Debug> StorageChannel<T> {
    fn new() -> Self {
        Self {
            stored_values: Arc::new(RwLock::new(Vec::new())),
            senders: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn send(&self, value: T) -> Result<(), String> {
        let lock = self.senders.read();

        for sender in lock.iter() {
            sender.send(value.to_owned()).unwrap();
        }

        self.stored_values.write().push(value);

        Ok(())
    }

    pub fn reciever(&self) -> Result<flume::Receiver<T>, String> {
        let (rx, tx) = flume::unbounded();

        for stored_value in self.stored_values.read().iter() {
            //TODO check if blocking send here is ok
            rx.send(stored_value.to_owned()).unwrap();
        }

        self.senders.write().push(rx);

        Ok(tx)
    }
}

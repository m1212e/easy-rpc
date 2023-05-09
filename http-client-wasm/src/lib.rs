use std::sync::Arc;
use std::sync::RwLock;

mod server;
mod target;
mod tests;

pub use server::Server;
pub use target::Target;

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

    pub fn send(&self, value: T) -> Result<(), String> {
        let lock = self
            .senders
            .read()
            .map_err(|err| format!("Lock is poisoned: {:#?}", err))?;

        for sender in lock.iter() {
            sender.send(value.to_owned());
        }

        self.stored_values
            .write()
            .map_err(|err| format!("Lock is poisoned: {:#?}", err))?
            .push(value);

        Ok(())
    }

    pub fn reciever(&self) -> Result<flume::Receiver<T>, String> {
        let (rx, tx) = flume::unbounded();

        for stored_value in self
            .stored_values
            .read()
            .map_err(|err| format!("Lock is poisoned: {:#?}", err))?
            .iter()
        {
            //TODO check if blocking send here is ok
            rx.send(stored_value.to_owned());
        }

        self.senders
            .write()
            .map_err(|err| format!("Lock is poisoned: {:#?}", err))?
            .push(rx);

        Ok(tx)
    }
}

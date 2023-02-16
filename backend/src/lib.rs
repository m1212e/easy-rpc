mod protocol;
mod tests;
mod handler;
pub mod server;
pub mod target;

//TODO write tests && integrate in CI

#[derive(Clone, Debug)]
pub struct Socket {
  pub sender: flume::Sender<protocol::socket::SocketMessage>,
  pub reciever: flume::Receiver<protocol::socket::SocketMessage>,
  pub role: String,
}

pub use server::ERPCServer;
pub use target::ERPCTarget;
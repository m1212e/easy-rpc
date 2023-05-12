mod target;
mod server;
mod handler;
mod tests;

//TODO write tests && integrate them in CI

#[derive(Clone, Debug)]
pub struct Socket {
    pub sender: flume::Sender<erpc::protocol::socket::SocketMessage>,
    pub reciever: flume::Receiver<erpc::protocol::socket::SocketMessage>,
    pub role: String,
}

pub use target::Target;
pub use server::Server;

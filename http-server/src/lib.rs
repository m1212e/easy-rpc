mod server;
mod target;
mod handler;
mod tests;

//TODO write tests && integrate in CI

#[derive(Clone, Debug)]
pub struct Socket {
    pub sender: flume::Sender<erpc::protocol::socket::SocketMessage>,
    pub reciever: flume::Receiver<erpc::protocol::socket::SocketMessage>,
    pub role: String,
}

pub use server::Server;
pub use target::Target;

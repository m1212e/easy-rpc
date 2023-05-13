mod handler;
mod server;
mod target;
mod tests;

//TODO write tests && integrate them in CI

#[derive(Clone, Debug)]
pub struct Socket {
    pub requests: flume::Sender<erpc::protocol::socket::Request>,
    pub responses: flume::Receiver<erpc::protocol::socket::Response>,
    pub role: String,
}

pub use server::Server;
pub use target::Target;

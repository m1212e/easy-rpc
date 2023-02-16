#[cfg(test)]
mod tests {
  use std::time::Duration;

  use tokio::time::sleep;

  use crate::server::ERPCServer;

  #[test]
  fn creation() {
    ERPCServer::new(5678, vec!["http://localhost".to_string()], true);
  }

  #[tokio::test]
  async fn run_stop() {
    let server = ERPCServer::new(5678, vec!["http://localhost".to_string()], true);
    let s2 = server.clone();
    tokio::spawn(async move {
      sleep(Duration::from_millis(3000)).await;
      s2.stop().unwrap();
    });
    server.run().unwrap().await;
  }
}

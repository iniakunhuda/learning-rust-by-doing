use crate::common::ChatError;
use crate::server::ChatServer;

pub mod common;
pub mod server;
pub mod client;

#[tokio::main]
async fn main() -> Result<(), ChatError> {
    let addr = "127.0.0.1:8080";
    let server = ChatServer::new(addr).await?;
    println!("Chat server running on {}", addr);
    server.run().await?;
    Ok(())
}
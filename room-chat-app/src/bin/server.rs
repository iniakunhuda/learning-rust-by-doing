use room_chat_app::server::ChatServer;
use room_chat_app::common::ChatError;

#[tokio::main]
async fn main() -> Result<(), ChatError> {
    let addr = "127.0.0.1:8080";
    
    println!("Starting chat server...");
    let server = ChatServer::new(addr).await?;
    println!("Server listening on {}", addr);
    
    server.run().await?;
    Ok(())
}
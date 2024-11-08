use room_chat_app::client::ChatClient;
use room_chat_app::common::ChatError;
use std::env;

#[tokio::main]
async fn main() -> Result<(), ChatError> {
    let username = env::args()
        .nth(1)
        .expect("Please provide a username as argument");

    let addr = "127.0.0.1:8080";
    let mut client = ChatClient::new(addr, username).await?;
    
    println!("Connected to server at {}", addr);
    println!("Commands:");
    println!("  /join <room>  - Join a chat room");
    println!("  /leave <room> - Leave a chat room");
    println!("  /list        - List available rooms");
    println!("  /users <room> - List users in a room");
    println!("  /quit        - Quit the application");
    
    client.run().await?;
    Ok(())
}
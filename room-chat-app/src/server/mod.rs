use crate::common::ChatError;
use tokio::net::TcpListener;
use std::sync::Arc;
use tokio::sync::Mutex;

pub mod handler;
pub mod room_manager;
pub mod client_manager;

pub struct ChatServer {
    listener: TcpListener,
    room_manager: Arc<Mutex<room_manager::RoomManager>>,
    client_manager: Arc<Mutex<client_manager::ClientManager>>,
}

impl ChatServer {
    pub async fn new(addr: &str) -> Result<Self, ChatError> {
        let listener = TcpListener::bind(addr).await?;

        let room_manager = Arc::new(Mutex::new(room_manager::RoomManager::new()));
        let client_manager = Arc::new(Mutex::new(client_manager::ClientManager::new()));

        // Create default lobby room
        {
            let mut rm = room_manager.lock().await;
            rm.create_room("lobby".to_string()).await?;
        }

        Ok(ChatServer {
            listener,
            room_manager,
            client_manager,
        })
    }

    pub async fn run(self) -> Result<(), ChatError> {
        println!("Server is running and ready to accept connections");

        loop {
            let (socket, addr) = self.listener.accept().await?;
            println!("New connection from: {}", addr);

            let rm = Arc::clone(&self.room_manager);
            let cm = Arc::clone(&self.client_manager);

            tokio::spawn(async move {
                let username = format!("user_{}", addr.port()); // Temporary username based on port
                match handler::ClientHandler::new(username, rm, cm).await {
                    Ok(handler) => {
                        if let Err(e) = handler.handle(socket).await {
                            eprintln!("Error handling client {}: {}", addr, e);
                        }
                    }
                    Err(e) => eprintln!("Error creating handler for {}: {}", addr, e),
                }
            });
        }
    }
}
use crate::common::{ChatError, ChatErrorKind, Message};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct ClientHandler {
    username: String,
    room_manager: Arc<Mutex<super::room_manager::RoomManager>>,
    client_manager: Arc<Mutex<super::client_manager::ClientManager>>,
    tx: mpsc::Sender<Message>,
    rx: mpsc::Receiver<Message>,
}

impl ClientHandler {
    pub async fn new(
        username: String,
        room_manager: Arc<Mutex<super::room_manager::RoomManager>>,
        client_manager: Arc<Mutex<super::client_manager::ClientManager>>,
    ) -> Result<Self, ChatError> {
        let (tx, rx) = mpsc::channel(100);
        
        // Register client
        client_manager.lock().await.add_client(username.clone(), tx.clone()).await?;
    
        Ok(ClientHandler {
            username,
            room_manager,
            client_manager,
            tx,
            rx,
        })
    }

    pub async fn handle(mut self, mut stream: TcpStream) -> Result<(), ChatError> {
        let (mut reader, mut writer) = stream.split();
        let mut buf = [0u8; 1024];

        loop {
            tokio::select! {
                result = reader.read(&mut buf) => {
                    match result {
                        Ok(0) => break, // Connection closed
                        Ok(n) => {
                            let msg: Message = serde_json::from_slice(&buf[..n])?;
                            if msg.content.starts_with('/') {
                                self.handle_command(&msg.content).await?;
                            } else {
                                let mut room_manager = self.room_manager.lock().await;
                                room_manager.broadcast_message(msg).await?;
                            }
                        }
                        Err(e) => return Err(e.into()),
                    }
                }
                Some(msg) = self.rx.recv() => {
                    let data = serde_json::to_vec(&msg)?;
                    writer.write_all(&data).await?;
                }
            }
        }

        // Cleanup
        let mut client_manager = self.client_manager.lock().await;
        client_manager.remove_client(&self.username).await?;

        Ok(())
    }

    async fn handle_command(&mut self, cmd_str: &str) -> Result<(), ChatError> {
        let parts: Vec<&str> = cmd_str.split_whitespace().collect();
        match parts.get(0) {
            Some(&"/join") => {
                if let Some(&room) = parts.get(1) {
                    let mut room_manager = self.room_manager.lock().await;
                    room_manager.join_room(&self.username, room).await?;
                } else {
                    return Err(ChatError {
                        kind: ChatErrorKind::Command,
                        message: "Usage: /join <room>".to_string(),
                    });
                }
            }
            Some(&"/leave") => {
                if let Some(&room) = parts.get(1) {
                    let mut room_manager = self.room_manager.lock().await;
                    room_manager.leave_room(&self.username, room).await?;
                } else {
                    return Err(ChatError {
                        kind: ChatErrorKind::Command,
                        message: "Usage: /leave <room>".to_string(),
                    });
                }
            }
            Some(&"/quit") => {
                let mut client_manager = self.client_manager.lock().await;
                client_manager.remove_client(&self.username).await?;
            }
            _ => {
                return Err(ChatError {
                    kind: ChatErrorKind::Command,
                    message: "Unknown command".to_string(),
                });
            }
        }
        Ok(())
    }
}
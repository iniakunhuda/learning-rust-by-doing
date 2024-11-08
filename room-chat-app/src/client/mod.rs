use tokio::net::TcpStream;
use crate::common::ChatError;
use tokio::sync::mpsc;

pub mod handler;
pub mod ui;

pub struct ChatClient {
    stream: TcpStream,
    username: String,
}

impl ChatClient {
    pub async fn new(address: &str, username: String) -> Result<Self, ChatError> {
        let stream = TcpStream::connect(address).await?;
        
        Ok(ChatClient { 
            stream,
            username,
        })
    }

    pub async fn run(&mut self) -> Result<(), ChatError> {
        let (tx, _rx) = mpsc::channel(100);
        
        // Create a new connection for the handler
        let handler_stream = TcpStream::connect(self.stream.peer_addr()?).await?;
        
        let handler = handler::ClientHandler::new(
            self.username.clone(),
            tx.clone(),
        );
        
        let mut ui = ui::UI::new(tx);
        
        tokio::select! {
            result = handler.run(handler_stream) => result?,
            result = ui.run() => result?,
        }

        Ok(())
    }
}
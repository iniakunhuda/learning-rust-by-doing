use crate::common::{ChatError, Message};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct ClientHandler {
    username: String,
    tx: mpsc::Sender<Message>,
}

impl ClientHandler {
    pub fn new(username: String, tx: mpsc::Sender<Message>) -> Self {
        ClientHandler {
            username,
            tx,
        }
    }

    pub async fn run(&self, mut stream: TcpStream) -> Result<(), ChatError> {
        let (mut reader, mut writer) = stream.split();
        let mut buf = [0u8; 1024];

        loop {
            match reader.read(&mut buf).await {
                Ok(0) => break, // Connection closed
                Ok(n) => {
                    let msg: Message = serde_json::from_slice(&buf[..n])?;
                    self.tx.send(msg).await.map_err(|e| ChatError {
                        kind: crate::common::ChatErrorKind::Message,
                        message: e.to_string(),
                    })?;
                }
                Err(e) => return Err(e.into()),
            }
        }
        Ok(())
    }
}
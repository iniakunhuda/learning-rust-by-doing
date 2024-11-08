use std::collections::HashMap;
use tokio::sync::mpsc;
use crate::common::{ChatError, Message};

pub struct ClientManager {
    clients: HashMap<String, mpsc::Sender<Message>>,
}

impl ClientManager {
    pub fn new() -> Self {
        ClientManager {
            clients: HashMap::new(),
        }
    }

    pub async fn add_client(
        &mut self,
        username: String,
        tx: mpsc::Sender<Message>,
    ) -> Result<(), ChatError> {
        if self.clients.contains_key(&username) {
            return Err(ChatError {
                kind: crate::common::ChatErrorKind::Authentication,
                message: "Username already taken".to_string(),
            });
        }
        self.clients.insert(username, tx);
        Ok(())
    }

    pub async fn remove_client(&mut self, username: &str) -> Result<(), ChatError> {
        self.clients.remove(username).ok_or(ChatError {
            kind: crate::common::ChatErrorKind::Authentication,
            message: "Client not found".to_string(),
        })?;
        Ok(())
    }
}
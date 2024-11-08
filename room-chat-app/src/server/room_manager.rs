use crate::common::{ChatError, Message, Room};
use std::collections::HashMap;
use tokio::sync::mpsc;

pub struct RoomManager {
    rooms: HashMap<String, Room>,
    clients: HashMap<String, mpsc::Sender<Message>>,
}

impl RoomManager {
    pub fn new() -> Self {
        RoomManager {
            rooms: HashMap::new(),
            clients: HashMap::new(),
        }
    }

    pub async fn create_room(&mut self, name: String) -> Result<(), ChatError> {
        if self.rooms.contains_key(&name) {
            return Err(ChatError {
                kind: crate::common::ChatErrorKind::Room,
                message: "Room already exists".to_string(),
            });
        }
        self.rooms.insert(name.clone(), Room::new(name));
        Ok(())
    }

    pub async fn join_room(&mut self, username: &str, room_name: &str) -> Result<(), ChatError> {
        let room = self.rooms.get_mut(room_name).ok_or(ChatError {
            kind: crate::common::ChatErrorKind::Room,
            message: "Room does not exist".to_string(),
        })?;

        if room.add_user(username.to_string()) {
            self.broadcast_message(Message::new(
                room_name.to_string(),
                "System".to_string(),
                format!("{} has joined the room", username),
            ))
            .await?;
        }
        Ok(())
    }

    pub async fn leave_room(&mut self, username: &str, room_name: &str) -> Result<(), ChatError> {
        let room = self.rooms.get_mut(room_name).ok_or(ChatError {
            kind: crate::common::ChatErrorKind::Room,
            message: "Room does not exist".to_string(),
        })?;

        if room.remove_user(username) {
            self.broadcast_message(Message::new(
                room_name.to_string(),
                "System".to_string(),
                format!("{} has left the room", username),
            ))
            .await?;
        }
        Ok(())
    }

    pub async fn broadcast_message(&mut self, message: Message) -> Result<(), ChatError> {
        let room = self.rooms.get(&message.room).ok_or(ChatError {
            kind: crate::common::ChatErrorKind::Room,
            message: "Room does not exist".to_string(),
        })?;

        for username in &room.users {
            if let Some(client) = self.clients.get(username) {
                client.send(message.clone()).await.map_err(|e| ChatError {
                    kind: crate::common::ChatErrorKind::Message,
                    message: e.to_string(),
                })?;
            }
        }
        Ok(())
    }

    pub async fn list_rooms(&self) -> Vec<String> {
        self.rooms.keys().cloned().collect()
    }

    pub async fn list_users(&self, room_name: &str) -> Result<Vec<String>, ChatError> {
        let room = self.rooms.get(room_name).ok_or(ChatError {
            kind: crate::common::ChatErrorKind::Room,
            message: "Room does not exist".to_string(),
        })?;
        Ok(room.users.clone())
    }
}
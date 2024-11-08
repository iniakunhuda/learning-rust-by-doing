use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: u64,
    pub room: String,
    pub sender: String,
    pub content: String,
    pub timestamp: SystemTime,
}

impl Message {
    pub fn new(room: String, sender: String, content: String) -> Self {
        Message {
            id: rand::random(),
            room,
            sender,
            content,
            timestamp: SystemTime::now(),
        }
    }
}
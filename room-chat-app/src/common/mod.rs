// src/common/mod.rs
pub mod message;
pub mod room;

pub use message::Message;
pub use room::Room;

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Command {
    Join(String),    // room name
    Leave(String),   // room name
    Quit,
    ListRooms,
    ListUsers(String), // room name
}

#[derive(Debug)]
pub struct ChatError {
    pub kind: ChatErrorKind,
    pub message: String,
}

#[derive(Debug)]
pub enum ChatErrorKind {
    Connection,
    Authentication,
    Room,
    Message,
    Internal,
    Command,
    IO,
    Serialization,
}

impl fmt::Display for ChatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.message)
    }
}

impl Error for ChatError {}

// Add conversion implementations for various error types
impl From<io::Error> for ChatError {
    fn from(error: io::Error) -> Self {
        ChatError {
            kind: ChatErrorKind::IO,
            message: error.to_string(),
        }
    }
}

impl From<serde_json::Error> for ChatError {
    fn from(error: serde_json::Error) -> Self {
        ChatError {
            kind: ChatErrorKind::Serialization,
            message: error.to_string(),
        }
    }
}
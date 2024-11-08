use super::Message;
use std::collections::VecDeque;

const MAX_HISTORY: usize = 100;

#[derive(Debug, Clone)]
pub struct Room {
    pub name: String,
    pub users: Vec<String>,
    pub history: VecDeque<Message>,
}

impl Room {
    pub fn new(name: String) -> Self {
        Room {
            name,
            users: Vec::new(),
            history: VecDeque::with_capacity(MAX_HISTORY),
        }
    }

    pub fn add_message(&mut self, message: Message) {
        if self.history.len() >= MAX_HISTORY {
            self.history.pop_front();
        }
        self.history.push_back(message);
    }

    pub fn add_user(&mut self, username: String) -> bool {
        if !self.users.contains(&username) {
            self.users.push(username);
            true
        } else {
            false
        }
    }

    pub fn remove_user(&mut self, username: &str) -> bool {
        if let Some(pos) = self.users.iter().position(|x| x == username) {
            self.users.remove(pos);
            true
        } else {
            false
        }
    }
}
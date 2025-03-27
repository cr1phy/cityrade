use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub struct GlobalChat {
    pub messages: Vec<ChatMessage>,
}

impl GlobalChat {
    pub fn new() -> GlobalChat {
        GlobalChat {
            messages: Vec::new(),
        }
    }

    pub fn add_message(&mut self, message: ChatMessage) {
        self.messages.push(message);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub username: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

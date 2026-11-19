use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ChatMessage {
    pub author: String,
    pub content: String,
}
impl Display for ChatMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]: {}", self.author, self.content)
    }
}

use std::fmt::Display;

use crate::data::chat_message::ChatMessage;

pub struct ServerClient {
    pub messages: Vec<ChatMessage>,
}
impl Display for ServerClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some((last_message, history)) = self.messages.split_last() {
            writeln!(f, "<chat_history_start>")?;
            for message in history {
                writeln!(f, "{message}")?;
            }
            writeln!(f, "</chat_history_end>")?;
            writeln!(f, "<current_user_message>")?;
            writeln!(f, "{last_message}")?;
            writeln!(f, "</current_user_message>")?;
        }
        Ok(())
    }
}

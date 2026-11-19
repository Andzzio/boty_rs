use std::{
    fmt::Display,
    fs::{create_dir_all, write},
};

use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use serenity::model::id::GuildId;

use crate::data::chat_message::ChatMessage;

const HISTORY_LIMIT: usize = 30;
const HISTORY_DRAIN: usize = 15;

#[derive(Deserialize, Serialize)]
pub struct ServerClient {
    pub messages: Vec<ChatMessage>,
    pub summary: String,
}
impl ServerClient {
    pub fn prepare_to_compress(&mut self) -> Option<String> {
        if self.messages.len() <= HISTORY_LIMIT {
            return None;
        }
        let mut extracted = String::new();
        for drained_message in self.messages.drain(0..HISTORY_DRAIN) {
            extracted.push_str(&format!("{}\n", drained_message));
        }
        let prompt = format!(
            "Instrucción interna: Eres el sistema de memoria. NO respondas como en un chat.
            Actualiza el <resumen_previo> si es que no está vacío, fusionándolo con los <mensajes_antiguos>
            en unos párrafos intentando no perder información, solo puedes crear máximo 2 párrafos de 8 líneas cada uno (no es obligatorio que sea así) conservando nombres de usuarios y hechos clave.
            Devuelve ÚNICAMENTE el texto del resumen.\n\n
            <resumen_previo>\n{}\n</resumen_previo>\n\n
            <mensajes_antiguos>\n{extracted}</mensajes_antiguos>",
            self.summary
        );
        Some(prompt)
    }
    pub fn save(&self, guild_id: GuildId) -> anyhow::Result<()> {
        let path = format!("history/{guild_id}");
        create_dir_all(&path)?;
        let json = to_string_pretty(self)?;
        write(format!("{path}/chat_state.json"), json)?;
        Ok(())
    }
}
impl Display for ServerClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some((last_message, history)) = self.messages.split_last() {
            writeln!(f, "<chat_history_start>")?;
            for message in history {
                writeln!(f, "{message}")?;
            }
            if !self.summary.is_empty() {
                writeln!(f, "<previous_history_summary>")?;
                writeln!(f, "{}", self.summary)?;
                writeln!(f, "</previous_history_summary>")?;
            }
            writeln!(f, "</chat_history_end>")?;
            writeln!(f, "<current_user_message>")?;
            writeln!(f, "{last_message}")?;
            writeln!(f, "</current_user_message>")?;
        }
        Ok(())
    }
}

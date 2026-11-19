use std::{
    collections::HashMap,
    fs::{read_dir, read_to_string},
};

use serde_json::from_str;
use serenity::model::id::GuildId;

use crate::data::server_client::ServerClient;

pub fn load_history() -> HashMap<GuildId, ServerClient> {
    let mut clients: HashMap<GuildId, ServerClient> = HashMap::new();
    let Ok(entries) = read_dir("history") else {
        return clients;
    };
    for entry in entries.flatten() {
        let folder_name = entry.file_name();
        if let Some(folder_name) = folder_name.to_str()
            && let Ok(guild_id) = folder_name.parse::<u64>()
        {
            let guild_id = GuildId::new(guild_id);
            let file_path = entry.path().join("chat_state.json");
            if let Ok(content) = read_to_string(file_path)
                && let Ok(server_client) = from_str::<ServerClient>(&content)
            {
                clients.insert(guild_id, server_client);
            }
        }
    }
    clients
}

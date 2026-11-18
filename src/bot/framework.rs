use std::collections::HashMap;

use poise::{Framework, FrameworkOptions};
use serenity::model::id::GuildId;
use tokio::sync::RwLock;

use crate::{
    bot::{
        commands::{hablar, hola},
        event_handler::event_handler,
    },
    data::{chat_message::ChatMessage, server_client::ServerClient},
    llm::llm_provider::LLMProvider,
};

pub struct Data {
    pub llm: Box<dyn LLMProvider>,
    pub server_clients: RwLock<HashMap<GuildId, ServerClient>>,
}
pub type Error = anyhow::Error;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub fn build_framework(
    llm: Box<dyn LLMProvider>,
    server_clients: RwLock<HashMap<GuildId, ServerClient>>,
) -> Framework<Data, Error> {
    poise::Framework::builder()
        .options(FrameworkOptions {
            commands: vec![hola(), hablar()],
            event_handler: |framework, event| Box::pin(event_handler(framework, event)),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    llm,
                    server_clients,
                })
            })
        })
        .build()
}
pub async fn record_message(
    framework: poise::FrameworkContext<'_, Data, Error>,
    guild_id: Option<GuildId>,
    content: String,
    author_name: String,
) {
    if let Some(guild_id) = guild_id {
        let mut clients = framework.user_data.server_clients.write().await;
        let server = clients.entry(guild_id).or_insert(ServerClient {
            messages: Vec::new(),
        });
        server.messages.push(ChatMessage {
            author: author_name,
            content,
        });
        print!("\x1B[2J\x1B[1;1H");
        println!("{server}");
    }
}
pub async fn get_message_history(
    framework: poise::FrameworkContext<'_, Data, Error>,
    guild_id: Option<GuildId>,
) -> String {
    if let Some(guild_id) = guild_id {
        let mut clients = framework.user_data.server_clients.write().await;
        let server = clients.entry(guild_id).or_insert(ServerClient {
            messages: Vec::new(),
        });
        return format!("{server}");
    }
    String::new()
}

use serenity::{client::ClientBuilder, model::id::GuildId, prelude::*};
use std::{collections::HashMap, fs};

use boty_rs::{
    bot::framework::build_framework,
    cli::root::Root,
    config::toml_config::{TomlConfig, load_config},
    data::server_client::ServerClient,
    llm::gemini::gemini_client::GeminiClient,
};
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let root = Root::parse();
    let content = fs::read_to_string(&root.config_path)?;
    let config: TomlConfig = load_config(&content)?;
    let dc_token = config.keys.discord_token;
    let llm_token = config.keys.llm_token;
    let tavily_token = config.keys.tavily_token;
    let server_clients: RwLock<HashMap<GuildId, ServerClient>> = RwLock::default();
    let client = reqwest::Client::new();
    let gemini_client = GeminiClient::new(llm_token, tavily_token, client);
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let mut client = ClientBuilder::new(dc_token, intents)
        .framework(build_framework(Box::new(gemini_client), server_clients))
        .await
        .expect("Err creating client");
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
    Ok(())
}

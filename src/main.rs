use serenity::{client::ClientBuilder, prelude::*};
use std::fs;

use boty_rs::{
    bot::{commands::build_framework, handler::Handler},
    cli::root::Root,
    config::toml_config::{TomlConfig, load_config},
};
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let root = Root::parse();
    let content = fs::read_to_string(&root.config_path)?;
    let config: TomlConfig = load_config(&content)?;
    let dc_token = config.keys.discord_token;
    let llm_token = config.keys.llm_token;
    println!("Discord Token: {dc_token:?}");
    println!("LLM Token: {llm_token:?}");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let mut client = ClientBuilder::new(dc_token, intents)
        .framework(build_framework())
        .event_handler(Handler)
        .await
        .expect("Err creating client");
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
    Ok(())
}

use serenity::prelude::*;
use std::fs;

use anyhow::Ok;
use boty_rs::{
    bot::handler::Handler,
    cli::root::Root,
    config::toml_config::{TomlConfig, load_config},
};
use clap::Parser;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let root = Root::parse();
    let content = fs::read_to_string(&root.config_path)?;
    let config: TomlConfig = load_config(&content)?;
    let token = config.keys.token;
    println!("Token: {token:?}");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
    Ok(())
}

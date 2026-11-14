use std::fs;

use anyhow::Ok;
use boty_rs::{
    cli::root::Root,
    config::toml_config::{TomlConfig, load_config},
};
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let root = Root::parse();
    let content = fs::read_to_string(&root.config_path)?;
    let config: TomlConfig = load_config(&content)?;
    let token = config.keys.token;
    println!("Token: {token:?}");
    Ok(())
}

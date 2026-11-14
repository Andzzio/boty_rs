use std::fs;

use boty_rs::{cli::root::Root, config::toml_config::TomlConfig};
use clap::Parser;

fn main() {
    let root = Root::parse();
    let content = fs::read_to_string(&root.config_path).unwrap();
    let config: TomlConfig = toml::from_str(&content).unwrap();
    let token = config.keys.token;
    println!("Token: {token:?}");
}

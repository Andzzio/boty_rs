use serde::Deserialize;

#[derive(Deserialize)]
pub struct TomlConfig {
    pub keys: Keys,
}
#[derive(Deserialize)]
pub struct Keys {
    pub discord_token: String,
    pub llm_token: String,
}
pub fn load_config(content: &str) -> anyhow::Result<TomlConfig> {
    let cfg = toml::from_str(content)?;
    Ok(cfg)
}

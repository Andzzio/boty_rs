use serde::Deserialize;

#[derive(Deserialize)]
pub struct TomlConfig {
    pub keys: Keys,
}
#[derive(Deserialize)]
pub struct Keys {
    pub token: String,
}

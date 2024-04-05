use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub woo: Woo,
}

impl Config {
    pub fn new<T: ToString>(file_name: T) -> Result<Self> {
        let file = std::fs::read_to_string(file_name.to_string())?;
        let config: Config = toml::from_str(&file)?;
        Ok(config)
    }
}

#[derive(Deserialize)]
pub struct Woo {
    pub ck: String,
    pub cs: String,
    pub host: String,
}

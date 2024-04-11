use anyhow::Result;
use serde::Deserialize;

/// Configuration struct for storing Woo parameters
#[derive(Deserialize)]
pub struct Config {
    /// The Woo struct containing ck, cs, and host strings
    pub woo: Woo,
}

impl Config {
    /// Create a new Config instance by reading from a file
    ///
    /// # Arguments
    ///
    /// * `file_name` - A file name (path) to read the configuration from
    ///
    /// # Returns
    ///
    /// A Result containing the Config instance if successful, or an error
    pub fn new<T: ToString>(file_name: T) -> Result<Self> {
        let file = std::fs::read_to_string(file_name.to_string())?;
        let config: Config = toml::from_str(&file)?;
        Ok(config)
    }
}
/// Woo struct for storing ck, cs, and host strings
#[derive(Deserialize)]
pub struct Woo {
    /// Consumer Key
    pub ck: String,
    /// Consumer Secret
    pub cs: String,
    /// Host URL
    pub host: String,
}

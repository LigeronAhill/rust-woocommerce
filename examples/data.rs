use rust_woocommerce::{ApiClient, Config, Data};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let config = Config::new("woo.toml")?;
    let client = ApiClient::new(&config)?;
    let data = client.list_all::<Data>().await?;
    info!("Got {} data", data.len());
    Ok(())
}

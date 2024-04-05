use anyhow::Result;
use rust_woocommerce::{products::Product, ApiClient, Config};
use tracing::info;
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let config = Config::new("woo.toml")?;
    let client = ApiClient::new(&config)?;
    let products = client.list_all::<Product>().await?;
    info!("Got {} products", products.len());
    let random_id = products.first().map(|p| p.id).unwrap_or_default();
    let retrieved = client.retrieve::<Product>(random_id).await?;
    info!("Retrieved product with sku: {}", retrieved.sku);
    Ok(())
}

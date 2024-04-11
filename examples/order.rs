use anyhow::anyhow;
use rust_woocommerce::Order;
use rust_woocommerce::{ApiClient, Config};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let config = Config::new("woo.toml")?;
    let client = ApiClient::new(&config)?;
    let orders = client.list_all::<Order>().await?;
    info!("Got {} orders", orders.len());
    let random_order_id = orders.first().ok_or(anyhow!("Error"))?.id;
    let retrieved_order = client.retrieve::<Order>(random_order_id).await?;
    info!(
        "Got order with number: {} with total: {}",
        retrieved_order.number, retrieved_order.total
    );
    Ok(())
}

use rust_woocommerce::Customer;
use rust_woocommerce::{ApiClient, Config};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let config = Config::new("woo.toml")?;
    let client = ApiClient::new(&config)?;
    let customers = client.list_all::<Customer>().await?;
    info!("Got {} customers", customers.len());
    let retrieved: Customer = client.retrieve(customers.first().unwrap().id).await?;
    info!("Retrieved customer name is {}", retrieved.first_name);
    Ok(())
}

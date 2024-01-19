// use rust_woocommerce::{controllers::ApiClient, Result};

use rust_woocommerce::{ApiClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let _client = ApiClient::from_env().unwrap();

    Ok(())
}

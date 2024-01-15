use rust_woocommerce::{controllers::ApiClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let _client = ApiClient::from_env().unwrap();

    Ok(())
}

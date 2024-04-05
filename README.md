# rust-woocommerce

Library for working with woocommerce API with Rust programming language.

## Usage

From environment:

```rust
use anyhow::Result;
use rust_woocommerce::{products::Product, ApiClient};
#[tokio::main]
async fn main() -> Result<()> {
    let client = ApiClient::from_env()?;
    let products = client.list_all::<Product>().await?;
    println!("Got {} products", products.len());
    let first = products.first().map(|p| p.name.clone()).unwrap_or_default();
    let last = products.last().map(|p| p.name.clone()).unwrap_or_default();
    println!("First product: {first}\nLast product: {last}");
    Ok(())
}
```

With builder:

```rust
use anyhow::Result;
use rust_woocommerce::{products::Product, ApiClient};
#[tokio::main]
async fn main() -> Result<()> {
    let site_url = "https://google.com";
    let consumer_key = "super6secret9";
    let consumer_secret = "consumer4secret2";
    let client = rust_woocommerce::ApiClient::builder()
        .auth(consumer_key, consumer_secret)
        .site_url(site_url)
        .build();
    let products = client.list_all::<Product>().await?;
    Ok(())
}
```


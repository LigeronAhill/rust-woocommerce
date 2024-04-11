# rust-woocommerce

Library for working with woocommerce API with Rust programming language.

## Usage


```rust
use anyhow::Result;
use rust_woocommerce::{products::Product, ApiClient, Config};
use tracing::info;
use rust_woocommerce::product_attributes::Attribute;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let config = Config::new("woo.toml")?;
    let client = ApiClient::new(&config)?;
    let start = std::time::Instant::now();
    let products = client.list_all::<Product>().await?;
    info!("Got {} products in {} seconds", products.len(), start.elapsed().as_secs());
    let random_id = products.first().map(|p| p.id).unwrap_or_default();
    let retrieved = client.retrieve::<Product>(random_id).await?;
    info!("Retrieved product with sku: {}", retrieved.sku);
    let attribute = Attribute::builder().name("Test Attribute").option("Best").visible().build();
    let new_product = Product::builder()
        .name("Test Product For Example")
        .featured()
        .short_description("The most professional description")
        .sku("product for test 42")
        .regular_price("6969")
        .manage_stock()
        .stock_quantity(42)
        .weight("50")
        .dimensions("4", "3", "2")
        .shipping_class("large")
        .images("https://cs14.pikabu.ru/post_img/2021/06/27/7/1624794514137159585.jpg")
        .attribute(attribute)
        .build();
    let created: Product = client.create(new_product).await?;
    info!("Create product {} with id: {}", created.name, created.id);
    let update = Product::builder().unfeatured().build();
    let updated: Product = client.update(created.id, update).await?;
    info!("Update product {}, new feature is {}", updated.name, updated.featured);
    let deleted: Product = client.delete(updated.id).await?;
    info!("Product {} deleted", deleted.name);
    Ok(())
}
```

Configuration file example:

```toml
[woo]
ck = "ck_6969696969696969696969696969696969696969"
cs = "cs_4242424242424242424242424242424242424242"
host = "wordpress.org"
```

Children:

```rust
use anyhow::Result;
use tracing::info;

use rust_woocommerce::{ApiClient, Config};
use rust_woocommerce::product_attributes::Attribute;
use rust_woocommerce::product_variations::ProductVariation;
use rust_woocommerce::products::{Product, ProductType};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let config = Config::new("woo.toml")?;
    let client = ApiClient::new(&config)?;
    let products = client.list_all::<Product>().await?;
    let random_variable_id = products.iter().find(|p| !p.variations.is_empty()).map(|p| p.id).unwrap_or_default();
    let variations = client.list_all_subentities::<ProductVariation>(random_variable_id).await?;
    info!("Got {} variations for product with id: {random_variable_id}", variations.len());
    let retrieved_variation: ProductVariation = client.retrieve_subentity(random_variable_id, variations.first().map(|v| v.id).unwrap_or_default()).await?;
    info!("Retrieved variation has sku: {}", retrieved_variation.sku);
    let attribute = Attribute::builder().name("Test Attribute").option("Best").option("Test").variation().visible().build();
    let new_variable_product = Product::builder()
        .name("Test Product For Example")
        .product_type(ProductType::Variable)
        .featured()
        .short_description("The most professional description")
        .sku("product for test 42")
        .regular_price("6969")
        .manage_stock()
        .stock_quantity(42)
        .weight("50")
        .dimensions("4", "3", "2")
        .shipping_class("large")
        .images("https://cs14.pikabu.ru/post_img/2021/06/27/7/1624794514137159585.jpg")
        .attribute(attribute)
        .build();
    let created: Product = client.create(new_variable_product).await?;
    info!("Create product {} with id: {}", created.name, created.id);
    
    let variation = ProductVariation::create()
        .sku(format!("{} Best", created.sku))
        .regular_price("6969")
        .manage_stock()
        .stock_quantity(96)
        .weight("52")
        .dimensions("5", "4", "3")
        .attribute(None, "Test Attribute", "Best")
        .build();
    let created_variation: ProductVariation = client.create_subentity(created.id, variation).await?;
    info!("Variation {} created with price: {}", created_variation.sku, created_variation.price);
    let update = ProductVariation::update().regular_price("7000").build();
    let updated_variation: ProductVariation = client.update_subentity(created.id, created_variation.id, update).await?;
    info!("Variation {} updated with price: {}", updated_variation.sku, updated_variation.price);
    let deleted_variation: ProductVariation = client.delete_subentity(created.id, updated_variation.id).await?;
    info!("Variation {} deleted", deleted_variation.sku);
    let deleted: Product = client.delete(created.id).await?;
    info!("Product {} deleted", deleted.name);
    Ok(())
}
```
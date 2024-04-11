use anyhow::{anyhow, Result};
use rust_woocommerce::{ApiClient, Attribute, Config, Product};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let config = Config::new("woo.toml")?;
    let client = ApiClient::new(&config)?;
    let start = std::time::Instant::now();
    let products = client.list_all::<Product>().await?;
    info!(
        "Got {} products in {} seconds",
        products.len(),
        start.elapsed().as_secs()
    );
    let random_id = products.first().map(|p| p.id).unwrap_or_default();
    let retrieved = client.retrieve::<Product>(random_id).await?;
    info!("Retrieved product has sku: {}", retrieved.sku);
    let attribute = Attribute::builder()
        .name("Test Attribute")
        .option("Best")
        .visible()
        .build();
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
    let batch_create = vec![new_product.clone()];
    let created: Product = client.create(new_product).await?;
    info!("Create product {} with id: {}", created.name, created.id);
    let update = Product::builder().unfeatured().build();
    let updated: Product = client.update(created.id, update).await?;
    info!(
        "Update product {}, new feature is {}",
        updated.name, updated.featured
    );
    let deleted: Product = client.delete(updated.id).await?;
    info!("Product {} deleted", deleted.name);
    let batch_created: Vec<Product> = client.batch_create(batch_create).await?;
    let id = batch_created.first().ok_or(anyhow!("Error"))?.id;
    let batch_update = Product::builder().id(id).unfeatured().build();
    let _batch_updated: Vec<Product> = client.batch_update(vec![batch_update]).await?;
    let _deleted: Product = client.delete(id).await?;
    Ok(())
}

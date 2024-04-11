use rust_woocommerce::{ApiClient, Config};
use rust_woocommerce::{Category, DisplayOption};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let config = Config::new("woo.toml")?;
    let client = ApiClient::new(&config)?;
    let categories = client.list_all::<Category>().await?;
    info!("Got {} categories", categories.len());
    let random_id = categories.first().unwrap().id;
    let retrieved: Category = client.retrieve(random_id).await?;
    info!("Retrieved category name: {}", retrieved.name);
    let create = Category::create("Test Category")
        .parent(retrieved.id)
        .description("Test description")
        .display(DisplayOption::Products)
        .image("https://woocommerce.github.io/woocommerce-rest-api-docs/images/logo-95f5c1ab.png");
    let batch_create = create.clone();
    let created: Category = client.create(create).await?;
    info!("Category with id: {} created", created.id);
    let update = Category::update().description("Some description");
    let updated: Category = client.update(created.id, update).await?;
    info!("New description is {}", updated.description);
    let deleted: Category = client.delete(updated.id).await?;
    info!("Category {} deleted", deleted.name);
    let batch_created: Vec<Category> = client.batch_create(vec![batch_create]).await?;
    info!("Batch created {} categories", batch_created.len());
    let batch_update = Category::update()
        .id(batch_created.first().unwrap().id)
        .description("Some description");
    let batch_updated: Vec<Category> = client.batch_update(vec![batch_update]).await?;
    let id = batch_updated.first().unwrap().id;
    info!("Batch updated categories contains category with id: {id}");
    let batch_deleted: Vec<Category> = client.batch_delete(vec![id]).await?;
    info!("Deleted {} categories", batch_deleted.len());
    Ok(())
}

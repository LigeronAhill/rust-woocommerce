use anyhow::{anyhow, Result};
use serde::Serialize;
use tokio::task::JoinSet;
use url::Url;

use crate::{ApiClient, BatchObject};

use super::Entity;

const BATCH: &str = "batch";

impl ApiClient {
    /// This API lets you retrieve and view a specific entity by ID.
    ///
    ///
    /// # Example
    ///
    /// ```no_run
    /// use anyhow::Result;
    /// use rust_woocommerce::{Product, ApiClient, Config};
    /// use tracing::info;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     tracing_subscriber::fmt::init();
    ///     let config = Config::new("woo.toml")?;
    ///     let client = ApiClient::new(&config)?;
    ///     let retrieved = client.retrieve::<Product>(12345).await?;
    ///     info!("Retrieved product has sku: {}", retrieved.sku);
    ///     Ok(())
    /// }
    /// ```
    pub async fn retrieve<T: Entity>(&self, entity_id: i32) -> Result<T> {
        let uri = self
            .base_url
            .join(&T::endpoint())?
            .join(&entity_id.to_string())?;
        self.get_request_with_tries(uri, 3).await
    }

    async fn get_request_with_tries<T: Entity>(&self, uri: Url, tries: i32) -> Result<T> {
        for i in 1..tries {
            tracing::debug!("Connecting {uri}, try {i}");
            match self
                .client
                .get(uri.clone())
                .basic_auth(self.ck(), Some(self.cs()))
                .send()
                .await?
                .json::<T>()
                .await
            {
                Ok(r) => {
                    return Ok(r);
                }
                Err(e) => {
                    tracing::error!(
                        "Failed to connect to {uri} with error: {e}\n{} tries left",
                        tries - i
                    );
                    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                    continue;
                }
            }
        }
        Err(anyhow!(
            "Error retrieving entity with uri: {}",
            uri.to_string()
        ))
    }
    /// This API helps you to view all entities of type T.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use anyhow::Result;
    /// use rust_woocommerce::{Product, ApiClient, Config};
    /// use tracing::info;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     tracing_subscriber::fmt::init();
    ///     let config = Config::new("woo.toml")?;
    ///     let client = ApiClient::new(&config)?;
    ///     let products = client.list_all::<Product>().await?;
    ///     info!("Got {} products", products.len());
    ///     Ok(())
    /// }
    /// ```
    pub async fn list_all<T: Entity>(&self) -> Result<Vec<T>> {
        let uri = self.base_url.join(&T::endpoint())?;
        let mut result = Vec::new();
        let mut set = JoinSet::new();
        let total_response = self
            .client
            .get(uri.clone())
            .basic_auth(&self.ck(), Some(self.cs()))
            .send()
            .await?;
        let total = total_response
            .headers()
            .get("X-WP-Total")
            .and_then(|h| h.to_str().ok().map(|p| p.parse::<i32>().ok()))
            .flatten()
            .unwrap_or_default();
        let per_page = 50;
        let total_pages = total / per_page + 1;
        for page in 1..=total_pages {
            let client = self.client();
            let ck = self.ck();
            let cs = Some(self.cs());
            let url = uri.clone();
            set.spawn(async move {
                client
                    .get(url)
                    .query(&[("page", page), ("per_page", per_page)])
                    .basic_auth(ck, cs)
                    .send()
                    .await?
                    .json::<Vec<T>>()
                    .await
            });
        }
        while let Some(Ok(Ok(v))) = set.join_next().await {
            result.extend(v)
        }
        Ok(result)
    }
    /// This API helps you to create a new entity of type T.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use anyhow::Result;
    /// use rust_woocommerce::{Product, ApiClient, Config, Attribute};
    /// use tracing::info;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     tracing_subscriber::fmt::init();
    ///     let config = Config::new("woo.toml")?;
    ///     let client = ApiClient::new(&config)?;
    ///     let attribute = Attribute::builder()
    ///         .name("Test Attribute")
    ///         .option("Best")
    ///         .visible()
    ///         .build();
    ///     let new_product = Product::builder()
    ///         .name("Test Product For Example")
    ///         .featured()
    ///         .short_description("The most professional description")
    ///         .sku("product for test 42")
    ///         .regular_price("6969")
    ///         .manage_stock()
    ///         .stock_quantity(42)
    ///         .weight("50")
    ///         .dimensions("4", "3", "2")
    ///         .shipping_class("large")
    ///         .images("https://cs14.pikabu.ru/post_img/2021/06/27/7/1624794514137159585.jpg")
    ///         .attribute(attribute)
    ///         .build();
    ///     let created: Product = client.create(new_product).await?;
    ///     info!("Create product {} with id: {}", created.name, created.id);
    ///     Ok(())
    /// }
    /// ```
    pub async fn create<T: Entity>(&self, object: impl Serialize) -> Result<T> {
        let uri = self.base_url.join(&T::endpoint())?;
        self.post_request_with_tries(&object, uri, 3).await
    }

    async fn post_request_with_tries<T: Entity, O: Serialize + Sized>(
        &self,
        object: &O,
        uri: Url,
        tries: i32,
    ) -> Result<T> {
        for i in 1..tries {
            tracing::debug!("Connecting {uri}, try {i}");
            match self
                .client
                .post(uri.clone())
                .basic_auth(self.ck(), Some(self.cs()))
                .json(&object)
                .send()
                .await?
                .json::<T>()
                .await
            {
                Ok(r) => {
                    return Ok(r);
                }
                Err(e) => {
                    tracing::error!(
                        "Failed to connect to {uri} with error: {e}\n{} tries left",
                        tries - i
                    );
                    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
                    continue;
                }
            }
        }
        Err(anyhow!(
            "Error creating entity with uri: {}",
            uri.to_string()
        ))
    }
    /// This API lets you make changes to entity.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use anyhow::{anyhow, Result};
    /// use rust_woocommerce::{Product, ApiClient, Config};
    /// use tracing::info;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     tracing_subscriber::fmt::init();
    ///     let config = Config::new("woo.toml")?;
    ///     let client = ApiClient::new(&config)?;
    ///     let update = Product::builder().unfeatured().build();
    ///     let updated: Product = client.update(12345, update).await?;
    ///     info!(
    ///     "Update product {}, new feature is {}",
    ///     updated.name, updated.featured
    /// );
    ///     Ok(())
    /// }
    /// ```
    pub async fn update<T: Entity>(&self, entity_id: i32, object: impl Serialize) -> Result<T> {
        let uri = self
            .base_url
            .join(&T::endpoint())?
            .join(&entity_id.to_string())?;
        self.put_request_with_tries(&object, uri, 3).await
    }

    async fn put_request_with_tries<T: Entity, O: Serialize + Sized>(
        &self,
        object: &O,
        uri: Url,
        tries: i32,
    ) -> Result<T> {
        for i in 1..tries {
            tracing::debug!("Connecting {uri}, try {i}");
            match self
                .client
                .put(uri.clone())
                .basic_auth(self.ck(), Some(self.cs()))
                .json(&object)
                .send()
                .await?
                .json::<T>()
                .await
            {
                Ok(r) => return Ok(r),
                Err(e) => {
                    tracing::error!(
                        "Failed to connect to {uri} with error: {e}\n{} tries left",
                        tries - i
                    );
                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                    continue;
                }
            }
        }
        Err(anyhow!(
            "Error updating entity with uri: {}",
            uri.to_string()
        ))
    }
    /// This API helps you delete a product.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use anyhow::Result;
    /// use rust_woocommerce::{Product, ApiClient, Config};
    /// use tracing::info;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     tracing_subscriber::fmt::init();
    ///     let config = Config::new("woo.toml")?;
    ///     let client = ApiClient::new(&config)?;
    ///     let deleted: Product = client.delete(12345).await?;
    ///     info!("Product {} deleted", deleted.name);
    ///     Ok(())
    /// }
    /// ```
    pub async fn delete<T: Entity>(&self, entity_id: i32) -> Result<T> {
        let uri = self
            .base_url
            .join(&T::endpoint())?
            .join(&entity_id.to_string())?;
        self.delete_request_with_tries(uri, 3).await
    }

    async fn delete_request_with_tries<T: Entity>(&self, uri: Url, tries: i32) -> Result<T> {
        for i in 1..tries {
            tracing::debug!("Connecting {uri}, try {i}");
            match self
                .client
                .delete(uri.clone())
                .basic_auth(self.ck(), Some(self.cs()))
                .query(&[("force", true)])
                .send()
                .await?
                .json::<T>()
                .await
            {
                Ok(r) => {
                    return Ok(r);
                }
                Err(e) => {
                    tracing::error!(
                        "Failed to connect to {uri} with error: {e}\n{} tries left",
                        tries - i
                    );
                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                    continue;
                }
            }
        }
        Err(anyhow!(
            "Error deleting entity with uri: {}",
            uri.to_string()
        ))
    }
    /// This API helps you to batch create multiple entities.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use anyhow::Result;
    /// use rust_woocommerce::{Product, ApiClient, Config, Attribute};
    /// use tracing::info;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     tracing_subscriber::fmt::init();
    ///     let config = Config::new("woo.toml")?;
    ///     let client = ApiClient::new(&config)?;
    ///     let attribute = Attribute::builder()
    ///         .name("Test Attribute")
    ///         .option("Best")
    ///         .visible()
    ///         .build();
    ///     let new_product = Product::builder()
    ///         .name("Test Product For Example")
    ///         .featured()
    ///         .short_description("The most professional description")
    ///         .sku("product for test 42")
    ///         .regular_price("6969")
    ///         .manage_stock()
    ///         .stock_quantity(42)
    ///         .weight("50")
    ///         .dimensions("4", "3", "2")
    ///         .shipping_class("large")
    ///         .images("https://cs14.pikabu.ru/post_img/2021/06/27/7/1624794514137159585.jpg")
    ///         .attribute(attribute)
    ///         .build();
    ///     let batch_create = vec![new_product];
    ///     let batch_created: Vec<Product> = client.batch_create(batch_create).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn batch_create<T: Entity, O: Serialize + Clone + Send + 'static>(
        &self,
        create_objects: Vec<O>,
    ) -> Result<Vec<T>> {
        let mut result = Vec::new();
        let mut set = JoinSet::new();
        let uri = self.base_url.join(&T::endpoint())?.join(BATCH)?;
        let batched = create_objects
            .chunks(100)
            .map(|c| BatchObject::builder().extend_create(c.to_vec()).build())
            .collect::<Vec<_>>();
        for batch in batched {
            let client = self.client();
            let ck = self.ck();
            let cs = Some(self.cs());
            let url = uri.clone();
            set.spawn(async move {
                client
                    .post(url)
                    .basic_auth(ck, cs)
                    .json(&batch)
                    .send()
                    .await?
                    .json::<BatchObject<T>>()
                    .await
            });
        }
        while let Some(Ok(Ok(v))) = set.join_next().await {
            result.extend(v.create)
        }
        Ok(result.into_iter().flatten().collect::<Vec<_>>())
    }
    /// This API helps you to batch update multiple entities.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use rust_woocommerce::{ApiClient, Config};
    /// use rust_woocommerce::Category;
    /// use anyhow::Result;
    /// use tracing::info;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     tracing_subscriber::fmt::init();
    ///     let config = Config::new("woo.toml")?;
    ///     let client = ApiClient::new(&config)?;
    ///     let batch_update = Category::update()
    ///         .id(12345)
    ///         .description("Some description")
    ///         .build();
    ///     let batch_updated: Vec<Category> = client.batch_update(vec![batch_update]).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn batch_update<T: Entity, O: Serialize + Clone + Send + 'static>(
        &self,
        update_objects: Vec<O>,
    ) -> Result<Vec<T>> {
        let mut result = Vec::new();
        let mut set = JoinSet::new();
        let uri = self.base_url.join(&T::endpoint())?.join(BATCH)?;
        let batched = update_objects
            .chunks(100)
            .map(|c| BatchObject::builder().extend_update(c.to_vec()).build())
            .collect::<Vec<_>>();
        for batch in batched {
            let client = self.client();
            let ck = self.ck();
            let cs = Some(self.cs());
            let url = uri.clone();
            set.spawn(async move {
                client
                    .post(url)
                    .basic_auth(ck, cs)
                    .json(&batch)
                    .send()
                    .await?
                    .json::<BatchObject<T>>()
                    .await
            });
        }
        while let Some(Ok(Ok(v))) = set.join_next().await {
            result.extend(v.update)
        }
        Ok(result.into_iter().flatten().collect::<Vec<_>>())
    }
    /// This API helps you to batch delete multiple entities.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use rust_woocommerce::{ApiClient, Config};
    /// use rust_woocommerce::Category;
    /// use anyhow::Result;
    /// use tracing::info;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     tracing_subscriber::fmt::init();
    ///     let config = Config::new("woo.toml")?;
    ///     let client = ApiClient::new(&config)?;
    ///     let batch_deleted: Vec<Category> = client.batch_delete(vec![12345]).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn batch_delete<T: Entity>(&self, delete_objects: Vec<i32>) -> Result<Vec<T>> {
        let mut result = Vec::new();
        let mut set = JoinSet::new();
        let uri = self.base_url.join(&T::endpoint())?.join(BATCH)?;
        let batched = delete_objects
            .chunks(100)
            .map(|c| BatchObject::builder().extend_delete(c.to_vec()).build())
            .collect::<Vec<_>>();
        for batch in batched {
            let client = self.client();
            let ck = self.ck();
            let cs = Some(self.cs());
            let url = uri.clone();
            set.spawn(async move {
                client
                    .post(url)
                    .basic_auth(ck, cs)
                    .query(&[("force", true)])
                    .json(&batch)
                    .send()
                    .await?
                    .json::<BatchObject<T>>()
                    .await
            });
        }
        while let Some(Ok(Ok(v))) = set.join_next().await {
            result.extend(v.delete)
        }
        Ok(result.into_iter().flatten().collect::<Vec<_>>())
    }
    /// This API lets you retrieve and view a specific subentity by ID.
    ///
    /// # Example
    /// ```no_run
    /// use anyhow::Result;
    /// use tracing::info;
    ///
    /// use rust_woocommerce::{ApiClient, Config, ProductVariation};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     tracing_subscriber::fmt::init();
    ///     let config = Config::new("woo.toml")?;
    ///     let client = ApiClient::new(&config)?;
    ///     let retrieved_variation: ProductVariation = client
    ///         .retrieve_subentity(12345, 42)
    ///         .await?;
    ///     info!("Retrieved variation has sku: {}", retrieved_variation.sku);
    ///     Ok(())
    /// }
    /// ```
    pub async fn retrieve_subentity<T: Entity>(
        &self,
        entity_id: i32,
        subentity_id: i32,
    ) -> Result<T> {
        let uri = self
            .base_url
            .join(&T::child_endpoint(entity_id))?
            .join(&subentity_id.to_string())?;
        self.get_request_with_tries(uri, 3).await
    }
    /// This API lets you view all subentities of entity.
    ///
    /// # Example
    /// ```no_run
    /// use anyhow::Result;
    /// use tracing::info;
    ///
    /// use rust_woocommerce::{ApiClient, Config, ProductVariation};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     tracing_subscriber::fmt::init();
    ///     let config = Config::new("woo.toml")?;
    ///     let client = ApiClient::new(&config)?;
    ///     let variations = client
    ///         .list_all_subentities::<ProductVariation>(12345)
    ///         .await?;
    ///     info!(
    ///         "Got {} variations for product with id: 12345",
    ///         variations.len()
    ///     );
    ///     Ok(())
    /// }
    /// ```
    pub async fn list_all_subentities<T: Entity>(&self, entity_id: i32) -> Result<Vec<T>> {
        let uri = self.base_url.join(&T::child_endpoint(entity_id))?;

        for i in 1..3 {
            tracing::debug!("Connecting {uri}, try {i}");
            match self
                .client
                .get(uri.clone())
                .basic_auth(self.ck(), Some(self.cs()))
                .send()
                .await?
                .json::<Vec<T>>()
                .await
            {
                Ok(r) => return Ok(r),
                Err(e) => {
                    tracing::error!(
                        "Failed to connect to {uri} with error: {e}\n{} tries left",
                        3 - i
                    );
                    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
                    continue;
                }
            }
        }
        Err(anyhow!(
            "Error retrieving subentitity for entity with id: {entity_id}"
        ))
    }
    /// This API helps you create a new subentity.
    ///
    /// # Example
    /// ```no_run
    /// use anyhow::Result;
    /// use tracing::info;
    ///
    /// use rust_woocommerce::{ApiClient, Config, ProductVariation};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     tracing_subscriber::fmt::init();
    ///     let config = Config::new("woo.toml")?;
    ///     let client = ApiClient::new(&config)?;
    ///     let variation = ProductVariation::builder()
    ///         .sku("Best variation")
    ///         .regular_price("6969")
    ///         .manage_stock()
    ///         .stock_quantity(96)
    ///         .weight("52")
    ///         .dimensions("5", "4", "3")
    ///         .attribute(None, "Test Attribute", "Best")
    ///         .build();
    ///     let created_variation: ProductVariation =
    ///         client.create_subentity(12345, variation).await?;
    ///     info!(
    ///         "Variation {} created with price: {}",
    ///         created_variation.sku, created_variation.price
    ///     );
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_subentity<T: Entity>(
        &self,
        entity_id: i32,
        object: impl Serialize,
    ) -> Result<T> {
        let uri = self.base_url.join(&T::child_endpoint(entity_id))?;
        self.post_request_with_tries(&object, uri, 3).await
    }
    /// This API lets you make changes to subentity.
    ///
    /// # Example
    /// ```no_run
    /// use anyhow::Result;
    /// use tracing::info;
    ///
    /// use rust_woocommerce::{ApiClient, Config, ProductVariation};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     tracing_subscriber::fmt::init();
    ///     let config = Config::new("woo.toml")?;
    ///     let client = ApiClient::new(&config)?;
    ///     let update = ProductVariation::builder().regular_price("7000").build();
    ///     let updated_variation: ProductVariation = client
    ///         .update_subentity(12345, 42, update)
    ///         .await?;
    ///     info!(
    ///         "Variation {} updated with price: {}",
    ///         updated_variation.sku, updated_variation.price
    ///     );
    ///     Ok(())
    /// }
    /// ```
    pub async fn update_subentity<T: Entity>(
        &self,
        entity_id: i32,
        subentity_id: i32,
        object: impl Serialize,
    ) -> Result<T> {
        let uri = self
            .base_url
            .join(&T::child_endpoint(entity_id))?
            .join(&subentity_id.to_string())?;
        self.put_request_with_tries(&object, uri, 3).await
    }
    /// This API helps you delete subentity.
    ///
    /// # Example
    /// ```no_run
    /// use anyhow::Result;
    /// use tracing::info;
    ///
    /// use rust_woocommerce::{ApiClient, Config, ProductVariation};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     tracing_subscriber::fmt::init();
    ///     let config = Config::new("woo.toml")?;
    ///     let client = ApiClient::new(&config)?;
    ///     let deleted_variation: ProductVariation = client
    ///         .delete_subentity(12345, 42)
    ///         .await?;
    ///     info!("Variation {} deleted", deleted_variation.sku);
    ///     Ok(())
    /// }
    /// ```
    pub async fn delete_subentity<T: Entity>(
        &self,
        entity_id: i32,
        subentity_id: i32,
    ) -> Result<T> {
        let uri = self
            .base_url
            .join(&T::child_endpoint(entity_id))?
            .join(&subentity_id.to_string())?;
        self.delete_request_with_tries(uri, 3).await
    }
    /// This API helps you to batch create subentities.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use anyhow::Result;
    /// use tracing::info;
    ///
    /// use rust_woocommerce::{ApiClient, Config, ProductVariation};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     tracing_subscriber::fmt::init();
    ///     let config = Config::new("woo.toml")?;
    ///     let client = ApiClient::new(&config)?;
    ///     let variation = ProductVariation::builder()
    ///         .sku("Best SKU")
    ///         .regular_price("6969")
    ///         .manage_stock()
    ///         .stock_quantity(96)
    ///         .weight("52")
    ///         .dimensions("5", "4", "3")
    ///         .attribute(None, "Test Attribute", "Best")
    ///         .build();
    ///     let batch_created_variation: Vec<ProductVariation> = client
    ///         .batch_create_subentity(12345, vec![variation])
    ///         .await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn batch_create_subentity<T: Entity, O: Serialize + Clone + Send + 'static>(
        &self,
        entity_id: i32,
        create_objects: Vec<O>,
    ) -> Result<Vec<T>> {
        let mut result = Vec::new();
        let mut set = JoinSet::new();
        let uri = self
            .base_url
            .join(&T::child_endpoint(entity_id))?
            .join(BATCH)?;
        let batched = create_objects
            .chunks(100)
            .map(|c| BatchObject::builder().extend_create(c.to_vec()).build())
            .collect::<Vec<_>>();
        for batch in batched {
            let client = self.client();
            let ck = self.ck();
            let cs = Some(self.cs());
            let url = uri.clone();
            set.spawn(async move {
                client
                    .post(url)
                    .basic_auth(ck, cs)
                    .json(&batch)
                    .send()
                    .await?
                    .json::<BatchObject<T>>()
                    .await
            });
        }
        while let Some(Ok(Ok(v))) = set.join_next().await {
            result.extend(v.create)
        }
        Ok(result.into_iter().flatten().collect::<Vec<_>>())
    }
    /// This API helps you to batch update subentities.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use anyhow::Result;
    /// use tracing::info;
    ///
    /// use rust_woocommerce::{ApiClient, Config, ProductVariation};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     tracing_subscriber::fmt::init();
    ///     let config = Config::new("woo.toml")?;
    ///     let client = ApiClient::new(&config)?;
    ///     let batch_update_variation = vec![ProductVariation::builder()
    ///         .id(42)
    ///         .regular_price("777")
    ///         .build()];
    ///     let batch_updated_variation: Vec<ProductVariation> = client
    ///         .batch_update_subentity(12345, batch_update_variation)
    ///         .await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn batch_update_subentity<T: Entity, O: Serialize + Clone + Send + 'static>(
        &self,
        entity_id: i32,
        update_objects: Vec<O>,
    ) -> Result<Vec<T>> {
        let mut result = Vec::new();
        let mut set = JoinSet::new();
        let uri = self
            .base_url
            .join(&T::child_endpoint(entity_id))?
            .join(BATCH)?;
        let batched = update_objects
            .chunks(100)
            .map(|c| BatchObject::builder().extend_update(c.to_vec()).build())
            .collect::<Vec<_>>();
        for batch in batched {
            let client = self.client();
            let ck = self.ck();
            let cs = Some(self.cs());
            let url = uri.clone();
            set.spawn(async move {
                client
                    .post(url)
                    .basic_auth(ck, cs)
                    .json(&batch)
                    .send()
                    .await?
                    .json::<BatchObject<T>>()
                    .await
            });
        }
        while let Some(Ok(Ok(v))) = set.join_next().await {
            result.extend(v.update)
        }
        Ok(result.into_iter().flatten().collect::<Vec<_>>())
    }
    /// This API helps you to batch delete subentities.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use anyhow::Result;
    /// use tracing::info;
    ///
    /// use rust_woocommerce::{ApiClient, Config, ProductVariation};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<()> {
    ///     tracing_subscriber::fmt::init();
    ///     let config = Config::new("woo.toml")?;
    ///     let client = ApiClient::new(&config)?;
    ///     let batch_deleted_variation: Vec<ProductVariation> = client
    ///         .batch_delete_subentity(12345, vec![42])
    ///         .await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn batch_delete_subentity<T: Entity, O: Serialize + Clone + Send + 'static>(
        &self,
        entity_id: i32,
        delete_objects: Vec<O>,
    ) -> Result<Vec<T>> {
        let mut result = Vec::new();
        let mut set = JoinSet::new();
        let uri = self
            .base_url
            .join(&T::child_endpoint(entity_id))?
            .join(BATCH)?;
        let batched = delete_objects
            .chunks(100)
            .map(|c| BatchObject::builder().extend_delete(c.to_vec()).build())
            .collect::<Vec<_>>();
        for batch in batched {
            let client = self.client();
            let ck = self.ck();
            let cs = Some(self.cs());
            let url = uri.clone();
            set.spawn(async move {
                client
                    .post(url)
                    .basic_auth(ck, cs)
                    .json(&batch)
                    .send()
                    .await?
                    .json::<BatchObject<T>>()
                    .await
            });
        }
        while let Some(Ok(Ok(v))) = set.join_next().await {
            result.extend(v.delete)
        }
        Ok(result.into_iter().flatten().collect::<Vec<_>>())
    }
}

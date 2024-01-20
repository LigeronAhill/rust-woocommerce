use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
#[derive(Default)]
pub struct WithName(String);
#[derive(Default)]
pub struct NoName;
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductTagCreate {
    /// Tag name.
    pub name: String,
    /// An alphanumeric identifier for the resource unique to its type.
    pub slug: Option<String>,
    /// HTML description of the resource.
    pub description: Option<String>,
}
#[derive(Default)]
pub struct ProductTagCreateBuilder<N> {
    /// Tag name.
    pub name: N,
    /// An alphanumeric identifier for the resource unique to its type.
    pub slug: Option<String>,
    /// HTML description of the resource.
    pub description: Option<String>,
}
impl<N> ProductTagCreateBuilder<N> {
    /// Tag name.
    pub fn name(self, name: impl Into<String>) -> ProductTagCreateBuilder<WithName> {
        ProductTagCreateBuilder {
            name: WithName(name.into()),
            slug: self.slug,
            description: self.description,
        }
    }
    /// An alphanumeric identifier for the resource unique to its type.
    pub fn slug(mut self, slug: impl Into<String>) -> Self {
        let _ = self.slug.insert(slug.into());
        self
    }
    /// HTML description of the resource.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        let _ = self.description.insert(description.into());
        self
    }
}
impl ProductTagCreateBuilder<WithName> {
    pub fn build(self) -> ProductTagCreate {
        ProductTagCreate {
            name: self.name.0,
            slug: self.slug,
            description: self.description,
        }
    }
}
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductTagUpdate {
    /// Unique identifier for the resource.
    pub id: Option<i32>,
    /// Tag name.
    pub name: Option<String>,
    /// An alphanumeric identifier for the resource unique to its type.
    pub slug: Option<String>,
    /// HTML description of the resource.
    pub description: Option<String>,
}
#[derive(Default)]
pub struct ProductTagUpdateBuilder {
    /// Unique identifier for the resource.
    pub id: Option<i32>,
    /// Tag name.
    pub name: Option<String>,
    /// An alphanumeric identifier for the resource unique to its type.
    pub slug: Option<String>,
    /// HTML description of the resource.
    pub description: Option<String>,
}
impl ProductTagUpdateBuilder {
    /// Unique identifier for the resource.
    pub fn id(mut self, id: i32) -> Self {
        let _ = self.id.insert(id);
        self
    }
    /// Tag name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        let _ = self.name.insert(name.into());
        self
    }
    /// An alphanumeric identifier for the resource unique to its type.
    pub fn slug(mut self, slug: impl Into<String>) -> Self {
        let _ = self.slug.insert(slug.into());
        self
    }
    /// HTML description of the resource.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        let _ = self.description.insert(description.into());
        self
    }
    pub fn build(self) -> ProductTagUpdate {
        ProductTagUpdate {
            id: self.id,
            name: self.name,
            slug: self.slug,
            description: self.description,
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::{models::product_tags::ProductTag, ApiClient, BatchObject, Entity};

    #[tokio::test]
    async fn test_list_all_product_tags() {
        let client = ApiClient::from_env().unwrap();
        let result = client
            .list_all::<ProductTag>(Entity::ProductTag)
            .await
            .unwrap();
        assert!(!result.is_empty());
    }
    #[tokio::test]
    async fn test_retrieve_product_tag() {
        let client = ApiClient::from_env().unwrap();
        let result = client
            .retrieve::<ProductTag>(Entity::ProductTag, 495)
            .await
            .unwrap();
        assert_eq!("test-tag", result.slug);
    }
    #[tokio::test]
    async fn test_create_delete_product_tag() {
        let client = ApiClient::from_env().unwrap();
        let create = ProductTag::create().name("test tag 2").build();
        let created = client
            .create::<ProductTag>(Entity::ProductTag, create)
            .await
            .unwrap();
        assert_eq!(created.name, "test tag 2");
        let _deleted = client
            .delete::<ProductTag>(Entity::ProductTag, created.id)
            .await
            .unwrap();
    }
    #[tokio::test]
    async fn test_update_product_tag() {
        let client = ApiClient::from_env().unwrap();
        let update = ProductTag::update().description("test description").build();
        let updated: ProductTag = client
            .update(Entity::ProductTag, 495, update)
            .await
            .unwrap();
        assert_eq!(updated.description, "test description");
    }
    #[tokio::test]
    async fn test_batch_update_product_tag() {
        let client = ApiClient::from_env().unwrap();
        let update = ProductTag::update().id(495).description("").build();
        let batch = BatchObject::builder().add_update(update).build();
        let updated: BatchObject<ProductTag> = client
            .batch_update(Entity::ProductTag, batch)
            .await
            .unwrap();
        assert!(updated.update.is_some());
    }
}

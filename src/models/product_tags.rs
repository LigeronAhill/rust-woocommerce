use serde::{Deserialize, Serialize};

use crate::controllers::product_tags::{NoName, ProductTagCreateBuilder, ProductTagUpdateBuilder};
/// ```
/// #[cfg(test)]
/// mod tests {
///     use crate::{product_tags::ProductTag, ApiClient, BatchObject, Entity};
///
///     #[tokio::test]
///     async fn test_list_all_product_tags() {
///         let client = ApiClient::from_env().unwrap();
///         let result = client
///             .list_all::<ProductTag>(Entity::ProductTag)
///             .await
///             .unwrap();
///         assert!(!result.is_empty());
///     }
///     #[tokio::test]
///     async fn test_retrieve_product_tag() {
///         let client = ApiClient::from_env().unwrap();
///         let result = client
///             .retrieve::<ProductTag>(Entity::ProductTag, 495)
///             .await
///             .unwrap();
///         assert_eq!("test-tag", result.slug);
///     }
///     #[tokio::test]
///     async fn test_create_delete_product_tag() {
///         let client = ApiClient::from_env().unwrap();
///         let create = ProductTag::create().name("test tag 2").build();
///         let created = client
///             .create::<ProductTag>(Entity::ProductTag, create)
///             .await
///             .unwrap();
///         assert_eq!(created.name, "test tag 2");
///         let _deleted = client
///             .delete::<ProductTag>(Entity::ProductTag, created.id)
///             .await
///             .unwrap();
///     }
///     #[tokio::test]
///     async fn test_update_product_tag() {
///         let client = ApiClient::from_env().unwrap();
///         let update = ProductTag::update().description("test description").build();
///         let updated: ProductTag = client
///             .update(Entity::ProductTag, 495, update)
///             .await
///             .unwrap();
///         assert_eq!(updated.description, "test description");
///     }
///     #[tokio::test]
///     async fn test_batch_update_product_tag() {
///         let client = ApiClient::from_env().unwrap();
///         let update = ProductTag::update().id(495).description("").build();
///         let batch = BatchObject::builder().add_update(update).build();
///         let updated: BatchObject<ProductTag> = client
///             .batch_update(Entity::ProductTag, batch)
///             .await
///             .unwrap();
///         assert!(updated.update.is_some());
///     }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductTag {
    /// Unique identifier for the resource.
    pub id: i32,
    /// Tag name.
    pub name: String,
    /// An alphanumeric identifier for the resource unique to its type.
    pub slug: String,
    /// HTML description of the resource.
    pub description: String,
    /// Number of published products for the resource.
    pub count: i32,
}
impl ProductTag {
    pub fn create() -> ProductTagCreateBuilder<NoName> {
        ProductTagCreateBuilder::default()
    }
    pub fn update() -> ProductTagUpdateBuilder {
        ProductTagUpdateBuilder::default()
    }
}

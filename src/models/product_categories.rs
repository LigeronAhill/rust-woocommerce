use serde::{Deserialize, Serialize};

use crate::controllers::product_categories::{
    CategoryCreateBuilder, CategoryUpdateBuilder, NoName,
};

use super::products::ProductImage;
/// ```rust
/// #[cfg(test)]
/// mod tests {
///     use crate::{
///         product_categories::{Category, DisplayOption},
///         ApiClient, BatchObject, Entity,
///     };
///
///     #[tokio::test]
///     async fn test_list_all_categories() {
///         let client = ApiClient::from_env().unwrap();
///         let result = client
///             .list_all::<Category>(Entity::ProductCategory)
///             .await
///             .unwrap();
///         assert!(!result.is_empty());
///     }
///     #[tokio::test]
///     async fn test_retrieve_category() {
///         let client = ApiClient::from_env().unwrap();
///         let result = client
///             .retrieve::<Category>(Entity::ProductCategory, 186)
///             .await
///             .unwrap();
///         assert_eq!("2640", result.name);
///     }
///     #[tokio::test]
///     async fn test_create_category() {
///         let client = ApiClient::from_env().unwrap();
///         let create = Category::create().name("Test Category").build();
///         let created: Category = client
///             .create(Entity::ProductCategory, create)
///             .await
///             .unwrap();
///         assert_eq!(created.name, "Test Category");
///         let _deleted: Category = client
///             .delete(Entity::ProductCategory, created.id)
///             .await
///             .unwrap();
///     }
///     #[tokio::test]
///     async fn test_update_category() {
///         let client = ApiClient::from_env().unwrap();
///         let update = Category::update().display(DisplayOption::Products).build();
///         let updated: Category = client
///             .update(Entity::ProductCategory, 330, update)
///             .await
///             .unwrap();
///         assert_eq!(updated.display, DisplayOption::Products)
///     }
///     #[tokio::test]
///     async fn test_batch_update_category() {
///         let client = ApiClient::from_env().unwrap();
///         let update = Category::update()
///             .id(330)
///             .display(DisplayOption::Products)
///             .build();
///         let batch = BatchObject::builder().add_update(update).build();
///         let updated: BatchObject<Category> = client
///             .batch_update(Entity::ProductCategory, batch)
///             .await
///             .unwrap();
///         assert!(updated.update.is_some());
///     }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    /// Unique identifier for the resource.
    pub id: i32,
    /// Category name.
    pub name: String,
    /// An alphanumeric identifier for the resource unique to its type.
    pub slug: String,
    /// The ID for the parent of the resource.
    pub parent: i32,
    /// HTML description of the resource.
    pub description: String,
    /// Category archive display type. Options: default, products, subcategories and both. Default is default.
    pub display: DisplayOption,
    /// Image data.
    pub image: Option<ProductImage>,
    /// Menu order, used to custom sort the resource.
    pub menu_order: i32,
    /// Number of published products for the resource.READ-ONLY
    pub count: i32,
}
impl Category {
    pub fn create() -> CategoryCreateBuilder<NoName> {
        CategoryCreateBuilder::default()
    }
    pub fn update() -> CategoryUpdateBuilder {
        CategoryUpdateBuilder::default()
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DisplayOption {
    #[default]
    Default,
    Products,
    Subcategories,
    Both,
}

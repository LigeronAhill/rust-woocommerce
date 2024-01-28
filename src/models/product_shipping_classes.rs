use serde::{Deserialize, Serialize};

use crate::controllers::product_shipping_classes::{
    NoName, ShippingClassCreateBuilder, ShippingClassUpdateBuilder,
};
/// ```rust
/// #[cfg(test)]
/// mod tests {
///     use crate::{product_shipping_classes::ShippingClass, ApiClient, BatchObject, Entity};
///
///     #[tokio::test]
///     async fn test_list_all_shipping_classes() {
///         let client = ApiClient::from_env().unwrap();
///         let result = client
///             .list_all::<ShippingClass>(Entity::ProductShippingClass)
///             .await
///             .unwrap();
///         assert!(!result.is_empty());
///     }
///     #[tokio::test]
///     async fn test_retrieve_shipping_class() {
///         let client = ApiClient::from_env().unwrap();
///         let result = client
///             .retrieve::<ShippingClass>(Entity::ProductShippingClass, 30)
///             .await
///             .unwrap();
///         assert_eq!("tiles", result.slug);
///     }
///     #[tokio::test]
///     async fn test_create_shipping_class() {
///         let client = ApiClient::from_env().unwrap();
///         let create = ShippingClass::create().name("test class").build();
///         let created = client
///             .create::<ShippingClass>(Entity::ProductShippingClass, create)
///             .await
///             .unwrap();
///         assert_eq!(created.name, "test class");
///         let _deleted = client
///             .delete::<ShippingClass>(Entity::ProductShippingClass, created.id)
///             .await
///             .unwrap();
///     }
///     #[tokio::test]
///     async fn test_update_shipping_class() {
///         let client = ApiClient::from_env().unwrap();
///         let update = ShippingClass::update()
///             .description("test description")
///             .build();
///         let updated: ShippingClass = client
///             .update(Entity::ProductShippingClass, 30, update)
///             .await
///             .unwrap();
///         assert_eq!(updated.description, "test description");
///         let update = ShippingClass::update().description("").build();
///         let _updated: ShippingClass = client
///             .update(Entity::ProductShippingClass, 30, update)
///             .await
///             .unwrap();
///     }
///     #[tokio::test]
///     async fn test_batch_update_shipping_class() {
///         let client = ApiClient::from_env().unwrap();
///         let update = ShippingClass::update().id(30).description("").build();
///         let batch = BatchObject::builder().add_update(update).build();
///         let updated: BatchObject<ShippingClass> = client
///             .batch_update(Entity::ProductShippingClass, batch)
///             .await
///             .unwrap();
///         assert!(updated.update.is_some());
///     }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingClass {
    /// Unique identifier for the resource.
    pub id: i32,
    /// Shipping class name.
    pub name: String,
    /// An alphanumeric identifier for the resource unique to its type.
    pub slug: String,
    /// HTML description of the resource.
    pub description: String,
    /// Number of published products for the resource.
    pub count: i32,
}
impl ShippingClass {
    pub fn create() -> ShippingClassCreateBuilder<NoName> {
        ShippingClassCreateBuilder::default()
    }
    pub fn update() -> ShippingClassUpdateBuilder {
        ShippingClassUpdateBuilder::default()
    }
}

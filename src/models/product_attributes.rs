use serde::{Deserialize, Serialize};

use crate::controllers::product_attributes::{
    AttributeCreateBuilder, AttributeUpdateBuilder, NoName,
};
/// ```rust
/// #[cfg(test)]
/// mod tests {
///     use crate::{
///         product_attributes::{Attribute, AttributeSortOrder},
///         ApiClient, BatchObject, Entity,
///     };
///
///     #[tokio::test]
///     async fn test_list_all_attributes() {
///         let client = ApiClient::from_env().unwrap();
///         let result = client
///             .list_all::<Attribute>(Entity::ProductAttribute)
///             .await
///             .unwrap();
///         assert!(!result.is_empty());
///     }
///     #[tokio::test]
///     async fn test_retrieve_attribute() {
///         let client = ApiClient::from_env().unwrap();
///         let result = client
///             .retrieve::<Attribute>(Entity::ProductAttribute, 4)
///             .await
///             .unwrap();
///         assert_eq!("Цвет", result.name);
///     }
///     #[tokio::test]
///     async fn test_create_attribute() {
///         let client = ApiClient::from_env().unwrap();
///         let create = Attribute::create()
///             .name("test attribute")
///             .enable_archives()
///             .build();
///         let created = client
///             .create::<Attribute>(Entity::ProductAttribute, create)
///             .await
///             .unwrap();
///         assert_eq!(created.name, "test attribute");
///         let _deleted: Attribute = client
///             .delete(Entity::ProductAttribute, created.id)
///             .await
///             .unwrap();
///     }
///     #[tokio::test]
///     async fn test_update_attribute() {
///         let client = ApiClient::from_env().unwrap();
///         let update = Attribute::update()
///             .order_by(AttributeSortOrder::Name)
///             .build();
///         let updated = client
///             .update::<Attribute>(Entity::ProductAttribute, 2, update)
///             .await
///             .unwrap();
///         assert_eq!(updated.order_by, AttributeSortOrder::Name)
///     }
///     #[tokio::test]
///     async fn test_batch_update_attribute() {
///         let client = ApiClient::from_env().unwrap();
///         let update = Attribute::update()
///             .id(2)
///             .order_by(AttributeSortOrder::Name)
///             .build();
///         let batch = BatchObject::builder().add_update(update).build();
///         let updated: BatchObject<Attribute> = client
///             .batch_update(Entity::ProductAttribute, batch)
///             .await
///             .unwrap();
///         assert!(updated.update.is_some());
///     }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    /// Unique identifier for the resource.
    pub id: i32,
    /// Attribute name.
    pub name: String,
    /// An alphanumeric identifier for the resource unique to its type.
    pub slug: String,
    /// Type of attribute. By default only select is supported.
    #[serde(rename = "type")]
    pub attribute_type: AttributeType,
    /// Default sort order. Options: menu_order, name, name_num and id. Default is menu_order.
    pub order_by: AttributeSortOrder,
    /// Enable/Disable attribute archives. Default is false.
    pub has_archives: bool,
}
impl Attribute {
    pub fn create() -> AttributeCreateBuilder<NoName> {
        AttributeCreateBuilder::default()
    }
    pub fn update() -> AttributeUpdateBuilder {
        AttributeUpdateBuilder::default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum AttributeType {
    #[default]
    Select,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AttributeSortOrder {
    #[default]
    MenuOrder,
    Name,
    NameNum,
    Id,
}

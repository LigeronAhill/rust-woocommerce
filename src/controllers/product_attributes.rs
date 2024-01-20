use serde_with::skip_serializing_none;

use serde::{Deserialize, Serialize};

use crate::models::product_attributes::{AttributeSortOrder, AttributeType};
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributeCreate {
    /// Attribute name.
    pub name: String,
    /// An alphanumeric identifier for the resource unique to its type.
    pub slug: Option<String>,
    /// Type of attribute. By default only select is supported.
    #[serde(rename = "type")]
    pub attribute_type: Option<AttributeType>,
    /// Default sort order. Options: menu_order, name, name_num and id. Default is menu_order.
    pub order_by: Option<AttributeSortOrder>,
    /// Enable/Disable attribute archives. Default is false.
    pub has_archives: Option<bool>,
}
#[derive(Default)]
pub struct AttributeCreateBuilder<N> {
    /// Attribute name.
    pub name: N,
    /// An alphanumeric identifier for the resource unique to its type.
    pub slug: Option<String>,
    /// Type of attribute. By default only select is supported.
    pub attribute_type: Option<AttributeType>,
    /// Default sort order. Options: menu_order, name, name_num and id. Default is menu_order.
    pub order_by: Option<AttributeSortOrder>,
    /// Enable/Disable attribute archives. Default is false.
    pub has_archives: Option<bool>,
}
#[derive(Default)]
pub struct WithName(String);
#[derive(Default)]
pub struct NoName;
impl<N> AttributeCreateBuilder<N> {
    /// Attribute name.
    pub fn name(self, name: impl Into<String>) -> AttributeCreateBuilder<WithName> {
        AttributeCreateBuilder {
            name: WithName(name.into()),
            slug: self.slug,
            attribute_type: self.attribute_type,
            order_by: self.order_by,
            has_archives: self.has_archives,
        }
    }
    /// An alphanumeric identifier for the resource unique to its type.
    pub fn slug(mut self, slug: impl Into<String>) -> Self {
        let _ = self.slug.insert(slug.into());
        self
    }
    /// Type of attribute. By default only select is supported.
    pub fn attribute_type(mut self, attribute_type: AttributeType) -> Self {
        let _ = self.attribute_type.insert(attribute_type);
        self
    }
    /// Default sort order. Options: menu_order, name, name_num and id. Default is menu_order.
    pub fn order_by(mut self, order_by: AttributeSortOrder) -> Self {
        let _ = self.order_by.insert(order_by);
        self
    }
    /// Enable/Disable attribute archives. Default is false.
    pub fn enable_archives(mut self) -> Self {
        let _ = self.has_archives.insert(true);
        self
    }
}
impl AttributeCreateBuilder<WithName> {
    pub fn build(self) -> AttributeCreate {
        AttributeCreate {
            name: self.name.0,
            slug: self.slug,
            attribute_type: self.attribute_type,
            order_by: self.order_by,
            has_archives: self.has_archives,
        }
    }
}
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributeUpdate {
    /// Unique identifier for the resource.
    pub id: Option<i32>,
    /// Attribute name.
    pub name: Option<String>,
    /// An alphanumeric identifier for the resource unique to its type.
    pub slug: Option<String>,
    /// Type of attribute. By default only select is supported.
    #[serde(rename = "type")]
    pub attribute_type: Option<AttributeType>,
    /// Default sort order. Options: menu_order, name, name_num and id. Default is menu_order.
    pub order_by: Option<AttributeSortOrder>,
    /// Enable/Disable attribute archives. Default is false.
    pub has_archives: Option<bool>,
}
#[derive(Default)]
pub struct AttributeUpdateBuilder {
    /// Unique identifier for the resource.
    pub id: Option<i32>,
    /// Attribute name.
    pub name: Option<String>,
    /// An alphanumeric identifier for the resource unique to its type.
    pub slug: Option<String>,
    /// Type of attribute. By default only select is supported.
    pub attribute_type: Option<AttributeType>,
    /// Default sort order. Options: menu_order, name, name_num and id. Default is menu_order.
    pub order_by: Option<AttributeSortOrder>,
    /// Enable/Disable attribute archives. Default is false.
    pub has_archives: Option<bool>,
}
impl AttributeUpdateBuilder {
    /// Unique identifier for the resource.
    pub fn id(&mut self, id: i32) -> &mut Self {
        let _ = self.id.insert(id);
        self
    }
    /// Attribute name.
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        let _ = self.name.insert(name.into());
        self
    }
    /// An alphanumeric identifier for the resource unique to its type.
    pub fn slug(&mut self, slug: impl Into<String>) -> &mut Self {
        let _ = self.slug.insert(slug.into());
        self
    }
    /// Type of attribute. By default only select is supported.
    pub fn attribute_type(&mut self, attribute_type: AttributeType) -> &mut Self {
        let _ = self.attribute_type.insert(attribute_type);
        self
    }
    /// Default sort order. Options: menu_order, name, name_num and id. Default is menu_order.
    pub fn order_by(&mut self, order_by: AttributeSortOrder) -> &mut Self {
        let _ = self.order_by.insert(order_by);
        self
    }
    /// Enable attribute archives.
    pub fn enable_archives(&mut self) -> &mut Self {
        let _ = self.has_archives.insert(true);
        self
    }
    /// Disable attribute archives.
    pub fn disable_archives(&mut self) -> &mut Self {
        let _ = self.has_archives.insert(false);
        self
    }
    pub fn build(&self) -> AttributeUpdate {
        AttributeUpdate {
            id: self.id,
            name: self.name.to_owned(),
            slug: self.slug.to_owned(),
            attribute_type: self.attribute_type.to_owned(),
            order_by: self.order_by.to_owned(),
            has_archives: self.has_archives,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        controllers::entities::Entity,
        models::product_attributes::{Attribute, AttributeSortOrder},
        ApiClient, BatchObject,
    };

    #[tokio::test]
    async fn test_list_all_attributes() {
        let client = ApiClient::from_env().unwrap();
        let result = client
            .list_all::<Attribute>(Entity::ProductAttribute)
            .await
            .unwrap();
        assert!(!result.is_empty());
    }
    #[tokio::test]
    async fn test_retrieve_attribute() {
        let client = ApiClient::from_env().unwrap();
        let result = client
            .retrieve::<Attribute>(Entity::ProductAttribute, 4)
            .await
            .unwrap();
        assert_eq!("Цвет", result.name);
    }
    #[tokio::test]
    async fn test_create_attribute() {
        let client = ApiClient::from_env().unwrap();
        let create = Attribute::create()
            .name("test attribute")
            .enable_archives()
            .build();
        let created = client
            .create::<Attribute>(Entity::ProductAttribute, create)
            .await
            .unwrap();
        assert_eq!(created.name, "test attribute");
        let _deleted: Attribute = client
            .delete(Entity::ProductAttribute, created.id)
            .await
            .unwrap();
    }
    #[tokio::test]
    async fn test_update_attribute() {
        let client = ApiClient::from_env().unwrap();
        let update = Attribute::update()
            .order_by(AttributeSortOrder::Name)
            .build();
        let updated = client
            .update::<Attribute>(Entity::ProductAttribute, 2, update)
            .await
            .unwrap();
        assert_eq!(updated.order_by, AttributeSortOrder::Name)
    }
    #[tokio::test]
    async fn test_batch_update_attribute() {
        let client = ApiClient::from_env().unwrap();
        let update = Attribute::update()
            .id(2)
            .order_by(AttributeSortOrder::Name)
            .build();
        let batch = BatchObject::builder().add_update(update).build();
        let updated: BatchObject<Attribute> = client
            .batch_update(Entity::ProductAttribute, batch)
            .await
            .unwrap();
        assert!(updated.update.is_some());
    }
}

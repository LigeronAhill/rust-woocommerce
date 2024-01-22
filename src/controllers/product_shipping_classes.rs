use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingClassCreate {
    name: String,
    slug: Option<String>,
    description: Option<String>,
}
#[derive(Default)]
pub struct WithName(String);
#[derive(Default)]
pub struct NoName;
#[derive(Default)]
pub struct ShippingClassCreateBuilder<N> {
    name: N,
    slug: Option<String>,
    description: Option<String>,
}
impl<N> ShippingClassCreateBuilder<N> {
    /// Shipping class name.
    pub fn name(self, name: impl Into<String>) -> ShippingClassCreateBuilder<WithName> {
        ShippingClassCreateBuilder {
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
impl ShippingClassCreateBuilder<WithName> {
    pub fn build(self) -> ShippingClassCreate {
        ShippingClassCreate {
            name: self.name.0,
            slug: self.slug,
            description: self.description,
        }
    }
}
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingClassUpdate {
    id: Option<i32>,
    name: Option<String>,
    slug: Option<String>,
    description: Option<String>,
}
#[derive(Default)]
pub struct ShippingClassUpdateBuilder {
    id: Option<i32>,
    name: Option<String>,
    slug: Option<String>,
    description: Option<String>,
}
impl ShippingClassUpdateBuilder {
    /// Unique identifier for the resource.
    pub fn id(mut self, id: i32) -> Self {
        let _ = self.id.insert(id);
        self
    }
    /// Shipping class name.
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
    pub fn build(self) -> ShippingClassUpdate {
        ShippingClassUpdate {
            id: self.id,
            name: self.name,
            slug: self.slug,
            description: self.description,
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::{product_shipping_classes::ShippingClass, ApiClient, BatchObject, Entity};

    #[tokio::test]
    async fn test_list_all_shipping_classes() {
        let client = ApiClient::from_env().unwrap();
        let result = client
            .list_all::<ShippingClass>(Entity::ProductShippingClass)
            .await
            .unwrap();
        assert!(!result.is_empty());
    }
    #[tokio::test]
    async fn test_retrieve_shipping_class() {
        let client = ApiClient::from_env().unwrap();
        let result = client
            .retrieve::<ShippingClass>(Entity::ProductShippingClass, 30)
            .await
            .unwrap();
        assert_eq!("tiles", result.slug);
    }
    #[tokio::test]
    async fn test_create_shipping_class() {
        let client = ApiClient::from_env().unwrap();
        let create = ShippingClass::create().name("test class").build();
        let created = client
            .create::<ShippingClass>(Entity::ProductShippingClass, create)
            .await
            .unwrap();
        assert_eq!(created.name, "test class");
        let _deleted = client
            .delete::<ShippingClass>(Entity::ProductShippingClass, created.id)
            .await
            .unwrap();
    }
    #[tokio::test]
    async fn test_update_shipping_class() {
        let client = ApiClient::from_env().unwrap();
        let update = ShippingClass::update()
            .description("test description")
            .build();
        let updated: ShippingClass = client
            .update(Entity::ProductShippingClass, 30, update)
            .await
            .unwrap();
        assert_eq!(updated.description, "test description");
        let update = ShippingClass::update().description("").build();
        let _updated: ShippingClass = client
            .update(Entity::ProductShippingClass, 30, update)
            .await
            .unwrap();
    }
    #[tokio::test]
    async fn test_batch_update_shipping_class() {
        let client = ApiClient::from_env().unwrap();
        let update = ShippingClass::update().id(30).description("").build();
        let batch = BatchObject::builder().add_update(update).build();
        let updated: BatchObject<ShippingClass> = client
            .batch_update(Entity::ProductShippingClass, batch)
            .await
            .unwrap();
        assert!(updated.update.is_some());
    }
}

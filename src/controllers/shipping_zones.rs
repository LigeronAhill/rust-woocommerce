use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingZoneCreate {
    name: String,
    order: i32,
}
impl ShippingZoneCreate {
    /// Shipping zone name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            order: 0,
        }
    }
    /// Shipping zone order.
    pub fn order(mut self, order: i32) -> Self {
        self.order = order;
        self
    }
}
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ShippingZoneUpdate {
    id: Option<i32>,
    name: Option<String>,
    order: Option<i32>,
}
impl ShippingZoneUpdate {
    /// Unique identifier for the resource.
    pub fn id(mut self, id: i32) -> Self {
        let _ = self.id.insert(id);
        self
    }
    /// Shipping zone name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        let _ = self.name.insert(name.into());
        self
    }
    /// Shipping zone order.
    pub fn order(mut self, order: i32) -> Self {
        let _ = self.order.insert(order);
        self
    }
}
#[cfg(test)]
mod tests {
    use crate::{shipping_zones::ShippingZone, ApiClient, Entity};

    #[tokio::test]
    async fn test_list_all_retrieve_shipping_zone() {
        let client = ApiClient::from_env().unwrap();
        let z = client
            .list_all::<ShippingZone>(Entity::ShippingZone)
            .await
            .unwrap();
        assert!(!z.is_empty());
        let first = z.first().unwrap();
        let z1: ShippingZone = client
            .retrieve(Entity::ShippingZone, first.id)
            .await
            .unwrap();
        assert_eq!(z1.name, first.name);
    }
    #[tokio::test]
    async fn test_create_update_delete_shipping_zone() {
        let client = ApiClient::from_env().unwrap();
        let create = ShippingZone::create("test sz").order(0);
        let created: ShippingZone = client.create(Entity::ShippingZone, create).await.unwrap();
        assert_eq!(created.name, "test sz");
        let update = ShippingZone::update().order(1);
        let updated: ShippingZone = client
            .update(Entity::ShippingZone, created.id, update)
            .await
            .unwrap();
        assert_eq!(updated.order, 1);
        let _deleted: ShippingZone = client
            .delete(Entity::ShippingZone, updated.id)
            .await
            .unwrap();
    }
}

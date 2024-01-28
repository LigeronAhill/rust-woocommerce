use serde::{Deserialize, Serialize};

use crate::controllers::shipping_zones::{ShippingZoneCreate, ShippingZoneUpdate};
/// ```rust
/// #[cfg(test)]
/// mod tests {
///     use crate::{shipping_zones::ShippingZone, ApiClient, Entity};
///
///     #[tokio::test]
///     async fn test_list_all_retrieve_shipping_zone() {
///         let client = ApiClient::from_env().unwrap();
///         let z = client
///             .list_all::<ShippingZone>(Entity::ShippingZone)
///             .await
///             .unwrap();
///         assert!(!z.is_empty());
///         let first = z.first().unwrap();
///         let z1: ShippingZone = client
///             .retrieve(Entity::ShippingZone, first.id)
///             .await
///             .unwrap();
///         assert_eq!(z1.name, first.name);
///     }
///     #[tokio::test]
///     async fn test_create_update_delete_shipping_zone() {
///         let client = ApiClient::from_env().unwrap();
///         let create = ShippingZone::create("test sz").order(0);
///         let created: ShippingZone = client.create(Entity::ShippingZone, create).await.unwrap();
///         assert_eq!(created.name, "test sz");
///         let update = ShippingZone::update().order(1);
///         let updated: ShippingZone = client
///             .update(Entity::ShippingZone, created.id, update)///
///             .await
///             .unwrap();
///         assert_eq!(updated.order, 1);
///         let _deleted: ShippingZone = client
///             .delete(Entity::ShippingZone, updated.id)
///             .await
///             .unwrap();
///     }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingZone {
    /// Unique identifier for the resource.
    pub id: i32,
    /// Shipping zone name.
    pub name: String,
    /// Shipping zone order.
    pub order: i32,
}
impl ShippingZone {
    pub fn create(name: impl Into<String>) -> ShippingZoneCreate {
        ShippingZoneCreate::new(name)
    }
    pub fn update() -> ShippingZoneUpdate {
        ShippingZoneUpdate::default()
    }
}

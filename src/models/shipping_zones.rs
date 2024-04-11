use crate::controllers::Entity;
use serde::{Deserialize, Serialize};

use crate::controllers::shipping_zones::{ShippingZoneCreate, ShippingZoneUpdate};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingZone {
    /// Unique identifier for the resource.
    pub id: i32,
    /// Shipping zone name.
    pub name: String,
    /// Shipping zone order.
    pub order: i32,
}
impl Entity for ShippingZone {
    fn endpoint() -> String {
        String::from("shipping/zones/")
    }

    fn child_endpoint(parent_id: i32) -> String {
        let _ = parent_id;
        String::new()
    }
}
impl ShippingZone {
    pub fn create(name: impl Into<String>) -> ShippingZoneCreate {
        ShippingZoneCreate::new(name)
    }
    pub fn update() -> ShippingZoneUpdate {
        ShippingZoneUpdate::default()
    }
}

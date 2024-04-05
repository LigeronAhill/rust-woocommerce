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
impl ShippingZone {
    pub fn create(name: impl Into<String>) -> ShippingZoneCreate {
        ShippingZoneCreate::new(name)
    }
    pub fn update() -> ShippingZoneUpdate {
        ShippingZoneUpdate::default()
    }
}

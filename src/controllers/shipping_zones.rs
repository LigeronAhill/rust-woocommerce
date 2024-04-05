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

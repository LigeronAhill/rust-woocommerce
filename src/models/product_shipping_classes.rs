use serde::{Deserialize, Serialize};

use crate::controllers::product_shipping_classes::{
    NoName, ShippingClassCreateBuilder, ShippingClassUpdateBuilder,
};
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

use crate::controllers::Entity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingMethod {
    /// Method ID.
    pub id: String,
    /// Shipping method title.
    pub title: String,
    /// Shipping method description.
    pub description: String,
}
impl Entity for ShippingMethod {
    fn endpoint() -> String {
        String::from("shipping_methods/")
    }

    fn child_endpoint(parent_id: i32) -> String {
        let _ = parent_id;
        String::new()
    }
}

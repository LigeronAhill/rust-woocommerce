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

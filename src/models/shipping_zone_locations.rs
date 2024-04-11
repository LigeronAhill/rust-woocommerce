use crate::controllers::Entity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingZoneLocation {
    /// Shipping zone location code.
    pub code: String,
    /// Shipping zone location type. Options: postcode, state, country and continent. Default is country.
    #[serde(rename = "type")]
    pub location_type: LocationType,
}
impl Entity for ShippingZoneLocation {
    fn endpoint() -> String {
        String::new()
    }

    fn child_endpoint(parent_id: i32) -> String {
        format!("shipping/zones/{parent_id}/locations/")
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LocationType {
    Postcode,
    State,
    Country,
    Continent,
}

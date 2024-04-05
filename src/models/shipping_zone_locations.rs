use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingZoneLocation {
    /// Shipping zone location code.
    pub code: String,
    /// Shipping zone location type. Options: postcode, state, country and continent. Default is country.
    #[serde(rename = "type")]
    pub location_type: LocationType,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LocationType {
    Postcode,
    State,
    Country,
    Continent,
}

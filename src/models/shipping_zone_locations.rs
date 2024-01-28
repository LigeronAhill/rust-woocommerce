use serde::{Deserialize, Serialize};
/// ```rust
/// #[cfg(test)]
/// mod tests {
///     use crate::{shipping_zone_locations::ShippingZoneLocation, ApiClient, Entity};
///
///     #[tokio::test]
///     async fn test_list_all_shipping_zone_locations() {
///         let client = ApiClient::from_env().unwrap();
///         let result = client
///             .list_all_subentities::<ShippingZoneLocation>(
///                 Entity::ShippingZone,
///                 1,
///                 crate::SubEntity::ShippingZoneLocation,
///             )
///             .await
///             .unwrap();
///         assert!(!result.is_empty());
///     }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingZoneLocation {
    /// Shipping zone location code.
    pub code: String,
    /// Shipping zone location type. Options: postcode, state, country and continent. Default is country.
    #[serde(rename = "type")]
    pub location_type: LocationType,
}
// impl ShippingZoneLocation {
//     pub fn update(code: impl Into<String>, location_type: LocationType) -> Self {
//         Self {
//             code: code.into(),
//             location_type,
//         }
//     }
// }
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LocationType {
    Postcode,
    State,
    Country,
    Continent,
}

use serde::{Deserialize, Serialize};
/// ```rust
/// #[cfg(test)]
/// mod tests {
///     use crate::{shipping_zone_methods::ShippingZoneMethod, ApiClient, Entity, SubEntity};
///
///     #[tokio::test]
///     async fn test_list_all_retrieve_shipping_zone_methods() {
///         let client = ApiClient::from_env().unwrap();
///         let result = client
///             .list_all_subentities::<ShippingZoneMethod>(
///                 Entity::ShippingZone,
///                 1,
///                 SubEntity::ShippingZoneMethod,
///             )
///             .await
///             .unwrap();
///         assert!(!result.is_empty());
///         let first = result.first().unwrap();
///         let retrieve_result = client
///             .retrieve_subentity::<ShippingZoneMethod>(
///                 Entity::ShippingZone,
///                 1,
///                 SubEntity::ShippingZoneMethod,
///                 first.instance_id,
///             )
///             .await
///             .unwrap();
///         assert_eq!(first.title, retrieve_result.title);
///     }
/// }
/// ```

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingZoneMethod {
    pub id: i64,
    /// Shipping method instance ID.
    pub instance_id: i32,
    /// Shipping method customer facing title.
    pub title: String,
    /// Shipping method sort order.
    pub order: i32,
    /// Shipping method enabled status.
    pub enabled: bool,
    /// Shipping method ID.
    pub method_id: String,
    /// Shipping method title.
    pub method_title: String,
    /// Shipping method description.
    pub method_description: String,
    /// Shipping method settings.
    pub settings: ShippingSettings,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingSettings {
    pub title: ShippingMethodSettings,
    pub requires: Option<ShippingMethodSettings>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingMethodSettings {
    /// A unique identifier for the setting.
    pub id: String,
    /// A human readable label for the setting used in interfaces.
    pub label: String,
    /// A human readable description for the setting used in interfaces.
    pub description: String,
    /// Type of setting. Options: text, email, number, color, password, textarea, select, multiselect, radio, image_width and checkbox.
    #[serde(rename = "type")]
    pub settings_type: SettingsType,
    /// Setting value.
    pub value: String,
    /// Default value for the setting.
    #[serde(rename = "default")]
    pub default_value: String,
    /// Additional help text shown to the user about the setting.
    pub tip: String,
    /// Placeholder text to be displayed in text inputs.
    pub placeholder: String,
    pub options: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SettingsType {
    Text,
    Email,
    Number,
    Color,
    Password,
    Textarea,
    Select,
    Multiselect,
    Radio,
    ImageWidth,
    Checkbox,
}

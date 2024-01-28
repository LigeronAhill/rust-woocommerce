use serde::{Deserialize, Serialize};

use crate::controllers::payment_gateways::PaymentGatewayUpdate;
/// ```rust
/// #[cfg(test)]
/// mod tests {
///     use crate::{payment_gateways::PaymentGateway, ApiClient, Entity};
///
///     #[tokio::test]
///     async fn test_list_all_retrieve_payment_gateway() {
///         let client = ApiClient::from_env().unwrap();
///         let gts = client
///             .list_all::<PaymentGateway>(Entity::PaymentGateway)
///             .await
///             .unwrap();
///         assert!(!gts.is_empty());
///         if let Some(f) = gts.first() {
///             let p: PaymentGateway = client
///                 .retrieve(Entity::PaymentGateway, &f.id)
///                 .await
///                 .unwrap();
///             assert_eq!(f.id, p.id);
///         }
///     }
///     #[tokio::test]
///     async fn test_update_payment_gateway() {
///         let client = ApiClient::from_env().unwrap();
///         let update = PaymentGateway::turn_off();
///         let id = "cheque";
///         let updated: PaymentGateway = client
///             .update(Entity::PaymentGateway, id, update)
///             .await
///             .unwrap();
///         assert!(!updated.enabled);
///     }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentGateway {
    /// Payment gateway ID.
    pub id: String,
    /// Payment gateway title on checkout.
    pub title: String,
    /// Payment gateway description on checkout.
    pub description: String,
    /// Payment gateway sort order.
    pub order: String,
    /// Payment gateway enabled status.
    pub enabled: bool,
    /// Payment gateway method title.
    pub method_title: String,
    /// Payment gateway method description.
    pub method_description: String,
    /// Supported features for this payment gateway.
    pub method_supports: Vec<String>,
    /// Payment gateway settings.
    pub settings: PaymentGatewaySettings,
}
impl PaymentGateway {
    pub fn turn_on() -> PaymentGatewayUpdate {
        PaymentGatewayUpdate { enabled: true }
    }
    pub fn turn_off() -> PaymentGatewayUpdate {
        PaymentGatewayUpdate { enabled: false }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentGatewaySettings {
    pub title: TitleInstructions,
    pub instructions: TitleInstructions,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleInstructions {
    /// A unique identifier for the setting.
    pub id: String,
    /// A human readable label for the setting used in interfaces.
    pub label: String,
    /// A human readable description for the setting used in interfaces.
    pub description: String,
    /// Type of setting. Options: text, email, number, color, password, textarea, select, multiselect, radio, image_width and checkbox.
    #[serde(rename = "type")]
    pub setting_type: SettingType,
    /// Setting value.
    pub value: String,
    /// Default value for the setting.
    pub default: String,
    /// Additional help text shown to the user about the setting.
    pub tip: String,
    /// Placeholder text to be displayed in text inputs.
    pub placeholder: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SettingType {
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
    SafeText,
}

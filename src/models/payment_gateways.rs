use serde::{Deserialize, Serialize};

use crate::controllers::payment_gateways::PaymentGatewayUpdate;

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

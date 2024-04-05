use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingOptionUpdate {
    /// Setting value.
    pub value: serde_json::Value,
}

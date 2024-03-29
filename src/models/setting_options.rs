use serde::{Deserialize, Serialize};

use crate::controllers::settings::SettingOptionUpdate;
/// ```rust
///     let client = ApiClient::from_env()?;
///     let upd = SettingOption::update("no");
///     let updated: SettingOption = client
///         .update_subentity(
///             Entity::Setting,
///             group_id,
///             SubEntity::SettingOption,
///             option_id,
///             upd,
///         )
///         .await?;
///     let upd = SettingOption::update("yes");
///     let updated: SettingOption = client
///         .update_subentity(
///             Entity::Setting,
///             group_id,
///             SubEntity::SettingOption,
///             option_id,
///             upd,
///         )
///         .await?;
///      let opts: Vec<SettingOption> = client
///             .retrieve(Entity::Setting, group_id)
///             .await?;
///      if let Some(first_opt) = opts.first() {
///          let option: SettingOption = client
///              .retrieve_subentity(
///                  Entity::Setting,
///                  group_id,
///                  SubEntity::SettingOption,
///                  first_opt.id.to_owned(),
///              )
///              .await?;
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingOption {
    /// A unique identifier for the setting.
    pub id: String,
    /// A human readable label for the setting used in interfaces.
    pub label: String,
    /// A human readable description for the setting used in interfaces.
    pub description: String,
    /// Setting value.
    pub value: Option<serde_json::Value>,
    /// Default value for the setting.
    #[serde(rename = "default")]
    pub default_value: Option<serde_json::Value>,
    /// Additional help text shown to the user about the setting.
    pub tip: Option<String>,
    /// Placeholder text to be displayed in text inputs.
    pub placeholder: Option<String>,
    /// Type of setting. Options: text, email, number, color, password, textarea, select, multiselect, radio, image_width and checkbox.
    #[serde(rename = "type")]
    pub settings_type: String,
    /// Array of options (key value pairs) for inputs such as select, multiselect, and radio buttons.
    pub options: Option<serde_json::Value>,
    /// An identifier for the group this setting belongs to.
    pub group_id: Option<String>,
}
impl SettingOption {
    pub fn update(value: impl Serialize) -> SettingOptionUpdate {
        SettingOptionUpdate {
            value: serde_json::json!(value),
        }
    }
}

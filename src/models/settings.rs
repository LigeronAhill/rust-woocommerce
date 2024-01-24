use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingGroup {
    /// A unique identifier that can be used to link settings together.
    pub id: String,
    /// A human readable label for the setting used in interfaces.
    pub label: String,
    /// A human readable description for the setting used in interfaces.
    pub description: String,
    /// ID of parent grouping.
    pub parent_id: String,
    /// IDs for settings sub groups.
    pub sub_groups: Vec<String>,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStatusTool {
    /// A unique identifier for the tool.
    pub id: String,
    /// Tool name.
    pub name: String,
    /// What running the tool will do.
    pub action: String,
    /// Tool description.
    pub description: String,
    /// Did the tool run successfully?
    pub success: Option<bool>,
    /// Tool return message.
    pub message: Option<String>,
    /// Confirm execution of the tool. Default is false.
    pub confirm: Option<bool>,
}

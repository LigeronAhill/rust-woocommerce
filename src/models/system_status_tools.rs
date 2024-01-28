use serde::{Deserialize, Serialize};
/// ```rust
/// #[cfg(test)]
/// mod tests {
///     use crate::{system_status_tools::SystemStatusTool, ApiClient};
///
///     #[tokio::test]
///     async fn test_list_all_retrieve_run_system_status_tool() {
///         let id = "regenerate_thumbnails";
///         let client = ApiClient::from_env().unwrap();
///         let result = client
///             .list_all::<SystemStatusTool>(crate::Entity::SystemStatusTool)
///             .await
///             .unwrap();
///         assert!(!result.is_empty());
///         let retrieved = client
///             .retrieve::<SystemStatusTool>(crate::Entity::SystemStatusTool, id)
///             .await
///             .unwrap();
///         assert_eq!(retrieved.id, id);
///         let r = client.run_system_status_tool(id).await.unwrap();
///         assert!(r.success.unwrap())
///     }
/// }
/// ```
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

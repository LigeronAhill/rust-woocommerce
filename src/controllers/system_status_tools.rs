use crate::{system_status_tools::SystemStatusTool, ApiClient, Result};

impl ApiClient {
    pub async fn run_system_status_tool(&self, id: impl Into<String>) -> Result<SystemStatusTool> {
        let uri = format!("{}system_status/tools/{}", self.base_url(), id.into());
        let result = self
            .client
            .put(&uri)
            .basic_auth(self.ck(), Some(self.cs()))
            .send()
            .await?
            .json()
            .await?;
        Ok(result)
    }
}
#[cfg(test)]
mod tests {
    use crate::{system_status_tools::SystemStatusTool, ApiClient};

    #[tokio::test]
    async fn test_list_all_retrieve_run_system_status_tool() {
        const ID: &str = "regenerate_thumbnails";
        let client = ApiClient::from_env().unwrap();
        let result = client
            .list_all::<SystemStatusTool>(crate::Entity::SystemStatusTool)
            .await
            .unwrap();
        assert!(!result.is_empty());
        let retrieved = client
            .retrieve::<SystemStatusTool>(crate::Entity::SystemStatusTool, ID)
            .await
            .unwrap();
        assert_eq!(retrieved.id, ID);
        // let r = client.run_system_status_tool(ID).await.unwrap();
        // assert!(r.success.unwrap())
    }
}

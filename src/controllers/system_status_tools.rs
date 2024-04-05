use crate::{system_status_tools::SystemStatusTool, ApiClient};
use anyhow::Result;

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

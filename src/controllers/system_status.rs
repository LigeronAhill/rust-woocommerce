#[cfg(test)]
mod tests {
    use crate::{system_status::SystemStatus, ApiClient};

    #[tokio::test]
    async fn test_list_all_system_status() {
        let client = ApiClient::from_env().unwrap();
        let result = client
            .list_all::<SystemStatus>(crate::Entity::SystemStatus)
            .await
            .unwrap();
        assert_eq!(result.len(), 1);
    }
}

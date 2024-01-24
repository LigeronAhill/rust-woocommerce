#[cfg(test)]
mod tests {
    use crate::{payment_gateways::PaymentGateway, ApiClient, Entity};

    #[tokio::test]
    async fn test_list_all_retrieve_payment_gateway() {
        let client = ApiClient::from_env().unwrap();
        let gts = client
            .list_all::<PaymentGateway>(Entity::PaymentGateway)
            .await
            .unwrap();
        assert!(!gts.is_empty());
        if let Some(f) = gts.first() {
            let p: PaymentGateway = client
                .retrieve(Entity::PaymentGateway, &f.id)
                .await
                .unwrap();
            assert_eq!(f.id, p.id);
        }
    }
    #[tokio::test]
    async fn test_update_payment_gateway() {
        let client = ApiClient::from_env().unwrap();
        let mut update = std::collections::HashMap::new();
        update.insert("enabled", false);
        let id = "cheque";
        let updated: PaymentGateway = client
            .update(Entity::PaymentGateway, id, update)
            .await
            .unwrap();
        assert!(!updated.enabled);
    }
}

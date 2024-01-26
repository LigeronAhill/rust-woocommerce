#[cfg(test)]
mod tests {
    use crate::{shipping_zone_methods::ShippingZoneMethod, ApiClient, Entity, SubEntity};

    #[tokio::test]
    async fn test_list_all_retrieve_shipping_zone_methods() {
        let client = ApiClient::from_env().unwrap();
        let result = client
            .list_all_subentities::<ShippingZoneMethod>(
                Entity::ShippingZone,
                1,
                SubEntity::ShippingZoneMethod,
            )
            .await
            .unwrap();
        assert!(!result.is_empty());
        let first = result.first().unwrap();
        let retrieve_result = client
            .retrieve_subentity::<ShippingZoneMethod>(
                Entity::ShippingZone,
                1,
                SubEntity::ShippingZoneMethod,
                first.instance_id,
            )
            .await
            .unwrap();
        assert_eq!(first.title, retrieve_result.title);
    }
}

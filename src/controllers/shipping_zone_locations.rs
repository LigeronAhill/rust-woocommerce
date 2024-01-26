#[cfg(test)]
mod tests {
    use crate::{shipping_zone_locations::ShippingZoneLocation, ApiClient, Entity};

    #[tokio::test]
    async fn test_list_all_shipping_zone_locations() {
        let client = ApiClient::from_env().unwrap();
        let result = client
            .list_all_subentities::<ShippingZoneLocation>(
                Entity::ShippingZone,
                1,
                crate::SubEntity::ShippingZoneLocation,
            )
            .await
            .unwrap();
        assert!(!result.is_empty());
    }
}

use serde::{Deserialize, Serialize};
/// ```rust
/// #[cfg(test)]
/// mod tests {
///     use crate::{shipping_methods::ShippingMethod, ApiClient, Entity};
///
///     #[tokio::test]
///     async fn test_list_all_retrieve_shipping_methods() {
///         let client = ApiClient::from_env().unwrap();
///         let result = client
///             .list_all::<ShippingMethod>(Entity::ShippingMethod)
///             .await
///             .unwrap();
///         assert!(!result.is_empty());
///         let first = result.first().unwrap();
///         let retrieved = client
///             .retrieve::<ShippingMethod>(Entity::ShippingMethod, &first.id)
///             .await
///             .unwrap();
///         assert_eq!(first.title, retrieved.title);
///     }
/// }
/// ```

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingMethod {
    /// Method ID.
    pub id: String,
    /// Shipping method title.
    pub title: String,
    /// Shipping method description.
    pub description: String,
}

use serde::{Deserialize, Serialize};

use crate::controllers::tax_rates::{TaxRateCreateBuilder, TaxRateUpdateBuilder};
/// ```rust
/// #[cfg(test)]
/// mod tests {
///     use crate::{tax_rates::TaxRate, ApiClient, BatchObject, Entity};
///
///     #[tokio::test]
///     async fn test_list_all_retrieve_tax_rates() {
///         let client = ApiClient::from_env().unwrap();
///         let all_tax_rate = client.list_all::<TaxRate>(Entity::TaxRate).await.unwrap();
///         if let Some(first) = all_tax_rate.first() {
///             let retrieved: TaxRate = client.retrieve(Entity::TaxRate, first.id).await.unwrap();
///             assert_eq!(first.id, retrieved.id);
///         }
///         assert!(!all_tax_rate.is_empty());
///     }
///     #[tokio::test]
///     async fn test_create_update_batch_update_delete_tax_rate() {
///         let client = ApiClient::from_env().unwrap();
///         let create = TaxRate::create()
///             .country("RU")
///             .postcode("117133")
///             .city("Москва")
///             .rate("10")
///             .name("Vaaagh")
///             .priority(9)
///             .compound()
///             .disable_shipping()
///             .order(6)
///             .build();
///         let created: TaxRate = client.create(Entity::TaxRate, create).await.unwrap();
///         let update = TaxRate::update().name("nooooo").build();
///         let updated: TaxRate = client
///             .update(Entity::TaxRate, created.id, update)
///             .await
///             .unwrap();
///         assert_eq!(created.id, updated.id);
///         let batch_update = TaxRate::update().name("yeeeeaaah").id(created.id).build();
///         let batch = BatchObject::builder().add_update(batch_update).build();
///         let updated: BatchObject<TaxRate> =
///             client.batch_update(Entity::TaxRate, batch).await.unwrap();
///         assert!(updated.update.is_some());
///         let deleted: TaxRate = client.delete(Entity::TaxRate, created.id).await.unwrap();
///         assert_eq!(deleted.id, created.id);
///     }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxRate {
    /// Unique identifier for the resource.READ-ONLY
    pub id: i32,
    /// Country ISO 3166 code.
    pub country: String,
    /// State code.
    pub state: String,
    /// Postcode/ZIP, it doesn't support multiple values. Deprecated as of WooCommerce 5.3, postcodes should be used instead.
    pub postcode: String,
    /// City name, it doesn't support multiple values. Deprecated as of WooCommerce 5.3, postcodes should be used instead.
    pub city: String,
    /// Postcodes/ZIPs.
    pub postcodes: Vec<String>,
    /// City names.
    pub cities: Vec<String>,
    /// Tax rate.
    pub rate: String,
    /// Tax rate name.
    pub name: String,
    /// Tax priority. Only 1 matching rate per priority will be used. To define multiple tax rates for a single area you need to specify a different priority per rate. Default is 1.
    pub priority: i32,
    /// Whether or not this is a compound rate. Compound tax rates are applied on top of other tax rates. Default is false.
    pub compound: bool,
    /// Whether or not this tax rate also gets applied to shipping. Default is true.
    pub shipping: bool,
    /// Indicates the order that will appear in queries.
    pub order: i32,
    /// Tax class. Default is standard.
    pub class: String,
}
impl TaxRate {
    pub fn create() -> TaxRateCreateBuilder {
        TaxRateCreateBuilder::default()
    }
    pub fn update() -> TaxRateUpdateBuilder {
        TaxRateUpdateBuilder::default()
    }
}

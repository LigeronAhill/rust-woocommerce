use serde::{Deserialize, Serialize};

use crate::controllers::tax_rates::{TaxRateCreateBuilder, TaxRateUpdateBuilder};

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

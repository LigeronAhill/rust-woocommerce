use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxRateCreate {
    country: Option<String>,
    state: Option<String>,
    postcodes: Option<Vec<String>>,
    cities: Option<Vec<String>>,
    rate: Option<String>,
    name: Option<String>,
    priority: Option<i32>,
    compound: Option<bool>,
    shipping: Option<bool>,
    order: Option<i32>,
    class: Option<String>,
}
#[derive(Default)]
pub struct TaxRateCreateBuilder {
    country: Option<String>,
    state: Option<String>,
    postcodes: Option<Vec<String>>,
    cities: Option<Vec<String>>,
    rate: Option<String>,
    name: Option<String>,
    priority: Option<i32>,
    compound: Option<bool>,
    shipping: Option<bool>,
    order: Option<i32>,
    class: Option<String>,
}
impl TaxRateCreateBuilder {
    /// Country ISO 3166 code.
    pub fn country(&mut self, country_iso: impl Into<String>) -> &mut Self {
        let _ = self.country.insert(country_iso.into());
        self
    }
    /// State code.
    pub fn state(&mut self, state_code: impl Into<String>) -> &mut Self {
        let _ = self.state.insert(state_code.into());
        self
    }
    /// Postcodes/ZIPs.
    pub fn postcode(&mut self, post_code: impl Into<String>) -> &mut Self {
        self.postcodes.get_or_insert(vec![]).push(post_code.into());
        self
    }
    /// City names.
    pub fn city(&mut self, city: impl Into<String>) -> &mut Self {
        self.cities.get_or_insert(vec![]).push(city.into());
        self
    }
    /// Tax rate.
    pub fn rate(&mut self, rate: impl Into<String>) -> &mut Self {
        let _ = self.rate.insert(rate.into());
        self
    }
    /// Tax rate name.
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        let _ = self.name.insert(name.into());
        self
    }
    /// Tax priority. Only 1 matching rate per priority will be used. To define multiple tax rates for a single area you need to specify a different priority per rate. Default is 1.
    pub fn priority(&mut self, priority: i32) -> &mut Self {
        let _ = self.priority.insert(priority);
        self
    }
    /// Whether or not this is a compound rate. Compound tax rates are applied on top of other tax rates. Default is false.
    pub fn compound(&mut self) -> &mut Self {
        let _ = self.compound.insert(true);
        self
    }
    /// Whether or not this tax rate also gets applied to shipping. Default is true.
    pub fn disable_shipping(&mut self) -> &mut Self {
        let _ = self.shipping.insert(false);
        self
    }
    /// Indicates the order that will appear in queries.
    pub fn order(&mut self, order: i32) -> &mut Self {
        let _ = self.order.insert(order);
        self
    }
    /// Tax class. Default is standard.
    pub fn class(&mut self, class: impl Into<String>) -> &mut Self {
        let _ = self.class.insert(class.into());
        self
    }
    pub fn build(&self) -> TaxRateCreate {
        TaxRateCreate {
            country: self.country.to_owned(),
            state: self.state.to_owned(),
            postcodes: self.postcodes.to_owned(),
            cities: self.cities.to_owned(),
            rate: self.rate.to_owned(),
            name: self.name.to_owned(),
            priority: self.priority,
            compound: self.compound,
            shipping: self.shipping,
            order: self.order,
            class: self.class.to_owned(),
        }
    }
}
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxRateUpdate {
    id: Option<i32>,
    country: Option<String>,
    state: Option<String>,
    postcodes: Option<Vec<String>>,
    cities: Option<Vec<String>>,
    rate: Option<String>,
    name: Option<String>,
    priority: Option<i32>,
    compound: Option<bool>,
    shipping: Option<bool>,
    order: Option<i32>,
    class: Option<String>,
}
#[derive(Default)]
pub struct TaxRateUpdateBuilder {
    id: Option<i32>,
    country: Option<String>,
    state: Option<String>,
    postcodes: Option<Vec<String>>,
    cities: Option<Vec<String>>,
    rate: Option<String>,
    name: Option<String>,
    priority: Option<i32>,
    compound: Option<bool>,
    shipping: Option<bool>,
    order: Option<i32>,
    class: Option<String>,
}
impl TaxRateUpdateBuilder {
    /// Unique identifier for the resource.
    pub fn id(&mut self, id: i32) -> &mut Self {
        let _ = self.id.insert(id);
        self
    }
    /// Country ISO 3166 code.
    pub fn country(&mut self, country_iso: impl Into<String>) -> &mut Self {
        let _ = self.country.insert(country_iso.into());
        self
    }
    /// State code.
    pub fn state(&mut self, state_code: impl Into<String>) -> &mut Self {
        let _ = self.state.insert(state_code.into());
        self
    }
    /// Postcodes/ZIPs.
    pub fn postcode(&mut self, post_code: impl Into<String>) -> &mut Self {
        self.postcodes.get_or_insert(vec![]).push(post_code.into());
        self
    }
    /// City names.
    pub fn city(&mut self, city: impl Into<String>) -> &mut Self {
        self.cities.get_or_insert(vec![]).push(city.into());
        self
    }
    /// Tax rate.
    pub fn rate(&mut self, rate: impl Into<String>) -> &mut Self {
        let _ = self.rate.insert(rate.into());
        self
    }
    /// Tax rate name.
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        let _ = self.name.insert(name.into());
        self
    }
    /// Tax priority. Only 1 matching rate per priority will be used. To define multiple tax rates for a single area you need to specify a different priority per rate. Default is 1.
    pub fn priority(&mut self, priority: i32) -> &mut Self {
        let _ = self.priority.insert(priority);
        self
    }
    /// Whether or not this is a compound rate. Compound tax rates are applied on top of other tax rates. Default is false.
    pub fn compound(&mut self) -> &mut Self {
        let _ = self.compound.insert(true);
        self
    }
    /// Whether or not this tax rate also gets applied to shipping. Default is true.
    pub fn disable_shipping(&mut self) -> &mut Self {
        let _ = self.shipping.insert(false);
        self
    }
    /// Indicates the order that will appear in queries.
    pub fn order(&mut self, order: i32) -> &mut Self {
        let _ = self.order.insert(order);
        self
    }
    /// Tax class. Default is standard.
    pub fn class(&mut self, class: impl Into<String>) -> &mut Self {
        let _ = self.class.insert(class.into());
        self
    }
    pub fn build(&self) -> TaxRateUpdate {
        TaxRateUpdate {
            id: self.id,
            country: self.country.to_owned(),
            state: self.state.to_owned(),
            postcodes: self.postcodes.to_owned(),
            cities: self.cities.to_owned(),
            rate: self.rate.to_owned(),
            name: self.name.to_owned(),
            priority: self.priority,
            compound: self.compound,
            shipping: self.shipping,
            order: self.order,
            class: self.class.to_owned(),
        }
    }
}

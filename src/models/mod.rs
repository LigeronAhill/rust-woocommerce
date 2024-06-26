use serde_with::skip_serializing_none;

pub mod coupons;
pub mod customers;
pub mod data;
pub mod order_notes;
pub mod orders;
pub mod payment_gateways;
pub mod product_attribute_terms;
pub mod product_attributes;
pub mod product_categories;
pub mod product_reviews;
pub mod product_shipping_classes;
pub mod product_tags;
pub mod product_variations;
pub mod products;
pub mod refunds;
pub mod reports;
pub mod settings;
pub mod shipping_methods;
pub mod shipping_zone_locations;
pub mod shipping_zone_methods;
pub mod shipping_zones;
pub mod tax_classes;
pub mod tax_rates;
pub mod webhooks;
#[skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MetaData {
    pub id: Option<i32>,
    pub key: String,
    pub value: serde_json::Value,
}
#[skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BatchObject<O: serde::Serialize> {
    pub create: Option<Vec<O>>,
    pub update: Option<Vec<O>>,
    pub delete: Option<Vec<O>>,
}
impl<O> BatchObject<O>
where
    O: serde::Serialize,
{
    pub fn builder() -> BatchObjectBuilder<O>
    where
        O: serde::Serialize + Clone,
    {
        BatchObjectBuilder::default()
    }
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BatchObjectBuilder<O: serde::Serialize> {
    pub create: Option<Vec<O>>,
    pub update: Option<Vec<O>>,
    pub delete: Option<Vec<O>>,
}
impl<O> Default for BatchObjectBuilder<O>
where
    O: serde::Serialize + Clone,
{
    fn default() -> Self {
        Self {
            create: None,
            update: None,
            delete: None,
        }
    }
}
impl<O> BatchObjectBuilder<O>
where
    O: serde::Serialize + Clone,
{
    pub fn add_create(&mut self, object: O) -> &mut Self {
        self.create.get_or_insert(vec![]).push(object);
        self
    }
    pub fn add_update(&mut self, object: O) -> &mut Self {
        self.update.get_or_insert(vec![]).push(object);
        self
    }
    pub fn add_delete(&mut self, object: O) -> &mut Self {
        self.delete.get_or_insert(vec![]).push(object);
        self
    }
    pub fn extend_create(&mut self, vec: Vec<O>) -> &mut Self {
        self.create.get_or_insert(vec![]).extend(vec);
        self
    }
    pub fn extend_update(&mut self, vec: Vec<O>) -> &mut Self {
        self.update.get_or_insert(vec![]).extend(vec);
        self
    }
    pub fn extend_delete(&mut self, vec: Vec<O>) -> &mut Self {
        self.delete.get_or_insert(vec![]).extend(vec);
        self
    }
    pub fn build(&self) -> BatchObject<O> {
        BatchObject {
            create: self.create.clone(),
            update: self.update.clone(),
            delete: self.delete.clone(),
        }
    }
}

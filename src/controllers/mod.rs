use serde::{Deserialize, Serialize};

use anyhow::Result;

use crate::Config;
pub mod coupons;
pub mod customers;
pub mod data;
pub mod entities;
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
pub mod system_status;
pub mod system_status_tools;
pub mod tax_classes;
pub mod tax_rates;
pub mod webhooks;

pub trait Entity: Serialize + for<'de> Deserialize<'de> + Clone + Send + 'static {
    fn endpoint() -> String;
    fn child_endpoint(parent_id: i32) -> String;
}

#[derive(Clone)]
pub struct ApiClient {
    ck: String,
    cs: String,
    base_url: url::Url,
    client: reqwest::Client,
}

impl ApiClient {
    pub fn new(config: &Config) -> Result<Self> {
        let ck = config.woo.ck.to_owned();
        let cs = config.woo.cs.to_owned();
        let client = reqwest::Client::builder().gzip(true).build()?;
        let base_url = match url::Url::parse(&config.woo.host) {
            Ok(url) => url.join("/wp-json/wc/v3/")?,
            Err(_) => {
                let raw_url = format!("https://{}", config.woo.host);
                let url = url::Url::parse(&raw_url)?;
                url.join("/wp-json/wc/v3/")?
            }
        };
        Ok(Self {
            ck,
            cs,
            base_url,
            client,
        })
    }
    pub fn from_env() -> Result<Self> {
        let ck = std::env::var("WOO_CK")?;
        let cs = std::env::var("WOO_CS")?;
        let base_url_raw = std::env::var("BASE_URL")?;
        let base_url = url::Url::parse(&format!("{base_url_raw}/wp-json/wc/v3/"))?;
        let client = reqwest::Client::builder().gzip(true).build()?;

        Ok(Self {
            ck,
            cs,
            base_url,
            client,
        })
    }
    pub fn ck(&self) -> String {
        self.ck.clone()
    }
    pub fn cs(&self) -> String {
        self.cs.clone()
    }
    pub fn client(&self) -> reqwest::Client {
        self.client.clone()
    }
    pub fn base_url(&self) -> String {
        self.base_url.to_string()
    }
}

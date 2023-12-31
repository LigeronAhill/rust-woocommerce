use serde::{de::DeserializeOwned, Serialize};

use super::ApiClient;
use crate::{models::BatchObject, Result};
pub enum Entity {
    Coupon,
    Customer,
    Order,
    Product,
    ProductAttribute,
    ProductCategory,
    ProductShippingClass,
    ProductTag,
    ProductReview,
    Report,
    TaxRate,
    TaxClass,
    Webhook,
    Setting,
    PaymentGateway,
    ShippingZone,
    SystemStatus,
    SystemStatusTool,
    Data,
}
impl std::fmt::Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Entity::Coupon => write!(f, "coupons"),
            Entity::Customer => write!(f, "customers"),
            Entity::Order => write!(f, "orders"),
            Entity::Product => write!(f, "products"),
            Entity::ProductAttribute => write!(f, "products/attributes"),
            Entity::ProductCategory => write!(f, "products/categories"),
            Entity::ProductShippingClass => write!(f, "products/shipping_classes"),
            Entity::ProductTag => write!(f, "products/tags"),
            Entity::ProductReview => write!(f, "products/reviews"),
            Entity::Report => write!(f, "reports"),
            Entity::TaxRate => write!(f, "taxes"),
            Entity::TaxClass => write!(f, "taxes/classes"),
            Entity::Webhook => write!(f, "webhooks"),
            Entity::Setting => write!(f, "settings"),
            Entity::PaymentGateway => write!(f, "payment_gateways"),
            Entity::ShippingZone => write!(f, "shipping/zones"),
            Entity::SystemStatus => write!(f, "system_status"),
            Entity::SystemStatusTool => write!(f, "system_status/tools"),
            Entity::Data => write!(f, "data"),
        }
    }
}
impl ApiClient {
    pub async fn retrieve<T>(&self, entity: Entity, entity_id: impl std::fmt::Display) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let uri = format!("{}{entity}/{entity_id}", self.base_url());
        let response: T = self
            .client
            .get(&uri)
            .basic_auth(self.ck(), Some(self.cs()))
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }
    pub async fn list_all<T>(&self, entity: Entity) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let uri = format!("{}{entity}", self.base_url());
        let response: T = self
            .client
            .get(&uri)
            .basic_auth(self.ck(), Some(self.cs()))
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }
    pub async fn create<T>(&self, entity: Entity, object: impl Serialize) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let uri = format!("{}{entity}", self.base_url());
        let response: T = self
            .client
            .post(&uri)
            .basic_auth(self.ck(), Some(self.cs()))
            .json(&object)
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }
    pub async fn update<T>(
        &self,
        entity: Entity,
        entity_id: impl std::fmt::Display,
        object: impl Serialize,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let uri = format!("{}{entity}/{entity_id}", self.base_url());
        let response: T = self
            .client
            .put(&uri)
            .basic_auth(self.ck(), Some(self.cs()))
            .json(&object)
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }
    pub async fn delete<T>(&self, entity: Entity, entity_id: impl std::fmt::Display) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let uri = format!("{}{entity}/{entity_id}?force=true", self.base_url());
        let response: T = self
            .client
            .delete(&uri)
            .basic_auth(self.ck(), Some(self.cs()))
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }
    pub async fn batch_update<T>(
        &self,
        entity: Entity,
        batch_object: BatchObject<T>,
    ) -> Result<BatchObject<T>>
    where
        T: DeserializeOwned + Serialize,
    {
        let uri = format!("{}{entity}/batch", self.base_url());
        let response: BatchObject<T> = self
            .client
            .post(&uri)
            .basic_auth(self.ck(), Some(self.cs()))
            .json(&batch_object)
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }
    pub async fn search<T>(&self, entity: Entity, search_string: impl Into<String>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let uri = format!(
            "{}{entity}?search={}",
            self.base_url(),
            search_string.into()
        );
        let response: T = self
            .client
            .get(&uri)
            .basic_auth(self.ck(), Some(self.cs()))
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }
}

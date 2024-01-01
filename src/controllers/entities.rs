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
    pub async fn list_all<T>(&self, entity: Entity) -> Result<Vec<T>>
    where
        T: DeserializeOwned,
    {
        let mut result = vec![];
        let mut page = 1;
        let per_page = 50;
        loop {
            let uri = format!(
                "{}{entity}?page={page}&per_page={per_page}",
                self.base_url()
            );
            let response: Vec<T> = self
                .client
                .get(&uri)
                .basic_auth(self.ck(), Some(self.cs()))
                .send()
                .await?
                .json()
                .await?;
            if response.is_empty() {
                break;
            }
            result.extend(response);
            page += 1;
        }
        Ok(result)
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
    ) -> Result<Vec<BatchObject<T>>>
    where
        T: DeserializeOwned + Serialize + Clone,
    {
        let uri = format!("{}{entity}/batch", self.base_url());
        // By default it's limited to up to 100 objects to be created, updated or deleted.
        let batches = create_batches(&batch_object);
        let mut modified = Vec::new();
        for batch in batches {
            let response: BatchObject<T> = self
                .client
                .post(&uri)
                .basic_auth(self.ck(), Some(self.cs()))
                .json(&batch)
                .send()
                .await?
                .json()
                .await?;
            modified.push(response);
        }
        Ok(modified)
    }
    pub async fn search<T>(
        &self,
        entity: Entity,
        search_string: impl Into<String>,
    ) -> Result<Vec<T>>
    where
        T: DeserializeOwned,
    {
        let mut result = vec![];
        let mut page = 1;
        let per_page = 50;
        let search = search_string.into();
        loop {
            let uri = format!(
                "{}{entity}?page={page}&per_page={per_page}&search={search}",
                self.base_url(),
            );
            let response: Vec<T> = self
                .client
                .get(&uri)
                .basic_auth(self.ck(), Some(self.cs()))
                .send()
                .await?
                .json()
                .await?;
            if response.is_empty() {
                break;
            }
            result.extend(response);
            page += 1;
        }
        Ok(result)
    }
}
fn create_batches<T>(input_batch: &BatchObject<T>) -> Vec<BatchObject<T>>
where
    T: serde::Serialize + Clone,
{
    let mut result_batches = Vec::new();
    let create_pages = {
        if let Some(create) = input_batch.create.to_owned() {
            create
                .chunks(50)
                .map(|slice| slice.to_vec())
                .collect::<Vec<Vec<T>>>()
        } else {
            vec![]
        }
    };
    let update_pages = {
        if let Some(update) = input_batch.update.to_owned() {
            update
                .chunks(50)
                .map(|slice| slice.to_vec())
                .collect::<Vec<Vec<T>>>()
        } else {
            vec![]
        }
    };
    let max_length = std::cmp::max(create_pages.len(), update_pages.len());
    for i in 0..max_length {
        let mut b = BatchObject::builder();
        if let Some(create) = create_pages.get(i) {
            b.extend_create(create.clone());
        }
        if let Some(update) = update_pages.get(i) {
            b.extend_update(update.clone());
        }
        let batch = b.build();
        result_batches.push(batch);
    }
    result_batches
}

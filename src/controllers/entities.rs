use serde::{de::DeserializeOwned, Serialize};

use crate::{data::Currency, ApiClient, BatchObject, Result};

/// Enum representing the various entities that can be retrieved using the API client.
#[derive(Debug, Clone, PartialEq)]
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
    ReportCouponsTotal,
    ReportCustomersTotal,
    ReportOrdersTotal,
    ReportProductsTotal,
    ReportReviewsTotal,
    TaxRate,
    TaxClass,
    Webhook,
    Setting,
    PaymentGateway,
    ShippingZone,
    SystemStatus,
    SystemStatusTool,
    Data,
    Currency,
    Country,
    Continent,
    CurrentCurrency,
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
            Entity::Currency => write!(f, "data/currencies"),
            Entity::Country => write!(f, "data/countries"),
            Entity::Continent => write!(f, "data/continents"),
            Entity::CurrentCurrency => write!(f, "data/currencies/current"),
            Entity::ReportCouponsTotal => write!(f, "reports/coupons/totals"),
            Entity::ReportCustomersTotal => write!(f, "reports/customers/totals"),
            Entity::ReportOrdersTotal => write!(f, "reports/orders/totals"),
            Entity::ReportProductsTotal => write!(f, "reports/products/totals"),
            Entity::ReportReviewsTotal => write!(f, "reports/reviews/totals"),
        }
    }
}
impl ApiClient {
    /// Asynchronously retrieves an entity of a specific type.
    ///
    /// # Arguments
    ///
    /// * `entity` - The type of entity to retrieve.
    /// * `entity_id` - The ID of the entity to retrieve.
    ///
    /// # Returns
    ///
    /// A Result containing the deserialized entity or an error if retrieval fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// let attribute_id = 4;
    /// let result = client
    ///     .retrieve::<Attribute>(Entity::ProductAttribute, attribute_id)
    ///     .await?;
    /// ```
    pub async fn retrieve<T>(&self, entity: Entity, entity_id: impl std::fmt::Display) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let uri = format!("{}{entity}/{entity_id}", self.base_url());
        let mut response = serde_json::Value::Null;
        for i in 1..6 {
            log::debug!("Connecting {uri}, try {i}");
            match self
                .client
                .get(&uri)
                .basic_auth(self.ck(), Some(self.cs()))
                .send()
                .await
            {
                Ok(r) => {
                    log::debug!("Deserializing response from {uri}");
                    match r.json().await {
                        Ok(v) => {
                            response = v;
                            break;
                        }
                        Err(e) => {
                            log::error!("Failed to deserialize response from {uri} with error: {e}\n{} tries left", 5-i);
                            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                            continue;
                        }
                    }
                }
                Err(e) => {
                    log::error!(
                        "Failed to connect to {uri} with error: {e}\n{} tries left",
                        5 - i
                    );
                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                    continue;
                }
            }
        }
        match serde_json::from_value::<T>(response.clone()) {
            Ok(result) => Ok(result),
            Err(e) => {
                let msg = format!("Error with response to {uri} --> {e}\n{response:#?}");
                Err(msg.into())
            }
        }
    }
    /// retrieve current currency
    pub async fn retrieve_current_currency(&self) -> Result<Currency> {
        let uri = format!("{}data/currencies/current", self.base_url());
        let mut response = serde_json::Value::Null;
        for i in 1..6 {
            log::debug!("Connecting {uri}, try {i}");
            match self
                .client
                .get(&uri)
                .basic_auth(self.ck(), Some(self.cs()))
                .send()
                .await
            {
                Ok(r) => {
                    log::debug!("Deserializing response from {uri}");
                    match r.json().await {
                        Ok(v) => {
                            response = v;
                            break;
                        }
                        Err(e) => {
                            log::error!("Failed to deserialize response from {uri} with error: {e}\n{} tries left", 5-i);
                            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                            continue;
                        }
                    }
                }
                Err(e) => {
                    log::error!(
                        "Failed to connect to {uri} with error: {e}\n{} tries left",
                        5 - i
                    );
                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                    continue;
                }
            }
        }
        match serde_json::from_value::<Currency>(response.clone()) {
            Ok(result) => Ok(result),
            Err(e) => {
                let msg = format!("Error getting current currency\n{response:#?}\n Error: {e:#?}");
                Err(msg.into())
            }
        }
    }
    /// Asynchronously lists all entities of a specific type.
    ///
    /// # Arguments
    ///
    /// * `entity` - The type of entity to list.
    ///
    /// # Returns
    ///
    /// A Result containing a vector of the retrieved entities or an error if retrieval fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// let result = client
    ///     .list_all::<Attribute>(Entity::ProductAttribute)
    ///     .await?;
    /// ```
    pub async fn list_all<T>(&self, entity: Entity) -> Result<Vec<T>>
    where
        T: DeserializeOwned,
    {
        let mut result = vec![];
        let mut page = 1;
        let per_page = 50;
        if entity == Entity::Data
            || entity == Entity::Currency
            || entity == Entity::Country
            || entity == Entity::Continent
            || entity == Entity::ProductAttribute
            || entity == Entity::Report
            || entity == Entity::ReportCouponsTotal
            || entity == Entity::ReportCustomersTotal
            || entity == Entity::ReportOrdersTotal
            || entity == Entity::ReportProductsTotal
            || entity == Entity::ReportReviewsTotal
            || entity == Entity::TaxClass
        {
            let uri = format!("{}{entity}", self.base_url());
            let mut response = serde_json::Value::Null;
            for i in 1..6 {
                log::debug!("Connecting {uri}, try {i}");
                match self
                    .client
                    .get(&uri)
                    .basic_auth(self.ck(), Some(self.cs()))
                    .send()
                    .await
                {
                    Ok(r) => {
                        log::debug!("Deserializing response from {uri}");
                        match r.json().await {
                            Ok(v) => {
                                response = v;
                                break;
                            }
                            Err(e) => {
                                log::error!("Failed to deserialize response from {uri} with error: {e}\n{} tries left", 5-i);
                                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                                continue;
                            }
                        }
                    }
                    Err(e) => {
                        log::error!(
                            "Failed to connect to {uri} with error: {e}\n{} tries left",
                            5 - i
                        );
                        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                        continue;
                    }
                }
            }
            match serde_json::from_value::<Vec<T>>(response.clone()) {
                Ok(result) => {
                    return Ok(result);
                }
                Err(e) => {
                    let msg = format!("Error getting {uri}\n\n{response:#?}\n\n{e}");
                    return Err(msg.into());
                }
            }
        } else {
            loop {
                let uri = format!(
                    "{}{entity}?page={page}&per_page={per_page}",
                    self.base_url()
                );
                let mut response = serde_json::Value::Null;
                for i in 1..6 {
                    log::debug!("Connecting {uri}, try {i}");
                    match self
                        .client
                        .get(&uri)
                        .basic_auth(self.ck(), Some(self.cs()))
                        .send()
                        .await
                    {
                        Ok(r) => {
                            log::debug!("Deserializing response from {uri}");
                            match r.json().await {
                                Ok(v) => {
                                    response = v;
                                    break;
                                }
                                Err(e) => {
                                    log::error!("Failed to deserialize response from {uri} with error: {e}\n{} tries left", 5-i);
                                    tokio::time::sleep(tokio::time::Duration::from_millis(500))
                                        .await;
                                    continue;
                                }
                            }
                        }
                        Err(e) => {
                            log::error!(
                                "Failed to connect to {uri} with error: {e}\n{} tries left",
                                5 - i
                            );
                            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                            continue;
                        }
                    }
                }
                match serde_json::from_value::<Vec<T>>(response.clone()) {
                    Ok(chunk_result) => {
                        if chunk_result.is_empty() {
                            break;
                        }
                        result.extend(chunk_result);
                        page += 1;
                    }
                    Err(e) => {
                        let msg = format!("Error getting {uri}\n{response:#?}\nError: {e:#?}");
                        return Err(msg.into());
                    }
                }
            }
        }
        Ok(result)
    }
    /// Asynchronously creates entity of a specific type.
    ///
    /// # Arguments
    ///
    /// * `entity` - The type of entity to list.
    /// * `object` - The serialized object containing the updated data for the entity.
    ///
    /// # Returns
    ///
    /// A Result containing the created entity or an error if the create fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// let attribute = AttributeDTO::builder()
    ///     .name("test attribute")
    ///     .option("69")
    ///     .build();
    /// let product_to_create = Product::create()
    ///     .name("Test product")
    ///     .sku("test product")
    ///     .regular_price("10000")
    ///     .attribute(attribute)
    ///     .build();
    /// let created_product: Product = client
    ///     .create(Entity::Product, product_to_create)
    ///     .await?;
    /// ```
    pub async fn create<T>(&self, entity: Entity, object: impl Serialize) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let uri = format!("{}{entity}", self.base_url());
        let mut response = serde_json::Value::Null;
        for i in 1..6 {
            log::debug!("Connecting {uri}, try {i}");
            match self
                .client
                .post(&uri)
                .basic_auth(self.ck(), Some(self.cs()))
                .json(&object)
                .send()
                .await
            {
                Ok(r) => {
                    log::debug!("Deserializing response from {uri}");
                    match r.json().await {
                        Ok(v) => {
                            response = v;
                            break;
                        }
                        Err(e) => {
                            log::error!("Failed to deserialize response from {uri} with error: {e}\n{} tries left", 5-i);
                            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                            continue;
                        }
                    }
                }
                Err(e) => {
                    log::error!(
                        "Failed to connect to {uri} with error: {e}\n{} tries left",
                        5 - i
                    );
                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                    continue;
                }
            }
        }
        match serde_json::from_value::<T>(response.clone()) {
            Ok(result) => Ok(result),
            Err(e) => {
                let msg = format!("Error getting {uri} --> {e}\n{response:#?}");
                Err(msg.into())
            }
        }
    }
    /// Asynchronously updates an entity of a specific type.
    ///
    /// # Arguments
    ///
    /// * `entity` - The type of entity to update.
    /// * `entity_id` - The ID of the entity to update.
    /// * `object` - The serialized object containing the updated data for the entity.
    ///
    /// # Returns
    ///
    /// A Result containing the updated entity or an error if the update fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// let product_id = 69;
    /// let product_to_update = Product::update().regular_price("5000").build();
    /// let updated_product: Product = client
    ///     .update(Entity::Product, product_id, product_to_update)
    ///     .await?;
    /// ```
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
        if entity == Entity::TaxClass {
            return Err("method not allowed".into());
        }
        let mut response = serde_json::Value::Null;
        for i in 1..6 {
            log::debug!("Connecting {uri}, try {i}");
            match self
                .client
                .put(&uri)
                .basic_auth(self.ck(), Some(self.cs()))
                .json(&object)
                .send()
                .await
            {
                Ok(r) => {
                    log::debug!("Deserializing response from {uri}");
                    match r.json().await {
                        Ok(v) => {
                            response = v;
                            break;
                        }
                        Err(e) => {
                            log::error!("Failed to deserialize response from {uri} with error: {e}\n{} tries left", 5-i);
                            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                            continue;
                        }
                    }
                }
                Err(e) => {
                    log::error!(
                        "Failed to connect to {uri} with error: {e}\n{} tries left",
                        5 - i
                    );
                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                    continue;
                }
            }
        }
        match serde_json::from_value::<T>(response.clone()) {
            Ok(result) => Ok(result),
            Err(_) => {
                let msg = format!("{response:#?}");
                Err(msg.into())
            }
        }
    }
    /// Asynchronously deletes an entity of a specific type.
    ///
    /// # Arguments
    ///
    /// * `entity` - The type of entity to delete.
    /// * `entity_id` - The ID of the entity to delete.
    ///
    /// # Returns
    ///
    /// A Result containing the deleted entity or an error if the deletion fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// let id = 69;
    /// let deleted: Product = client.delete(Entity::Product, id).await?;
    /// ```
    pub async fn delete<T>(&self, entity: Entity, entity_id: impl std::fmt::Display) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let uri = format!("{}{entity}/{entity_id}?force=true", self.base_url());
        let mut response = serde_json::Value::Null;
        for i in 1..6 {
            log::debug!("Connecting {uri}, try {i}");
            match self
                .client
                .delete(&uri)
                .basic_auth(self.ck(), Some(self.cs()))
                .send()
                .await
            {
                Ok(r) => {
                    log::debug!("Deserializing response from {uri}");
                    match r.json().await {
                        Ok(v) => {
                            response = v;
                            break;
                        }
                        Err(e) => {
                            log::error!("Failed to deserialize response from {uri} with error: {e}\n{} tries left", 5-i);
                            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                            continue;
                        }
                    }
                }
                Err(e) => {
                    log::error!(
                        "Failed to connect to {uri} with error: {e}\n{} tries left",
                        5 - i
                    );
                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                    continue;
                }
            }
        }
        match serde_json::from_value::<T>(response.clone()) {
            Ok(result) => Ok(result),
            Err(_) => {
                let msg = format!("{response:#?}");
                Err(msg.into())
            }
        }
    }
    /// Asynchronously updates a batch of entities for a specific type.
    ///
    /// # Arguments
    ///
    /// * `entity` - The type of entity to update.
    /// * `batch_object` - The batch object containing the entities to update.
    ///
    /// # Returns
    ///
    /// A Result containing the modified batch object or an error if the update fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// let product_id = 69;
    /// let product_to_update = Product::update().id(product_id).regular_price("5000").build();
    /// let batch = BatchObject::builder().add_update(product_to_update).build();
    /// let updated_products: BatchObject<Product> =
    ///     client.batch_update(Entity::Product, batch).await?;
    /// ```
    pub async fn batch_update<T, O>(
        &self,
        entity: Entity,
        batch_object: BatchObject<O>,
    ) -> Result<BatchObject<T>>
    where
        T: DeserializeOwned + Serialize + Clone,
        O: DeserializeOwned + Serialize + Clone,
    {
        let uri = format!("{}{entity}/batch", self.base_url());
        // By default it's limited to up to 100 objects to be created, updated or deleted.
        let batches = create_batches(&batch_object);
        let mut modified = BatchObject::builder();
        for batch in batches {
            let mut response = serde_json::Value::Null;
            for i in 1..6 {
                log::debug!("Connecting {uri}, try {i}");
                match self
                    .client
                    .post(&uri)
                    .basic_auth(self.ck(), Some(self.cs()))
                    .json(&batch)
                    .send()
                    .await
                {
                    Ok(r) => {
                        log::debug!("Deserializing response from {uri}");
                        match r.json().await {
                            Ok(v) => {
                                response = v;
                                break;
                            }
                            Err(e) => {
                                log::error!("Failed to deserialize response from {uri} with error: {e}\n{} tries left", 5-i);
                                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                                continue;
                            }
                        }
                    }
                    Err(e) => {
                        log::error!(
                            "Failed to connect to {uri} with error: {e}\n{} tries left",
                            5 - i
                        );
                        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                        continue;
                    }
                }
            }
            match serde_json::from_value::<BatchObject<T>>(response.clone()) {
                Ok(result) => {
                    if let Some(create) = result.create {
                        modified.extend_create(create);
                    }
                    if let Some(update) = result.update {
                        modified.extend_update(update);
                    }
                }
                Err(_) => {
                    let msg = format!("{response:#?}");
                    return Err(msg.into());
                }
            }
        }
        Ok(modified.build())
    }
    /// Asynchronously searches for entities of a specific type based on a provided search string.
    ///
    /// # Arguments
    ///
    /// * `entity` - The type of entity to search.
    /// * `search_string` - The search string used to filter entities.
    ///
    /// # Returns
    ///
    /// A Result containing a vector of entities matching the search criteria or an error if the search fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// let search_string = "string to search";
    /// let search_result: Vec<Product> =
    ///     client.search(Entity::Product, search_string).await?;
    /// ```
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
            let mut response = serde_json::Value::Null;
            for i in 1..6 {
                log::debug!("Connecting {uri}, try {i}");
                match self
                    .client
                    .get(&uri)
                    .basic_auth(self.ck(), Some(self.cs()))
                    .send()
                    .await
                {
                    Ok(r) => {
                        log::debug!("Deserializing response from {uri}");
                        match r.json().await {
                            Ok(v) => {
                                response = v;
                                break;
                            }
                            Err(e) => {
                                log::error!("Failed to deserialize response from {uri} with error: {e}\n{} tries left", 5-i);
                                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                                continue;
                            }
                        }
                    }
                    Err(e) => {
                        log::error!(
                            "Failed to connect to {uri} with error: {e}\n{} tries left",
                            5 - i
                        );
                        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                        continue;
                    }
                }
            }
            match serde_json::from_value::<Vec<T>>(response.clone()) {
                Ok(chunk_result) => {
                    if chunk_result.is_empty() {
                        break;
                    }
                    result.extend(chunk_result);
                    page += 1;
                }
                Err(e) => {
                    let msg = format!("Error getting {uri} - {e}\n{response:#?}");
                    return Err(msg.into());
                }
            }
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
/// Enum representing various sub-entities that can be associated with the main entities.
#[derive(Debug, Clone, PartialEq)]
pub enum SubEntity {
    OrderNote,
    Refund,
    ProductVariation,
    AttributeTerm,
}
impl std::fmt::Display for SubEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubEntity::OrderNote => write!(f, "notes"),
            SubEntity::Refund => write!(f, "refunds"),
            SubEntity::ProductVariation => write!(f, "variations"),
            SubEntity::AttributeTerm => write!(f, "terms"),
        }
    }
}
impl ApiClient {
    /// Asynchronously retrieves a specific subentity of a given entity.
    ///
    /// # Arguments
    ///
    /// * `entity` - The type of the main entity.
    /// * `entity_id` - The ID of the main entity.
    /// * `subentity` - The subentity to retrieve.
    /// * `subentity_id` - The ID of the subentity to retrieve.
    ///
    /// # Returns
    ///
    /// A Result containing the deserialized subentity or an error if retrieval fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// let product_id = 69;
    /// let variation_id = 42;
    /// let result: ProductVariation = client
    ///     .retrieve_subentity(Entity::Product, product_id, SubEntity::ProductVariation, variation_id)
    ///     .await?;
    /// ```
    pub async fn retrieve_subentity<T>(
        &self,
        entity: Entity,
        entity_id: impl std::fmt::Display,
        subentity: SubEntity,
        subentity_id: impl std::fmt::Display,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let uri = format!(
            "{}{entity}/{entity_id}/{subentity}/{subentity_id}",
            self.base_url()
        );
        let mut response = serde_json::Value::Null;
        for i in 1..6 {
            log::debug!("Connecting {uri}, try {i}");
            match self
                .client
                .get(&uri)
                .basic_auth(self.ck(), Some(self.cs()))
                .send()
                .await
            {
                Ok(r) => {
                    log::debug!("Deserializing response from {uri}");
                    match r.json().await {
                        Ok(v) => {
                            response = v;
                            break;
                        }
                        Err(e) => {
                            log::error!("Failed to deserialize response from {uri} with error: {e}\n{} tries left", 5-i);
                            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                            continue;
                        }
                    }
                }
                Err(e) => {
                    log::error!(
                        "Failed to connect to {uri} with error: {e}\n{} tries left",
                        5 - i
                    );
                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                    continue;
                }
            }
        }
        match serde_json::from_value::<T>(response.clone()) {
            Ok(result) => Ok(result),
            Err(e) => {
                let msg = format!("Error with response to {uri} --> {e}\n{response:#?}");
                Err(msg.into())
            }
        }
    }
    /// Asynchronously lists all subentities of a specific entity.
    ///
    /// # Arguments
    ///
    /// * `entity` - The type of the main entity.
    /// * `entity_id` - The ID of the main entity.
    /// * `subentity` - The type of subentity to list.
    ///
    /// # Returns
    ///
    /// A Result containing a vector of the retrieved subentities or an error if retrieval fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// let product_id = 69;
    /// let result: Vec<ProductVariation> = client
    ///     .list_all_subentities(Entity::Product, product_id, SubEntity::ProductVariation)
    ///     .await?;
    /// ```
    pub async fn list_all_subentities<T>(
        &self,
        entity: Entity,
        entity_id: impl std::fmt::Display,
        subentity: SubEntity,
    ) -> Result<Vec<T>>
    where
        T: DeserializeOwned,
    {
        let uri = format!("{}{entity}/{entity_id}/{subentity}", self.base_url());
        let mut response = serde_json::Value::Null;
        for i in 1..6 {
            log::debug!("Connecting {uri}, try {i}");
            match self
                .client
                .get(&uri)
                .basic_auth(self.ck(), Some(self.cs()))
                .send()
                .await
            {
                Ok(r) => {
                    log::debug!("Deserializing response from {uri}");
                    match r.json().await {
                        Ok(v) => {
                            response = v;
                            break;
                        }
                        Err(e) => {
                            log::error!("Failed to deserialize response from {uri} with error: {e}\n{} tries left", 5-i);
                            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                            continue;
                        }
                    }
                }
                Err(e) => {
                    log::error!(
                        "Failed to connect to {uri} with error: {e}\n{} tries left",
                        5 - i
                    );
                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                    continue;
                }
            }
        }
        match serde_json::from_value::<Vec<T>>(response.clone()) {
            Ok(result) => Ok(result),
            Err(e) => {
                let msg = format!("Error getting {uri}\n\n{response:#?}\n\n{e}");
                Err(msg.into())
            }
        }
    }
    /// Asynchronously reates a specific subentity of a given entity.
    ///
    /// # Arguments
    ///
    /// * `entity` - The type of the main entity.
    /// * `entity_id` - The ID of the main entity.
    /// * `subentity` - The type of the subentity to update.
    /// * `object` - The serialized object containing the data to create for the subentity.
    ///
    /// # Returns
    ///
    /// A Result containing the created subentity or an error if the update fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// let product_id = 69;
    /// let variation_to_create = ProductVariation::create().sku("test-sku").build();
    /// let created: ProductVariation = client
    ///     .create_subentity(
    ///         Entity::Product,
    ///         product_id,
    ///         SubEntity::ProductVariation,
    ///         variation_to_create,
    ///     )
    ///     .await?;
    /// ```
    pub async fn create_subentity<T>(
        &self,
        entity: Entity,
        entity_id: impl std::fmt::Display,
        subentity: SubEntity,
        object: impl Serialize,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let uri = format!("{}{entity}/{entity_id}/{subentity}", self.base_url());
        let mut response = serde_json::Value::Null;
        for i in 1..6 {
            log::debug!("Connecting {uri}, try {i}");
            match self
                .client
                .post(&uri)
                .basic_auth(self.ck(), Some(self.cs()))
                .json(&object)
                .send()
                .await
            {
                Ok(r) => {
                    log::debug!("Deserializing response from {uri}");
                    match r.json().await {
                        Ok(v) => {
                            response = v;
                            break;
                        }
                        Err(e) => {
                            log::error!("Failed to deserialize response from {uri} with error: {e}\n{} tries left", 5-i);
                            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                            continue;
                        }
                    }
                }
                Err(e) => {
                    log::error!(
                        "Failed to connect to {uri} with error: {e}\n{} tries left",
                        5 - i
                    );
                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                    continue;
                }
            }
        }
        match serde_json::from_value::<T>(response.clone()) {
            Ok(result) => Ok(result),
            Err(e) => {
                let msg = format!("Error getting {uri} --> {e}\n{response:#?}");
                Err(msg.into())
            }
        }
    }
    /// Asynchronously updates a specific subentity of a given entity.
    ///
    /// # Arguments
    ///
    /// * `entity` - The type of the main entity.
    /// * `entity_id` - The ID of the main entity.
    /// * `subentity` - The type of the subentity to update.
    /// * `subentity_id` - The ID of the subentity to update.
    /// * `object` - The serialized object containing the updated data for the subentity.
    ///
    /// # Returns
    ///
    /// A Result containing the updated subentity or an error if the update fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// let product_id = 69;
    /// let variation_id = 69;
    /// let update = ProductVariation::update().regular_price("4000").build();
    /// let updated: ProductVariation = client
    ///     .update_subentity(
    ///         Entity::Product,
    ///         product_id,
    ///         SubEntity::ProductVariation,
    ///         variation_id,
    ///         update,
    ///     )
    ///     .await?;
    /// ```
    pub async fn update_subentity<T>(
        &self,
        entity: Entity,
        entity_id: impl std::fmt::Display,
        subentity: SubEntity,
        subentity_id: impl std::fmt::Display,
        object: impl Serialize,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let uri = format!(
            "{}{entity}/{entity_id}/{subentity}/{subentity_id}",
            self.base_url()
        );
        if subentity == SubEntity::OrderNote || subentity == SubEntity::Refund {
            let msg = format!("No such method for {subentity}");
            return Err(msg.into());
        }
        let mut response = serde_json::Value::Null;
        for i in 1..6 {
            log::debug!("Connecting {uri}, try {i}");
            match self
                .client
                .put(&uri)
                .basic_auth(self.ck(), Some(self.cs()))
                .json(&object)
                .send()
                .await
            {
                Ok(r) => {
                    log::debug!("Deserializing response from {uri}");
                    match r.json().await {
                        Ok(v) => {
                            response = v;
                            break;
                        }
                        Err(e) => {
                            log::error!("Failed to deserialize response from {uri} with error: {e}\n{} tries left", 5-i);
                            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                            continue;
                        }
                    }
                }
                Err(e) => {
                    log::error!(
                        "Failed to connect to {uri} with error: {e}\n{} tries left",
                        5 - i
                    );
                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                    continue;
                }
            }
        }
        match serde_json::from_value::<T>(response.clone()) {
            Ok(result) => Ok(result),
            Err(e) => {
                let msg = format!("Error getting {uri} - {e}\n{response:#?}");
                Err(msg.into())
            }
        }
    }
    /// Asynchronously deletes a specific subentity of a given entity.
    ///
    /// # Arguments
    ///
    /// * `entity` - The type of the main entity.
    /// * `entity_id` - The ID of the main entity.
    /// * `subentity` - The type of the subentity to delete.
    /// * `subentity_id` - The ID of the subentity to delete.
    ///
    /// # Returns
    ///
    /// A Result containing the status of the deletion operation or an error if the deletion fails.    
    ///
    /// # Example
    ///
    /// ```rust
    /// let product_id = 69;
    /// let variation_id = 69;
    /// let deleted: ProductVariation = client
    ///     .delete_subentity(Entity::Product, product_id, SubEntity::ProductVariation, variation_id)
    ///     .await?;
    /// ```
    pub async fn delete_subentity<T>(
        &self,
        entity: Entity,
        entity_id: impl std::fmt::Display,
        subentity: SubEntity,
        subentity_id: impl std::fmt::Display,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let uri = format!(
            "{}{entity}/{entity_id}/{subentity}/{subentity_id}?force=true",
            self.base_url()
        );
        let mut response = serde_json::Value::Null;
        for i in 1..6 {
            log::debug!("Connecting {uri}, try {i}");
            match self
                .client
                .delete(&uri)
                .basic_auth(self.ck(), Some(self.cs()))
                .send()
                .await
            {
                Ok(r) => {
                    log::debug!("Deserializing response from {uri}");
                    match r.json().await {
                        Ok(v) => {
                            response = v;
                            break;
                        }
                        Err(e) => {
                            log::error!("Failed to deserialize response from {uri} with error: {e}\n{} tries left", 5-i);
                            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                            continue;
                        }
                    }
                }
                Err(e) => {
                    log::error!(
                        "Failed to connect to {uri} with error: {e}\n{} tries left",
                        5 - i
                    );
                    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                    continue;
                }
            }
        }
        match serde_json::from_value::<T>(response.clone()) {
            Ok(result) => Ok(result),
            Err(e) => {
                let msg = format!("Error getting {uri} - {e}\n{response:#?}");
                Err(msg.into())
            }
        }
    }
}

use serde_with::skip_serializing_none;

use crate::models::{coupons::DiscountType, MetaData};
#[skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct CreateCoupon {
    code: String,
    discount_type: DiscountType,
    amount: String,
    description: Option<String>,
    date_expires: Option<String>,
    date_expires_gmt: Option<String>,
    individual_use: bool,
    product_ids: Option<Vec<i32>>,
    excluded_product_ids: Option<Vec<i32>>,
    usage_limit: Option<i32>,
    usage_limit_per_user: Option<i32>,
    limit_usage_to_x_items: Option<i32>,
    free_shipping: bool,
    product_categories: Option<Vec<i32>>,
    excluded_product_categories: Option<Vec<i32>>,
    exclude_sale_items: bool,
    minimum_amount: Option<String>,
    maximum_amount: Option<String>,
    email_restrictions: Option<Vec<String>>,
    meta_data: Option<Vec<MetaData>>,
}
impl CreateCoupon {
    pub fn new(
        code: impl Into<String>,
        discount_type: DiscountType,
        amount: impl Into<String>,
    ) -> Self {
        Self {
            code: code.into(),
            discount_type,
            amount: amount.into(),
            ..Default::default()
        }
    }
    pub fn builder() -> CreateCouponBuilder<NoCode, NoDiscountType, NoAmount> {
        CreateCouponBuilder::default()
    }
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct CreateCouponBuilder<C, D, A> {
    code: C,
    discount_type: D,
    amount: A,
    description: Option<String>,
    date_expires: Option<String>,
    date_expires_gmt: Option<String>,
    individual_use: bool,
    product_ids: Option<Vec<i32>>,
    excluded_product_ids: Option<Vec<i32>>,
    usage_limit: Option<i32>,
    usage_limit_per_user: Option<i32>,
    limit_usage_to_x_items: Option<i32>,
    free_shipping: bool,
    product_categories: Option<Vec<i32>>,
    excluded_product_categories: Option<Vec<i32>>,
    exclude_sale_items: bool,
    minimum_amount: Option<String>,
    maximum_amount: Option<String>,
    email_restrictions: Option<Vec<String>>,
    meta_data: Option<Vec<MetaData>>,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct NoCode;
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct NoDiscountType;
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct NoAmount;
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct WithCode(String);
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct WithDiscountType(DiscountType);
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct WithAmount(String);
impl<C, D, A> CreateCouponBuilder<C, D, A> {
    /// Coupon code.
    pub fn code(self, code: impl Into<String>) -> CreateCouponBuilder<WithCode, D, A> {
        CreateCouponBuilder {
            code: WithCode(code.into()),
            discount_type: self.discount_type,
            amount: self.amount,
            description: self.description,
            date_expires: self.date_expires,
            date_expires_gmt: self.date_expires_gmt,
            individual_use: self.individual_use,
            product_ids: self.product_ids,
            excluded_product_ids: self.excluded_product_ids,
            usage_limit: self.usage_limit,
            usage_limit_per_user: self.usage_limit_per_user,
            limit_usage_to_x_items: self.limit_usage_to_x_items,
            free_shipping: self.free_shipping,
            product_categories: self.product_categories,
            excluded_product_categories: self.excluded_product_categories,
            exclude_sale_items: self.exclude_sale_items,
            minimum_amount: self.minimum_amount,
            maximum_amount: self.maximum_amount,
            email_restrictions: self.email_restrictions,
            meta_data: self.meta_data,
        }
    }
    /// Determines the type of discount that will be applied.
    pub fn discount_type(
        self,
        discount_type: DiscountType,
    ) -> CreateCouponBuilder<C, WithDiscountType, A> {
        CreateCouponBuilder {
            code: self.code,
            discount_type: WithDiscountType(discount_type),
            amount: self.amount,
            description: self.description,
            date_expires: self.date_expires,
            date_expires_gmt: self.date_expires_gmt,
            individual_use: self.individual_use,
            product_ids: self.product_ids,
            excluded_product_ids: self.excluded_product_ids,
            usage_limit: self.usage_limit,
            usage_limit_per_user: self.usage_limit_per_user,
            limit_usage_to_x_items: self.limit_usage_to_x_items,
            free_shipping: self.free_shipping,
            product_categories: self.product_categories,
            excluded_product_categories: self.excluded_product_categories,
            exclude_sale_items: self.exclude_sale_items,
            minimum_amount: self.minimum_amount,
            maximum_amount: self.maximum_amount,
            email_restrictions: self.email_restrictions,
            meta_data: self.meta_data,
        }
    }
    /// The amount of discount. Should always be numeric, even if setting a percentage.
    pub fn amount(self, amount: impl Into<String>) -> CreateCouponBuilder<C, D, WithAmount> {
        CreateCouponBuilder {
            code: self.code,
            discount_type: self.discount_type,
            amount: WithAmount(amount.into()),
            description: self.description,
            date_expires: self.date_expires,
            date_expires_gmt: self.date_expires_gmt,
            individual_use: self.individual_use,
            product_ids: self.product_ids,
            excluded_product_ids: self.excluded_product_ids,
            usage_limit: self.usage_limit,
            usage_limit_per_user: self.usage_limit_per_user,
            limit_usage_to_x_items: self.limit_usage_to_x_items,
            free_shipping: self.free_shipping,
            product_categories: self.product_categories,
            excluded_product_categories: self.excluded_product_categories,
            exclude_sale_items: self.exclude_sale_items,
            minimum_amount: self.minimum_amount,
            maximum_amount: self.maximum_amount,
            email_restrictions: self.email_restrictions,
            meta_data: self.meta_data,
        }
    }
    /// Coupon description.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        let _ = self.description.insert(description.into());
        self
    }
    /// The date the coupon expires, in the site's timezone. Format: YEAR-MONTH-DAY. Example: "2024-01-31"
    pub fn date_expires(mut self, date_expires: impl Into<String>) -> Self {
        let _ = self.date_expires.insert(date_expires.into());
        self
    }
    /// The date the coupon expires, as GMT.Format: YEAR-MONTH-DAY. Example: "2024-01-31"
    pub fn date_expires_gmt(mut self, date_expires_gmt: impl Into<String>) -> Self {
        let _ = self.date_expires_gmt.insert(date_expires_gmt.into());
        self
    }
    /// If true, the coupon can only be used individually. Other applied coupons will be removed from the cart. Default is false.
    pub fn individual_use(mut self) -> Self {
        self.individual_use = true;
        self
    }
    /// List of product IDs the coupon can be used on.
    pub fn product_id(mut self, product_id: i32) -> Self {
        self.product_ids.get_or_insert(vec![]).push(product_id);
        self
    }
    /// List of product IDs the coupon cannot be used on.
    pub fn excluded_product_id(mut self, excluded_product_id: i32) -> Self {
        self.excluded_product_ids
            .get_or_insert(vec![])
            .push(excluded_product_id);
        self
    }
    /// How many times the coupon can be used in total.
    pub fn usage_limit(mut self, usage_limit: i32) -> Self {
        let _ = self.usage_limit.insert(usage_limit);
        self
    }
    /// How many times the coupon can be used per customer.
    pub fn usage_limit_per_user(mut self, usage_limit_per_user: i32) -> Self {
        let _ = self.usage_limit_per_user.insert(usage_limit_per_user);
        self
    }
    /// Max number of items in the cart the coupon can be applied to.
    pub fn limit_usage_to_x_items(mut self, limit_usage_to_x_items: i32) -> Self {
        let _ = self.limit_usage_to_x_items.insert(limit_usage_to_x_items);
        self
    }
    /// If true and if the free shipping method requires a coupon, this coupon will enable free shipping. Default is false.
    pub fn free_shipping(mut self) -> Self {
        self.free_shipping = true;
        self
    }
    /// List of category IDs the coupon applies to.
    pub fn product_category(mut self, category_id: i32) -> Self {
        self.product_categories
            .get_or_insert(vec![])
            .push(category_id);
        self
    }
    /// List of category IDs the coupon does not apply to.
    pub fn excluded_product_category(mut self, exclude_product_category_id: i32) -> Self {
        self.excluded_product_categories
            .get_or_insert(vec![])
            .push(exclude_product_category_id);
        self
    }
    /// If true, this coupon will not be applied to items that have sale prices. Default is false.
    pub fn exclude_sale_items(mut self) -> Self {
        self.exclude_sale_items = true;
        self
    }
    /// Minimum order amount that needs to be in the cart before coupon applies.
    pub fn minimum_amount(mut self, minimum_amount: impl Into<String>) -> Self {
        let _ = self.minimum_amount.insert(minimum_amount.into());
        self
    }
    /// Maximum order amount allowed when using the coupon.
    pub fn maximum_amount(mut self, maximum_amount: impl Into<String>) -> Self {
        let _ = self.maximum_amount.insert(maximum_amount.into());
        self
    }
    /// List of email addresses that can use this coupon.
    pub fn email_restriction(mut self, email: impl Into<String>) -> Self {
        self.email_restrictions
            .get_or_insert(vec![])
            .push(email.into());
        self
    }
    /// Meta data.
    pub fn meta_data(mut self, key: impl Into<String>, value: impl serde::Serialize) -> Self {
        self.meta_data.get_or_insert(vec![]).push(MetaData {
            id: None,
            key: key.into(),
            value: serde_json::json!(value),
        });
        self
    }
}
impl CreateCouponBuilder<WithCode, WithDiscountType, WithAmount> {
    pub fn build(self) -> CreateCoupon {
        CreateCoupon {
            code: self.code.0,
            discount_type: self.discount_type.0,
            amount: self.amount.0,
            description: self.description,
            date_expires: self.date_expires,
            date_expires_gmt: self.date_expires_gmt,
            individual_use: self.individual_use,
            product_ids: self.product_ids,
            excluded_product_ids: self.excluded_product_ids,
            usage_limit: self.usage_limit,
            usage_limit_per_user: self.usage_limit_per_user,
            limit_usage_to_x_items: self.limit_usage_to_x_items,
            free_shipping: self.free_shipping,
            product_categories: self.product_categories,
            excluded_product_categories: self.excluded_product_categories,
            exclude_sale_items: self.exclude_sale_items,
            minimum_amount: self.minimum_amount,
            maximum_amount: self.maximum_amount,
            email_restrictions: self.email_restrictions,
            meta_data: self.meta_data,
        }
    }
}
#[skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct UpdateCoupon {
    id: Option<i32>,
    code: Option<String>,
    discount_type: Option<DiscountType>,
    amount: Option<String>,
    description: Option<String>,
    date_expires: Option<String>,
    date_expires_gmt: Option<String>,
    individual_use: Option<bool>,
    product_ids: Option<Vec<i32>>,
    excluded_product_ids: Option<Vec<i32>>,
    usage_limit: Option<i32>,
    usage_limit_per_user: Option<i32>,
    limit_usage_to_x_items: Option<i32>,
    free_shipping: Option<bool>,
    product_categories: Option<Vec<i32>>,
    excluded_product_categories: Option<Vec<i32>>,
    exclude_sale_items: Option<bool>,
    minimum_amount: Option<String>,
    maximum_amount: Option<String>,
    email_restrictions: Option<Vec<String>>,
    meta_data: Option<Vec<MetaData>>,
}
impl UpdateCoupon {
    pub fn builder() -> UpdateCouponBuilder {
        UpdateCouponBuilder::default()
    }
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct UpdateCouponBuilder {
    id: Option<i32>,
    code: Option<String>,
    discount_type: Option<DiscountType>,
    amount: Option<String>,
    description: Option<String>,
    date_expires: Option<String>,
    date_expires_gmt: Option<String>,
    individual_use: Option<bool>,
    product_ids: Option<Vec<i32>>,
    excluded_product_ids: Option<Vec<i32>>,
    usage_limit: Option<i32>,
    usage_limit_per_user: Option<i32>,
    limit_usage_to_x_items: Option<i32>,
    free_shipping: Option<bool>,
    product_categories: Option<Vec<i32>>,
    excluded_product_categories: Option<Vec<i32>>,
    exclude_sale_items: Option<bool>,
    minimum_amount: Option<String>,
    maximum_amount: Option<String>,
    email_restrictions: Option<Vec<String>>,
    meta_data: Option<Vec<MetaData>>,
}
impl UpdateCouponBuilder {
    /// Unique identifier for the object.
    pub fn id(&mut self, id: i32) -> &mut Self {
        let _ = self.id.insert(id);
        self
    }
    /// Coupon code.
    pub fn code(&mut self, code: impl Into<String>) -> &mut Self {
        let _ = self.code.insert(code.into());
        self
    }
    /// Determines the type of discount that will be applied.
    pub fn discount_type(&mut self, discount_type: DiscountType) -> &mut Self {
        let _ = self.discount_type.insert(discount_type);
        self
    }
    /// The amount of discount. Should always be numeric, even if setting a percentage.
    pub fn amount(&mut self, amount: impl Into<String>) -> &mut Self {
        let _ = self.amount.insert(amount.into());
        self
    }
    /// Coupon description.
    pub fn description(&mut self, description: impl Into<String>) -> &mut Self {
        let _ = self.description.insert(description.into());
        self
    }
    /// The date the coupon expires, in the site's timezone. Format: YEAR-MONTH-DAY. Example: "2024-01-31"
    pub fn date_expires(&mut self, date_expires: impl Into<String>) -> &mut Self {
        let _ = self.date_expires.insert(date_expires.into());
        self
    }
    /// The date the coupon expires, as GMT.Format: YEAR-MONTH-DAY. Example: "2024-01-31"
    pub fn date_expires_gmt(&mut self, date_expires_gmt: impl Into<String>) -> &mut Self {
        let _ = self.date_expires_gmt.insert(date_expires_gmt.into());
        self
    }
    /// If true, the coupon can only be used individually. Other applied coupons will be removed from the cart. Default is false.
    pub fn individual_use(&mut self, individual_use: bool) -> &mut Self {
        let _ = self.individual_use.insert(individual_use);
        self
    }
    /// List of product IDs the coupon can be used on.
    pub fn product_id(&mut self, product_id: i32) -> &mut Self {
        self.product_ids.get_or_insert(vec![]).push(product_id);
        self
    }
    /// List of product IDs the coupon cannot be used on.
    pub fn excluded_product_id(&mut self, excluded_product_id: i32) -> &mut Self {
        self.excluded_product_ids
            .get_or_insert(vec![])
            .push(excluded_product_id);
        self
    }
    /// How many times the coupon can be used in total.
    pub fn usage_limit(&mut self, usage_limit: i32) -> &mut Self {
        let _ = self.usage_limit.insert(usage_limit);
        self
    }
    /// How many times the coupon can be used per customer.
    pub fn usage_limit_per_user(&mut self, usage_limit_per_user: i32) -> &mut Self {
        let _ = self.usage_limit_per_user.insert(usage_limit_per_user);
        self
    }
    /// Max number of items in the cart the coupon can be applied to.
    pub fn limit_usage_to_x_items(&mut self, limit_usage_to_x_items: i32) -> &mut Self {
        let _ = self.limit_usage_to_x_items.insert(limit_usage_to_x_items);
        self
    }
    /// If true and if the free shipping method requires a coupon, this coupon will enable free shipping. Default is false.
    pub fn free_shipping(&mut self, free_shipping: bool) -> &mut Self {
        let _ = self.free_shipping.insert(free_shipping);
        self
    }
    /// List of category IDs the coupon applies to.
    pub fn product_category(&mut self, category_id: i32) -> &mut Self {
        self.product_categories
            .get_or_insert(vec![])
            .push(category_id);
        self
    }
    /// List of category IDs the coupon does not apply to.
    pub fn excluded_product_category(&mut self, exclude_product_category_id: i32) -> &mut Self {
        self.excluded_product_categories
            .get_or_insert(vec![])
            .push(exclude_product_category_id);
        self
    }
    /// If true, this coupon will not be applied to items that have sale prices. Default is false.
    pub fn exclude_sale_items(&mut self, exclude_sale_items: bool) -> &mut Self {
        let _ = self.exclude_sale_items.insert(exclude_sale_items);
        self
    }
    /// Minimum order amount that needs to be in the cart before coupon applies.
    pub fn minimum_amount(&mut self, minimum_amount: impl Into<String>) -> &mut Self {
        let _ = self.minimum_amount.insert(minimum_amount.into());
        self
    }
    /// Maximum order amount allowed when using the coupon.
    pub fn maximum_amount(&mut self, maximum_amount: impl Into<String>) -> &mut Self {
        let _ = self.maximum_amount.insert(maximum_amount.into());
        self
    }
    /// List of email addresses that can use this coupon.
    pub fn email_restriction(&mut self, email: impl Into<String>) -> &mut Self {
        self.email_restrictions
            .get_or_insert(vec![])
            .push(email.into());
        self
    }
    /// Meta data.
    pub fn meta_data(&mut self, key: impl Into<String>, value: impl serde::Serialize) -> &mut Self {
        self.meta_data.get_or_insert(vec![]).push(MetaData {
            id: None,
            key: key.into(),
            value: serde_json::json!(value),
        });
        self
    }
    pub fn build(&self) -> UpdateCoupon {
        UpdateCoupon {
            id: self.id,
            code: self.code.clone(),
            discount_type: self.discount_type.clone(),
            amount: self.amount.clone(),
            description: self.description.clone(),
            date_expires: self.date_expires.clone(),
            date_expires_gmt: self.date_expires_gmt.clone(),
            individual_use: self.individual_use,
            product_ids: self.product_ids.clone(),
            excluded_product_ids: self.excluded_product_ids.clone(),
            usage_limit: self.usage_limit,
            usage_limit_per_user: self.usage_limit_per_user,
            limit_usage_to_x_items: self.limit_usage_to_x_items,
            free_shipping: self.free_shipping,
            product_categories: self.product_categories.clone(),
            excluded_product_categories: self.excluded_product_categories.clone(),
            exclude_sale_items: self.exclude_sale_items,
            minimum_amount: self.minimum_amount.clone(),
            maximum_amount: self.maximum_amount.clone(),
            email_restrictions: self.email_restrictions.clone(),
            meta_data: self.meta_data.clone(),
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::{
        controllers::{entities::Entity, ApiClient},
        models::{coupons::Coupon, BatchObject},
    };

    use super::*;
    #[tokio::test]
    async fn test_create_coupon() {
        let client = ApiClient::from_env().unwrap();
        let coupon = Coupon::create()
            .code("complicated-coupon-for-create")
            .discount_type(DiscountType::FixedCart)
            .amount("10")
            .description("test description")
            .date_expires("2026-01-03")
            .date_expires_gmt("2026-01-03")
            .individual_use()
            .product_id(3748)
            .excluded_product_id(3744)
            .usage_limit(10)
            .usage_limit_per_user(1)
            .limit_usage_to_x_items(5)
            .free_shipping()
            .product_category(342)
            .excluded_product_category(149)
            .exclude_sale_items()
            .minimum_amount("10000")
            .maximum_amount("100000")
            .email_restriction("test@gmail.com")
            .meta_data("test-key", "test-value");
        let coupon = coupon.product_id(6);
        let coupon_to_create = coupon.build();
        let created_coupon = client
            .create::<Coupon>(Entity::Coupon, coupon_to_create)
            .await
            .unwrap();
        assert_eq!(created_coupon.usage_count, 0);
        let id = created_coupon.id;
        let _deleted: Coupon = client.delete(Entity::Coupon, id).await.unwrap();
    }
    #[tokio::test]
    async fn test_search_coupons() {
        let client = ApiClient::from_env().unwrap();
        let search_result: Vec<Coupon> = client
            .search(Entity::Coupon, "simple-coupon-for-test")
            .await
            .unwrap();
        assert!(!search_result.is_empty());
    }
    #[tokio::test]
    async fn test_update_coupon() {
        let client = ApiClient::from_env().unwrap();
        let search_result: Vec<Coupon> = client
            .search(Entity::Coupon, "simple-coupon-for-test")
            .await
            .unwrap();
        let id = search_result.first().unwrap().id;
        let update = Coupon::update().usage_limit_per_user(3).build();
        let updated: Coupon = client.update(Entity::Coupon, id, update).await.unwrap();
        assert_eq!(updated.usage_limit_per_user, Some(3))
    }
    #[tokio::test]
    async fn test_delete_coupon() {
        let client = ApiClient::from_env().unwrap();
        let coupon = Coupon::create()
            .code("complicated-coupon-for-delete")
            .discount_type(DiscountType::FixedCart)
            .amount("10")
            .description("test description")
            .date_expires("2026-01-03")
            .date_expires_gmt("2026-01-03")
            .individual_use()
            .product_id(3748)
            .excluded_product_id(3744)
            .usage_limit(10)
            .usage_limit_per_user(1)
            .limit_usage_to_x_items(5)
            .free_shipping()
            .product_category(342)
            .excluded_product_category(149)
            .exclude_sale_items()
            .minimum_amount("10000")
            .maximum_amount("100000")
            .build();
        let created_coupon = client
            .create::<Coupon>(Entity::Coupon, coupon)
            .await
            .unwrap();
        let id = created_coupon.id;
        let deleted: Coupon = client.delete(Entity::Coupon, id).await.unwrap();
        assert_eq!(id, deleted.id)
    }
    #[tokio::test]
    async fn test_retrieve_coupon() {
        let client = ApiClient::from_env().unwrap();
        let search_result: Vec<Coupon> = client
            .search(Entity::Coupon, "simple-coupon-for-test")
            .await
            .unwrap();
        let id = search_result.first().unwrap().id;
        let coupon: Coupon = client.retrieve(Entity::Coupon, id).await.unwrap();
        assert_eq!(id, coupon.id)
    }
    #[tokio::test]
    async fn test_list_all_coupons() {
        let client = ApiClient::from_env().unwrap();
        let coupons: Vec<Coupon> = client.list_all(Entity::Coupon).await.unwrap();
        assert!(!coupons.is_empty());
    }
    #[tokio::test]
    async fn test_batch_update_coupons() {
        let client = ApiClient::from_env().unwrap();
        let search_result: Vec<Coupon> = client
            .search(Entity::Coupon, "simple-coupon-for-test")
            .await
            .unwrap();
        let id = search_result.first().unwrap().id;
        let update = Coupon::update().amount("3").id(id).build();
        let batch = BatchObject::builder().add_update(update).build();
        let batched = client.batch_update(Entity::Coupon, batch).await.unwrap();
        assert!(!batched.is_empty());
    }
}

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::controllers::coupons::{
    CreateCouponBuilder, NoAmount, NoCode, NoDiscountType, UpdateCouponBuilder,
};

use super::MetaData;

/// ```rust
/// #[cfg(test)]
/// mod tests {

///     use crate::{coupons::Coupon, ApiClient, BatchObject, Entity};

///     use super::*;
///     #[tokio::test]
///     async fn test_create_coupon() {
///         let client = ApiClient::from_env().unwrap();
///         let coupon = Coupon::create()
///             .code("complicated-coupon-for-create")
///             .discount_type(DiscountType::FixedCart)
///             .amount("10")
///             .description("test description")
///             .date_expires("2026-01-03")
///             .date_expires_gmt("2026-01-03")
///             .individual_use()
///             .product_id(3748)
///             .excluded_product_id(3744)
///             .usage_limit(10)
///             .usage_limit_per_user(1)
///             .limit_usage_to_x_items(5)
///             .free_shipping()
///             .product_category(342)
///             .excluded_product_category(149)
///             .exclude_sale_items()
///             .minimum_amount("10000")
///             .maximum_amount("100000")
///             .email_restriction("test@gmail.com")
///             .meta_data("test-key", "test-value");
///         let coupon = coupon.product_id(6);
///         let coupon_to_create = coupon.build();
///         let created_coupon = client
///             .create::<Coupon>(Entity::Coupon, coupon_to_create)
///             .await
///             .unwrap();
///         assert_eq!(created_coupon.usage_count, 0);
///         let id = created_coupon.id;
///         let _deleted: Coupon = client.delete(Entity::Coupon, id).await.unwrap();
///     }
///     #[tokio::test]
///     async fn test_search_coupons() {
///         let client = ApiClient::from_env().unwrap();
///         let search_result: Vec<Coupon> = client
///             .search(Entity::Coupon, "simple-coupon-for-test")
///             .await
///             .unwrap();
///         assert!(!search_result.is_empty());
///     }
///     #[tokio::test]
///     async fn test_update_coupon() {
///         let client = ApiClient::from_env().unwrap();
///         let search_result: Vec<Coupon> = client
///             .search(Entity::Coupon, "simple-coupon-for-test")
///             .await
///             .unwrap();
///         let id = search_result.first().unwrap().id;
///         let update = Coupon::update().usage_limit_per_user(3).build();
///         let updated: Coupon = client.update(Entity::Coupon, id, update).await.unwrap();
///         assert_eq!(updated.usage_limit_per_user, Some(3))
///     }
///     #[tokio::test]
///     async fn test_delete_coupon() {
///         let client = ApiClient::from_env().unwrap();
///         let coupon = Coupon::create()
///             .code("complicated-coupon-for-delete")
///             .discount_type(DiscountType::FixedCart)
///             .amount("10")
///             .description("test description")
///             .date_expires("2026-01-03")
///             .date_expires_gmt("2026-01-03")
///             .individual_use()
///             .product_id(3748)
///             .excluded_product_id(3744)
///             .usage_limit(10)
///             .usage_limit_per_user(1)
///             .limit_usage_to_x_items(5)
///             .free_shipping()
///             .product_category(342)
///             .excluded_product_category(149)
///             .exclude_sale_items()
///             .minimum_amount("10000")
///             .maximum_amount("100000")
///             .build();
///         let created_coupon = client
///             .create::<Coupon>(Entity::Coupon, coupon)
///             .await
///             .unwrap();
///         let id = created_coupon.id;
///         let deleted: Coupon = client.delete(Entity::Coupon, id).await.unwrap();
///         assert_eq!(id, deleted.id)
///     }
///     #[tokio::test]
///     async fn test_retrieve_coupon() {
///         let client = ApiClient::from_env().unwrap();
///         let search_result: Vec<Coupon> = client
///             .search(Entity::Coupon, "simple-coupon-for-test")
///             .await
///             .unwrap();
///         let id = search_result.first().unwrap().id;
///         let coupon: Coupon = client.retrieve(Entity::Coupon, id).await.unwrap();
///         assert_eq!(id, coupon.id)
///     }
///     #[tokio::test]
///     async fn test_list_all_coupons() {
///         let client = ApiClient::from_env().unwrap();
///         let coupons: Vec<Coupon> = client.list_all(Entity::Coupon).await.unwrap();
///         assert!(!coupons.is_empty());
///     }
///     #[tokio::test]
///     async fn test_batch_update_coupons() {
///         let client = ApiClient::from_env().unwrap();
///         let search_result: Vec<Coupon> = client
///             .search(Entity::Coupon, "simple-coupon-for-test")
///             .await
///             .unwrap();
///         let id = search_result.first().unwrap().id;
///         let update = Coupon::update().amount("3").id(id).build();
///         let batch = BatchObject::builder().add_update(update).build();
///         let batched: BatchObject<Coupon> =
///             client.batch_update(Entity::Coupon, batch).await.unwrap();
///         assert!(batched.update.is_some());
///     }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coupon {
    /// Unique identifier for the object.
    pub id: i32,
    /// Coupon code.
    pub code: String,
    /// The amount of discount. Should always be numeric, even if setting a percentage.
    pub amount: String,
    /// The date the coupon was created, in the site's timezone.
    pub date_created: NaiveDateTime,
    /// The date the coupon was created, as GMT.
    pub date_created_gmt: NaiveDateTime,
    /// The date the coupon was last modified, in the site's timezone.
    pub date_modified: NaiveDateTime,
    /// The date the coupon was last modified, as GMT.
    pub date_modified_gmt: NaiveDateTime,
    /// Determines the type of discount that will be applied. Options: percent, fixed_cart and fixed_product. Default is fixed_cart.
    pub discount_type: DiscountType,
    /// Coupon description.
    pub description: String,
    /// The date the coupon expires, in the site's timezone.
    pub date_expires: Option<String>,
    /// The date the coupon expires, as GMT.
    pub date_expires_gmt: Option<String>,
    /// Number of times the coupon has been used already.
    pub usage_count: i32,
    /// If true, the coupon can only be used individually. Other applied coupons will be removed from the cart. Default is false.
    pub individual_use: bool,
    /// List of product IDs the coupon can be used on.
    pub product_ids: Vec<i32>,
    /// List of product IDs the coupon cannot be used on.
    pub excluded_product_ids: Vec<i32>,
    /// How many times the coupon can be used in total.
    pub usage_limit: Option<i32>,
    /// How many times the coupon can be used per customer.
    pub usage_limit_per_user: Option<i32>,
    /// Max number of items in the cart the coupon can be applied to.
    pub limit_usage_to_x_items: Option<i32>,
    /// If true and if the free shipping method requires a coupon, this coupon will enable free shipping. Default is false.
    pub free_shipping: bool,
    /// List of category IDs the coupon applies to.
    pub product_categories: Vec<i32>,
    /// List of category IDs the coupon does not apply to.
    pub excluded_product_categories: Vec<i32>,
    /// If true, this coupon will not be applied to items that have sale prices. Default is false.
    pub exclude_sale_items: bool,
    /// Minimum order amount that needs to be in the cart before coupon applies.
    pub minimum_amount: String,
    /// Maximum order amount allowed when using the coupon.
    pub maximum_amount: String,
    /// List of email addresses that can use this coupon.
    pub email_restrictions: Vec<String>,
    /// List of user IDs (or guest email addresses) that have used the coupon.
    pub used_by: Vec<String>,
    /// Meta data.
    pub meta_data: Vec<MetaData>,
}
impl Coupon {
    pub fn create() -> CreateCouponBuilder<NoCode, NoDiscountType, NoAmount> {
        CreateCouponBuilder::default()
    }
    pub fn update() -> UpdateCouponBuilder {
        UpdateCouponBuilder::default()
    }
}
/// Determines the type of discount that will be applied.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum DiscountType {
    #[default]
    FixedCart,
    FixedProduct,
    Percent,
}

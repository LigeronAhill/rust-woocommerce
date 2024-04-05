use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::controllers::coupons::{
    CreateCouponBuilder, NoAmount, NoCode, NoDiscountType, UpdateCouponBuilder,
};

use super::MetaData;

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

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use super::MetaData;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coupon {
    pub id: i32,
    pub code: String,
    pub amount: String,
    pub date_created: NaiveDateTime,
    pub date_created_gmt: NaiveDateTime,
    pub date_modified: NaiveDateTime,
    pub date_modified_gmt: NaiveDateTime,
    pub discount_type: DiscountType,
    pub description: String,
    pub date_expires: Option<String>,
    pub date_expires_gmt: Option<String>,
    pub usage_count: i32,
    pub individual_use: bool,
    pub product_ids: Vec<i32>,
    pub excluded_product_ids: Vec<i32>,
    pub usage_limit: Option<i32>,
    pub usage_limit_per_user: Option<i32>,
    pub limit_usage_to_x_items: Option<i32>,
    pub free_shipping: bool,
    pub product_categories: Vec<i32>,
    pub excluded_product_categories: Vec<i32>,
    pub exclude_sale_items: bool,
    pub minimum_amount: String,
    pub maximum_amount: String,
    pub email_restrictions: Vec<String>,
    pub used_by: Vec<String>,
    pub meta_data: Vec<MetaData>,
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

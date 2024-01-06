use crate::controllers::orders::{CreateOrderBuilder, UpdateOrderBuilder};

use super::{
    customers::{Billing, Shipping},
    data::CurrencyISO,
    MetaData,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    /// Unique identifier for the resource.
    pub id: i32,
    /// Parent order ID.
    pub parent_id: Option<i32>,
    /// Order number.
    pub number: String,
    /// Order key.
    pub order_key: String,
    /// Shows where the order was created.
    pub created_via: String,
    /// Version of WooCommerce which last updated the order.
    pub version: String,
    /// Order status.
    pub status: OrderStatus,
    /// Currency the order was created with, in ISO format.
    pub currency: CurrencyISO,
    /// The date the order was created, in the site's timezone.
    pub date_created: NaiveDateTime,
    /// The date the order was created, as GMT.    
    pub date_created_gmt: NaiveDateTime,
    ///The date the order was last modified, in the site's timezone.
    pub date_modified: NaiveDateTime,
    /// The date the order was last modified, as GMT
    pub date_modified_gmt: NaiveDateTime,
    /// Total discount amount for the order.
    pub discount_total: String,
    /// Total discount tax amount for the order.
    pub discount_tax: String,
    /// Total shipping amount for the order.
    pub shipping_total: String,
    /// Total shipping tax amount for the order.
    pub shipping_tax: String,
    /// Sum of line item taxes only.
    pub cart_tax: String,
    /// Grand total.
    pub total: String,
    /// Sum of all taxes.
    pub total_tax: String,
    /// True the prices included tax during checkout.
    pub prices_include_tax: bool,
    /// User ID who owns the order. 0 for guests. Default is 0.
    pub customer_id: i32,
    /// Customer's IP address.
    pub customer_ip_address: String,
    /// User agent of the customer.
    pub customer_user_agent: String,
    /// Note left by customer during checkout.
    pub customer_note: String,
    /// Billing address.
    pub billing: Billing,
    /// Shipping address.
    pub shipping: Shipping,
    /// Payment method ID.
    pub payment_method: String,
    /// Payment method title.
    pub payment_method_title: String,
    /// Unique transaction ID.
    pub transaction_id: String,
    /// The date the order was paid, in the site's timezone.
    pub date_paid: Option<NaiveDateTime>,
    /// The date the order was paid, as GMT.
    pub date_paid_gmt: Option<NaiveDateTime>,
    /// The date the order was completed, in the site's timezone.
    pub date_completed: Option<NaiveDateTime>,
    /// The date the order was completed, as GMT.
    pub date_completed_gmt: Option<NaiveDateTime>,
    /// MD5 hash of cart items to ensure orders are not modified.
    pub cart_hash: String,
    /// Meta data.
    pub meta_data: Vec<MetaData>,
    /// Line items data.
    pub line_items: Vec<OrderLineItemProperties>,
    /// Tax lines data.
    pub tax_lines: Vec<OrderTaxLineProperties>,
    /// Shipping lines data.
    pub shipping_lines: Vec<ShippingLineProperties>,
    /// Fee lines data.
    pub fee_lines: Vec<OrderFeeLineProperties>,
    /// Coupons line data.
    pub coupon_lines: Vec<OrderCouponLineProperties>,
    /// List of refunds.
    pub refunds: Vec<OrderRefundProperties>,
    // Define if the order is paid. It will set the status to processing and reduce stock items. Default is false.
    // pub set_paid: bool,
}
impl Order {
    pub fn create() -> CreateOrderBuilder {
        CreateOrderBuilder::default()
    }
    pub fn update() -> UpdateOrderBuilder {
        UpdateOrderBuilder::default()
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum OrderStatus {
    #[default]
    Pending,
    Processing,
    OnHold,
    Completed,
    Cancelled,
    Refunded,
    Failed,
    Trash,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderLineItemProperties {
    /// Item ID.
    pub id: i32,
    /// Product name.
    pub name: String,
    /// Product ID.
    pub product_id: i32,
    /// Variation ID, if applicable.
    pub variation_id: Option<i32>,
    /// Quantity ordered.
    pub quantity: i32,
    /// Slug of the tax class of product.
    pub tax_class: String,
    /// Line subtotal (before discounts).
    pub subtotal: String,
    /// Line subtotal tax (before discounts).
    pub subtotal_tax: String,
    /// Line total (after discounts).
    pub total: String,
    /// Line total tax (after discounts).
    pub total_tax: String,
    /// Line taxes.
    pub taxes: Vec<OrderTax>,
    /// Meta data.
    pub meta_data: Vec<MetaData>,
    /// Product SKU.
    pub sku: Option<String>,
    /// Product price.
    pub price: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderTaxLineProperties {
    /// Item ID.
    pub id: i32,
    /// Tax rate code.
    pub rate_code: String,
    /// Tax rate ID.
    pub rate_id: String,
    /// Tax rate label.
    pub label: String,
    /// Show if is a compound tax rate.
    pub compound: bool,
    /// Tax total (not including shipping taxes).
    pub tax_total: String,
    /// Shipping tax total.
    pub shipping_tax_total: String,
    /// Meta data.
    pub meta_data: Vec<MetaData>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingLineProperties {
    /// Item ID.
    pub id: i32,
    /// Shipping method name.
    pub method_title: String,
    /// Shipping method ID.
    pub method_id: String,
    /// Line total (after discounts).
    pub total: String,
    /// Line total tax (after discounts).
    pub total_tax: String,
    /// Line taxes.
    pub taxes: Vec<OrderTax>,
    /// Meta data.
    pub meta_data: Vec<MetaData>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderFeeLineProperties {
    /// Item ID.
    pub id: i32,
    /// Fee name.
    pub name: String,
    /// Tax class of fee.
    pub tax_class: String,
    /// Tax status of fee. Options: taxable and none.
    pub tax_status: TaxStatus,
    /// Line total (after discounts).
    pub total: String,
    /// Line total tax (after discounts).
    pub total_tax: String,
    /// Line taxes.
    pub taxes: Vec<OrderTax>,
    /// Meta data.
    pub meta_data: Vec<MetaData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderCouponLineProperties {
    /// Item ID.
    pub id: i32,
    /// Coupon code.
    pub code: String,
    /// Discount total.
    pub discount: String,
    /// Discount total tax.
    pub discount_tax: String,
    /// Meta data.
    pub meta_data: Vec<MetaData>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderRefundProperties {
    /// Refund ID.
    pub id: i32,
    /// Refund reason.
    pub reason: String,
    /// Refund total.
    pub total: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderTax {
    /// Item ID.
    pub id: i32,
    /// Tax rate code.
    pub rate_code: String,
    /// Tax rate ID.
    pub rate_id: String,
    /// Tax rate label.
    pub label: String,
    /// Show if is a compound tax rate.
    pub compound: bool,
    /// Tax total (not including shipping taxes).
    pub tax_total: String,
    /// Shipping tax total.
    pub shipping_tax_total: String,
    /// Meta data.
    pub meta_data: Vec<MetaData>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TaxStatus {
    Taxable,
    None,
}

use crate::controllers::refunds::{NoAmount, NoItems, RefundCreate, RefundCreateBuilder};

use super::MetaData;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Refund {
    /// Unique identifier for the resource.
    pub id: i32,
    /// The date the order refund was created, in the site's timezone.
    pub date_created: NaiveDateTime,
    /// The date the order refund was created, as GMT.
    pub date_created_gmt: NaiveDateTime,
    /// Total refund amount. Optional. If this parameter is provided, it will take precedence over line item totals, even when total of line items does not matches with this amount.
    pub amount: Option<String>,
    /// Reason for refund.
    pub reason: String,
    /// User ID of user who created the refund.    
    pub refunded_by: i32,
    /// If the payment was refunded via the API. See api_refund.    
    pub refunded_payment: bool,
    /// Meta data.    
    pub meta_data: Vec<MetaData>,
    /// Line items data.
    pub line_items: Vec<OrderRefundLineItem>,
    // When true, the payment gateway API is used to generate the refund. Default is true.
    // pub api_refund:	bool,
    // When true, the selected line items are restocked Default is true.
    // pub api_restock: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderRefundLineItem {
    /// Item ID
    pub id: i32,
    /// Product name.
    pub name: String,
    /// Product ID.
    pub product_id: i32,
    /// Variation ID, if applicable.    
    pub variation_id: Option<i32>,
    /// Quantity ordered.
    pub quantity: i32,
    /// Tax class of product.
    pub tax_class: i32,
    /// Line subtotal (before discounts).
    pub subtotal: String,
    /// Line subtotal tax (before discounts).
    pub subtotal_tax: String,
    /// Line total (after discounts).    
    pub total: String,
    /// Line total tax (after discounts).
    pub total_tax: String,
    /// Line taxes.    
    pub taxes: Vec<OrderRefundLineItemTaxesProperties>,
    /// Meta data.
    pub meta_data: Vec<MetaData>,
    /// Product SKU.    
    pub sku: String,
    /// Product price.    
    pub price: String,
    // The amount to refund for this line item, excluding taxes.
    // pub refund_total: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderRefundLineItemTaxesProperties {
    /// Tax rate ID.
    pub id: i32,
    /// Tax total.
    pub total: String,
    /// Tax subtotal.
    pub subtotal: String,
    // The amount to refund for this tax.
    // pub refund_total: f64,
}
impl Refund {
    pub fn create() -> RefundCreateBuilder<NoAmount, NoItems> {
        RefundCreate::new()
    }
}

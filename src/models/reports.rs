use crate::controllers::Entity;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub slug: String,
    pub description: String,
}
impl Entity for Report {
    fn endpoint() -> String {
        String::from("reports/")
    }

    fn child_endpoint(parent_id: i32) -> String {
        let _ = parent_id;
        String::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaleReport {
    /// Gross sales in the period.
    pub total_sales: String,
    /// Net sales in the period.
    pub net_sales: String,
    /// Average net daily sales.
    pub average_sales: String,
    /// Total of orders placed.
    pub total_orders: i32,
    /// Total of items purchased.
    pub total_items: i32,
    /// Total charged for taxes.
    pub total_tax: String,
    /// Total charged for shipping.
    pub total_shipping: String,
    /// Total of refunded orders.
    pub total_refunds: i32,
    /// Total of coupons used.
    pub total_discount: String,
    /// Group type.
    pub totals_grouped_by: String,
    /// Totals.
    pub totals: std::collections::HashMap<NaiveDate, Total>,
    pub total_customers: i32,
}
impl Entity for SaleReport {
    fn endpoint() -> String {
        String::from("reports/sales/")
    }

    fn child_endpoint(parent_id: i32) -> String {
        let _ = parent_id;
        String::new()
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Total {
    pub sales: String,
    pub orders: i32,
    pub items: i32,
    pub tax: String,
    pub shipping: String,
    pub discount: String,
    pub customers: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopSellersReport {
    /// Product title.
    pub name: String,
    /// Product ID.
    pub product_id: i32,
    /// Total number of purchases.
    pub quantity: i32,
}

impl Entity for TopSellersReport {
    fn endpoint() -> String {
        String::from("reports/top_sellers/")
    }

    fn child_endpoint(parent_id: i32) -> String {
        let _ = parent_id;
        String::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportOrdersTotals {
    pub slug: String,
    pub name: String,
    pub total: i32,
}
impl Entity for ReportOrdersTotals {
    fn endpoint() -> String {
        String::from("reports/orders/totals/")
    }

    fn child_endpoint(parent_id: i32) -> String {
        let _ = parent_id;
        String::new()
    }
}

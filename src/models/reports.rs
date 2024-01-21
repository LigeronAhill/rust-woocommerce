use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub slug: String,
    pub description: String,
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
// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "snake_case")]
pub enum Period {
    Week,
    Month,
    LastMonth,
    Year,
}
impl std::fmt::Display for Period {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Period::Week => write!(f, "week"),
            Period::Month => write!(f, "month"),
            Period::LastMonth => write!(f, "last_month"),
            Period::Year => write!(f, "year"),
        }
    }
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportTotals {
    pub slug: String,
    pub name: String,
    pub total: i32,
}

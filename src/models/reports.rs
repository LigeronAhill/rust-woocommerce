use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
/// ```rust
///     let client = ApiClient::from_env()?;
///     let reports = client.list_all::<Report>(Entity::Report).await?;
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub slug: String,
    pub description: String,
}

/// ```rust
///     let client = ApiClient::from_env()?;
///     let result_with_period = client
///         .retrieve_sales_report_with_period(Period::Week)
///         .await?;
///     let result_with_dates = client
///         .retrieve_sales_report_with_min_max_dates("2023-12-01", "2023-12-31")
///         .await?;
/// ```
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

/// ```rust
///     let client = ApiClient::from_env()?;
///     let result_with_period = client
///         .retrieve_top_sellers_report_with_period(Period::Week)
///         .await?;
///     let result_with_dates = client
///         .retrieve_top_sellers_report_with_min_max_dates("2023-12-01", "2023-12-31")
///         .await?;
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopSellersReport {
    /// Product title.
    pub name: String,
    /// Product ID.
    pub product_id: i32,
    /// Total number of purchases.
    pub quantity: i32,
}

/// ```rust
///     let client = ApiClient::from_env()?;
///     let coupons: Vec<ReportTotals> = client.list_all(Entity::ReportCouponsTotal).await?;
///     let customers: Vec<ReportTotals> =
///         client.list_all(Entity::ReportCustomersTotal).await?;
///     let orders: Vec<ReportTotals> = client.list_all(Entity::ReportOrdersTotal).await?;
///     let products: Vec<ReportTotals> =
///         client.list_all(Entity::ReportProductsTotal).await?;
///     let reviews: Vec<ReportTotals> = client.list_all(Entity::ReportReviewsTotal).await?;
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportTotals {
    pub slug: String,
    pub name: String,
    pub total: i32,
}

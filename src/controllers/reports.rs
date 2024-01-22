use crate::reports::TopSellersReport;
use crate::Result;
use crate::{
    reports::{Period, SaleReport},
    ApiClient,
};

impl ApiClient {
    /// Retrieves and views a sales report for the specified period.
    ///
    /// # Arguments
    ///
    /// * `period` - A `Period` object indicating the time period for the sales report.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<SaleReport>>` - A result containing a vector of `SaleReport` objects if the operation is successful, or an error.
    ///
    /// # Example
    ///
    /// ```rust
    /// let result_with_period = client
    ///     .retrieve_sales_report_with_period(Period::Week)
    ///     .await?;
    /// ```
    pub async fn retrieve_sales_report_with_period(
        &self,
        period: Period,
    ) -> Result<Vec<SaleReport>> {
        let uri = format!("{}reports/sales?period={period}", self.base_url());
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
        match serde_json::from_value::<Vec<SaleReport>>(response.clone()) {
            Ok(r) => Ok(r),
            Err(e) => Err(e.into()),
        }
    }
    /// Retrieves and views a sales report for the specified date range.
    ///
    /// # Arguments
    ///
    /// * `date_min` - A string representing the minimum date for the sales report need to be in the YYYY-MM-DD format..
    /// * `date_max` - A string representing the maximum date for the sales report need to be in the YYYY-MM-DD format..
    ///
    /// # Returns
    ///
    /// * `Result<Vec<SaleReport>>` - A result containing a vector of `SaleReport` objects if the operation is successful, or an error.
    ///
    /// # Example
    ///
    /// ```rust
    /// let result_with_dates = client
    ///     .retrieve_sales_report_with_min_max_dates("2023-12-01", "2023-12-31")
    ///     .await?;
    /// ```
    pub async fn retrieve_sales_report_with_min_max_dates(
        &self,
        date_min: impl Into<String>,
        date_max: impl Into<String>,
    ) -> Result<Vec<SaleReport>> {
        let uri = format!(
            "{}reports/sales?date_min={}&date_max={}",
            self.base_url(),
            date_min.into(),
            date_max.into()
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
        match serde_json::from_value::<Vec<SaleReport>>(response.clone()) {
            Ok(r) => Ok(r),
            Err(e) => Err(e.into()),
        }
    }
    /// Retrieves and views a top sellers report for the specified period.
    ///
    /// # Arguments
    ///
    /// * `period` - A `Period` object indicating the time period for the sales report.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<TopSellersReport>>` - A result containing a vector of `TopSellersReport` objects if the operation is successful, or an error.
    ///
    /// # Example
    ///
    /// ```rust
    /// let result_with_period = client
    ///     .retrieve_top_sellers_report_with_period(Period::Week)
    ///     .await?;
    /// ```
    pub async fn retrieve_top_sellers_report_with_period(
        &self,
        period: Period,
    ) -> Result<Vec<TopSellersReport>> {
        let uri = format!("{}reports/top_sellers?period={period}", self.base_url());
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
        match serde_json::from_value::<Vec<TopSellersReport>>(response.clone()) {
            Ok(r) => Ok(r),
            Err(e) => Err(e.into()),
        }
    }
    /// Retrieves and views a top sellers report for the specified date range.
    ///
    /// # Arguments
    ///
    /// * `date_min` - A string representing the minimum date for the sales report need to be in the YYYY-MM-DD format..
    /// * `date_max` - A string representing the maximum date for the sales report need to be in the YYYY-MM-DD format..
    ///
    /// # Returns
    ///
    /// * `Result<Vec<TopSellersReport>>` - A result containing a vector of `TopSellersReport` objects if the operation is successful, or an error.
    ///
    /// # Example
    ///
    /// ```rust
    /// let result_with_dates = client
    ///     .retrieve_top_sellers_report_with_min_max_dates("2023-12-01", "2023-12-31")
    ///     .await?;
    /// ```
    pub async fn retrieve_top_sellers_report_with_min_max_dates(
        &self,
        date_min: impl Into<String>,
        date_max: impl Into<String>,
    ) -> Result<Vec<TopSellersReport>> {
        let uri = format!(
            "{}reports/top_sellers?date_min={}&date_max={}",
            self.base_url(),
            date_min.into(),
            date_max.into()
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
        match serde_json::from_value::<Vec<TopSellersReport>>(response.clone()) {
            Ok(r) => Ok(r),
            Err(e) => Err(e.into()),
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::{
        models::reports::Report,
        reports::{Period, ReportTotals},
        ApiClient, Entity,
    };

    #[tokio::test]
    async fn test_list_all_reports() {
        let client = ApiClient::from_env().unwrap();
        let reports = client.list_all::<Report>(Entity::Report).await.unwrap();
        assert!(!reports.is_empty())
    }
    #[tokio::test]
    async fn test_retrieve_sales_reports() {
        let client = ApiClient::from_env().unwrap();
        let result_with_period = client
            .retrieve_sales_report_with_period(Period::Week)
            .await
            .unwrap();
        assert!(!result_with_period.is_empty());
        let result_with_dates = client
            .retrieve_sales_report_with_min_max_dates("2023-12-01", "2023-12-31")
            .await
            .unwrap();
        assert!(!result_with_dates.is_empty());
    }
    #[tokio::test]
    async fn test_retrieve_top_sellers_reports() {
        let client = ApiClient::from_env().unwrap();
        let result_with_period = client
            .retrieve_top_sellers_report_with_period(Period::Week)
            .await
            .unwrap();
        assert!(result_with_period.is_empty());
        let result_with_dates = client
            .retrieve_top_sellers_report_with_min_max_dates("2023-12-01", "2023-12-31")
            .await
            .unwrap();
        assert!(result_with_dates.is_empty());
    }
    #[tokio::test]
    async fn test_retrieve_report_totals() {
        let client = ApiClient::from_env().unwrap();
        let coupons: Vec<ReportTotals> = client.list_all(Entity::ReportCouponsTotal).await.unwrap();
        assert!(!coupons.is_empty());
        let customers: Vec<ReportTotals> =
            client.list_all(Entity::ReportCustomersTotal).await.unwrap();
        assert!(!customers.is_empty());
        let orders: Vec<ReportTotals> = client.list_all(Entity::ReportOrdersTotal).await.unwrap();
        assert!(!orders.is_empty());
        let products: Vec<ReportTotals> =
            client.list_all(Entity::ReportProductsTotal).await.unwrap();
        assert!(!products.is_empty());
        let reviews: Vec<ReportTotals> = client.list_all(Entity::ReportReviewsTotal).await.unwrap();
        assert!(!reviews.is_empty());
    }
}

use crate::Result;
pub mod coupons;
pub mod customers;
pub mod data;
pub mod entities;
pub mod order_notes;
pub mod orders;
pub mod payment_gateways;
pub mod product_attribute_terms;
pub mod product_attributes;
pub mod product_categories;
pub mod product_reviews;
pub mod product_shipping_classes;
pub mod product_tags;
pub mod product_variations;
pub mod products;
pub mod refunds;
pub mod reports;
pub mod settings;
pub mod shipping_methods;
pub mod shipping_zone_locations;
pub mod shipping_zone_methods;
pub mod shipping_zones;
pub mod system_status;
pub mod system_status_tools;
pub mod tax_classes;
pub mod tax_rates;
pub mod webhooks;
/// Struct representing the configured API client.
/// # Example from environment
/// ```
/// let client = rust_woocommerce::ApiClient::from_env().unwrap();
/// let result = client
///     .list_all::<Attribute>(Entity::ProductAttribute)
///     .await?;
/// ```
/// # Example with builder
/// ```
/// let site_url = "google.com";
/// let customer_key = "super6secret9";
/// let customer_secret = "customer4secret2";
/// let client = rust_woocommerce::ApiClient::builder()
///     .auth(customer_key, customer_secret)
///     .site_url(site_url)
///     .build();
/// ```
pub struct ApiClient {
    auth: Auth,
    base_url: String,
    client: reqwest::Client,
}
/// Struct representing the authentication credentials.
pub struct Auth {
    ck: String,
    cs: String,
}
/// Struct for building an API client with authentication and base URL.
pub struct ApiClientBuilder<A, B> {
    auth: A,
    base_url: B,
    client: reqwest::Client,
}
impl<A, B> ApiClientBuilder<A, B> {
    /// Sets the authentication credentials for the API client.
    pub fn auth(
        self,
        ck: impl Into<String>,
        cs: impl Into<String>,
    ) -> ApiClientBuilder<WithAuth, B> {
        let Self {
            base_url, client, ..
        } = self;
        let auth = Auth {
            ck: ck.into(),
            cs: cs.into(),
        };
        ApiClientBuilder {
            auth: WithAuth(auth),
            base_url,
            client,
        }
    }
    /// Sets the site URL to be used as the base URL for the API client.
    pub fn site_url(self, url: impl Into<String>) -> ApiClientBuilder<A, WithBaseUrl> {
        let Self { auth, client, .. } = self;
        let base_url = format!("{}/wp-json/wc/v3/", url.into());
        ApiClientBuilder {
            auth,
            base_url: WithBaseUrl(base_url),
            client,
        }
    }
}
impl ApiClientBuilder<WithAuth, WithBaseUrl> {
    /// Builds the configured API client with the provided settings.
    pub fn build(self) -> ApiClient {
        let auth = self.auth.0;
        let base_url = self.base_url.0;
        ApiClient {
            auth,
            base_url,
            client: self.client,
        }
    }
}
/// Struct representing the absence of authentication.
pub struct NoAuth;
/// Struct representing the absence of a base URL.
pub struct NoBaseUrl;
/// Wrapper struct for authentication-enabled API client builder.
pub struct WithAuth(Auth);
/// Wrapper struct for base-URL-enabled API client builder.
pub struct WithBaseUrl(String);
impl ApiClient {
    pub fn from_env() -> Result<Self> {
        let ck = std::env::var("WOO_CK")?;
        let cs = std::env::var("WOO_CS")?;
        let auth = Auth { ck, cs };
        let base_url_raw = std::env::var("BASE_URL")?;
        let base_url = format!("{base_url_raw}/wp-json/wc/v3/");
        let client = reqwest::Client::builder().gzip(true).build()?;

        Ok(Self {
            auth,
            base_url,
            client,
        })
    }
    pub fn builder() -> ApiClientBuilder<NoAuth, NoBaseUrl> {
        let client = reqwest::Client::builder().gzip(true).build().unwrap();
        ApiClientBuilder {
            auth: NoAuth,
            base_url: NoBaseUrl,
            client,
        }
    }
    pub fn ck(&self) -> String {
        self.auth.ck.clone()
    }
    pub fn cs(&self) -> String {
        self.auth.cs.clone()
    }
    pub fn client(&self) -> reqwest::Client {
        self.client.clone()
    }
    pub fn base_url(&self) -> String {
        self.base_url.clone()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn api_client_from_env_test() {
        let woo_api_client = ApiClient::from_env().unwrap();
        let system_status_uri = format!("{}system_status", woo_api_client.base_url);
        let system_status = woo_api_client
            .client
            .get(&system_status_uri)
            .basic_auth(&woo_api_client.auth.ck, Some(&woo_api_client.auth.cs))
            .send()
            .await
            .unwrap();
        assert!(system_status.status().is_success())
    }
    #[tokio::test]
    async fn api_client_new_test() {
        let woo_api_client = ApiClient::builder()
            .auth("wrong_ck", "wrong_cs")
            .site_url("https://google.com")
            .build();
        let system_status_uri = format!("{}system_status", woo_api_client.base_url);
        let system_status = woo_api_client
            .client
            .get(&system_status_uri)
            .basic_auth(&woo_api_client.auth.ck, Some(&woo_api_client.auth.cs))
            .send()
            .await
            .unwrap();
        assert!(system_status.status().is_client_error())
    }
}

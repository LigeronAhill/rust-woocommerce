use crate::Result;
pub mod coupons;
pub mod customers;
pub mod data;
pub mod entities;
pub mod order_notes;
pub mod orders;
pub struct ApiClient {
    auth: Auth,
    base_url: String,
    client: reqwest::Client,
}
pub struct Auth {
    ck: String,
    cs: String,
}
pub struct ApiClientBuilder<A, B> {
    auth: A,
    base_url: B,
    client: reqwest::Client,
}
impl<A, B> ApiClientBuilder<A, B> {
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
pub struct NoAuth;
pub struct NoBaseUrl;
pub struct WithAuth(Auth);
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

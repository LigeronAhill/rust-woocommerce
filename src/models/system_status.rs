use serde::{Deserialize, Serialize};
/// ```rust
/// #[cfg(test)]
/// mod tests {
///     use crate::{system_status::SystemStatus, ApiClient};
///
///     #[tokio::test]
///     async fn test_list_all_system_status() {
///         let client = ApiClient::from_env().unwrap();
///         let result = client
///             .list_all::<SystemStatus>(crate::Entity::SystemStatus)
///             .await
///             .unwrap();
///         assert_eq!(result.len(), 1);
///     }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStatus {
    /// Environment.
    pub environment: Environment,
    /// Database.
    pub database: Database,
    /// Active plugins.
    pub active_plugins: Vec<Plugin>,
    /// Inactive plugins.
    pub inactive_plugins: Option<Vec<Plugin>>,
    /// Theme.
    pub theme: Theme,
    /// Settings.
    pub settings: Settings,
    /// Security.
    pub security: Security,
    /// WooCommerce pages.
    pub pages: Vec<Page>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    /// Home URL.
    pub home_url: String,
    /// Site URL.
    pub site_url: String,
    /// WooCommerce version.
    pub wc_version: Option<String>,
    /// Log directory.
    pub log_directory: String,
    /// Is log directory writable?
    pub log_directory_writable: bool,
    /// WordPress version.
    pub wp_version: String,
    /// Is WordPress multisite?
    pub wp_multisite: bool,
    /// WordPress memory limit.
    pub wp_memory_limit: i64,
    /// Is WordPress debug mode active?
    pub wp_debug_mode: bool,
    /// Are WordPress cron jobs enabled?
    pub wp_cron: bool,
    /// WordPress language.
    pub language: String,
    /// Server info.
    pub server_info: String,
    /// PHP version.
    pub php_version: String,
    /// PHP post max size.
    pub php_post_max_size: i64,
    /// PHP max execution time.
    pub php_max_execution_time: i64,
    /// PHP max input vars.
    pub php_max_input_vars: i64,
    /// cURL version.
    pub curl_version: String,
    /// Is SUHOSIN installed?
    pub suhosin_installed: bool,
    /// Max upload size.
    pub max_upload_size: i64,
    /// MySQL version.
    pub mysql_version: String,
    /// Default timezone.
    pub default_timezone: String,
    /// Is fsockopen/cURL enabled?
    pub fsockopen_or_curl_enabled: bool,
    /// Is SoapClient class enabled?
    pub soapclient_enabled: bool,
    /// Is DomDocument class enabled?
    pub domdocument_enabled: bool,
    /// Is GZip enabled?
    pub gzip_enabled: bool,
    /// Is mbstring enabled?
    pub mbstring_enabled: bool,
    /// Remote POST successful?
    pub remote_post_successful: bool,
    /// Remote POST response.
    pub remote_post_response: serde_json::Value,
    /// Remote GET successful?
    pub remote_get_successful: bool,
    /// Remote GET response.
    pub remote_get_response: serde_json::Value,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Database {
    /// WC database version.
    pub wc_database_version: String,
    /// Database prefix.
    pub database_prefix: String,
    /// MaxMind GeoIP database.
    pub maxmind_geoip_database: String,
    /// Database tables.
    pub database_tables: DatabaseTables,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseTables {
    pub woocommerce: serde_json::Value,
    pub other: serde_json::Value,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plugin {
    pub plugin: String,
    pub name: String,
    pub version: String,
    pub version_latest: String,
    pub url: String,
    pub author_name: String,
    pub author_url: String,
    pub network_activated: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    /// Theme name.
    pub name: String,
    /// Theme version.
    pub version: String,
    /// Latest version of theme.
    pub version_latest: String,
    /// Theme author URL.
    pub author_url: String,
    /// Is this theme a child theme?
    pub is_child_theme: bool,
    /// Does the theme declare WooCommerce support?
    pub has_woocommerce_support: bool,
    /// Does the theme have a woocommerce.php file?
    pub has_woocommerce_file: bool,
    /// Does this theme have outdated templates?
    pub has_outdated_templates: bool,
    /// Template overrides.
    pub overrides: Vec<serde_json::Value>,
    /// Parent theme name.
    pub parent_name: String,
    /// Parent theme version.
    pub parent_version: String,
    /// Parent theme author URL.
    pub parent_author_url: String,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Settings {
    /// REST API enabled?
    pub api_enabled: bool,
    /// SSL forced?
    pub force_ssl: bool,
    /// Currency
    pub currency: String,
    /// Currency symbol.
    pub currency_symbol: String,
    /// Currency position.
    pub currency_position: String,
    /// Thousand separator.
    pub thousand_separator: String,
    /// Decimal separator.
    pub decimal_separator: String,
    /// Number of decimals.
    pub number_of_decimals: i64,
    /// Geolocation enabled?
    pub geolocation_enabled: bool,
    /// Taxonomy terms for product/order statuses.
    pub taxonomies: Taxonomies,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Taxonomies {
    pub external: String,
    pub grouped: String,
    pub simple: String,
    pub variable: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Security {
    /// Is the connection to your store secure?
    pub secure_connection: bool,
    /// Hide errors from visitors?
    pub hide_errors: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Page {
    pub page_name: String,
    pub page_id: String,
    pub page_set: bool,
    pub page_exists: bool,
    pub page_visible: bool,
    pub shortcode: String,
    pub block: String,
    pub shortcode_required: bool,
    pub shortcode_present: bool,
    pub block_present: bool,
    pub block_required: bool,
}

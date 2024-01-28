use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::controllers::customers::{CreateCustomerBuilder, NoEmail, UpdateCustomerBuilder};

use super::MetaData;
/// ```rust
/// #[cfg(test)]
/// mod tests {
///     use crate::{customers::Customer, ApiClient, Entity};
///
///     #[tokio::test]
///     async fn test_list_all_customers() {
///         let client = ApiClient::from_env().unwrap();
///         let result: Vec<Customer> = client.list_all(Entity::Customer).await.unwrap();
///         assert!(!result.is_empty());
///     }
///     #[tokio::test]
///     async fn test_retrieve_customer() {
///         let id = 4;
///         let client = ApiClient::from_env().unwrap();
///         let result: Customer = client.retrieve(Entity::Customer, id).await.unwrap();
///         assert_eq!(id, result.id);
///     }
///     #[tokio::test]
///     async fn test_search_customer() {
///         let search_string = "Хасаншина";
///         let client = ApiClient::from_env().unwrap();
///         let result: Vec<Customer> = client
///             .search(Entity::Customer, search_string)
///             .await
///             .unwrap();
///         assert_eq!(result.first().unwrap().id, 4);
///     }
///     #[tokio::test]
///     async fn test_create_delete_customer() {
///         let customer_to_create = Customer::create()
///             .email("info@google.com")
///             .first_name("John")
///             .last_name("Doe")
///             .username("johny")
///             .password("johnyspassword")
///             .billing_first_name("John")
///             .billing_last_name("Doe")
///             .billing_company("Umbrella Corp")
///             .billing_address_1("Leninsky street")
///             .billing_address_2("дом 4")
///             .billing_city("Moscow")
///             .billing_state("Moscow")
///             .billing_postcode("117000")
///             .billing_country("Russia")
///             .billing_email("info@google.com")
///             .billing_phone("+79996669966")
///             .shipping_first_name("John")
///             .shipping_last_name("Doe")
///             .shipping_company("Umbrella Corp")
///             .shipping_address_1("Leninsky street")
///             .shipping_address_2("дом 4")
///             .shipping_city("Moscow")
///             .shipping_state("Moscow")
///             .shipping_postcode("117000")
///             .shipping_country("Russia")
///             .meta_data("test-meta", "test_value")
///             .build();
///         let client = ApiClient::from_env().unwrap();
///         let created: Customer = client
///             .create(Entity::Customer, customer_to_create)
///             .await
///             .unwrap();
///         assert_eq!(created.email, "info@google.com");
///         let id = created.id;
///         let _deleted: Customer = client.delete(Entity::Customer, id).await.unwrap();
///     }
///     #[tokio::test]
///     async fn test_update_customer() {
///         let id = 4;
///         let update = Customer::update().email("four@google.com").build();
///         let client = ApiClient::from_env().unwrap();
///         let updated: Customer = client.update(Entity::Customer, id, update).await.unwrap();
///         assert_eq!(updated.id, id)
///     }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    /// Unique identifier for the resource
    pub id: i32,
    /// The date the customer was created, in the site's timezone.
    pub date_created: NaiveDateTime,
    /// The date the customer was created, as GMT
    pub date_modified: NaiveDateTime,
    /// The date the customer was last modified, as GMT.
    pub date_modified_gmt: NaiveDateTime,
    /// The email address for the customer
    pub email: String,
    /// Customer first name.    
    pub first_name: String,
    /// Customer last name.
    pub last_name: String,
    /// Customer role.
    pub role: Role,
    /// Customer login name.
    pub username: String,
    /// List of billing address data.    
    pub billing: Billing,
    /// List of shipping address data.
    pub shipping: Shipping,
    /// Is the customer a paying customer
    pub is_paying_customer: bool,
    /// Avatar URL
    pub avatar_url: String,
    /// Meta data.
    pub meta_data: Vec<MetaData>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Billing {
    /// First name.
    pub first_name: String,
    /// Last name.
    pub last_name: String,
    /// Company name.
    pub company: String,
    /// Address line 1
    pub address_1: String,
    /// Address line 2
    pub address_2: String,
    /// City name.
    pub city: String,
    /// ISO code or name of the state, province or district.
    pub state: String,
    /// Postal code.
    pub postcode: String,
    /// ISO code of the country.
    pub country: String,
    /// Email address.
    pub email: String,
    /// Phone number.
    pub phone: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Shipping {
    /// First name.
    pub first_name: String,
    /// Last name.
    pub last_name: String,
    /// Company name.
    pub company: String,
    /// Address line 1
    pub address_1: String,
    /// Address line 2
    pub address_2: String,
    /// City name.
    pub city: String,
    /// ISO code or name of the state, province or district.
    pub state: String,
    /// Postal code.
    pub postcode: String,
    /// ISO code of the country.
    pub country: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Administrator,
    Editor,
    Author,
    Contributor,
    Subscriber,
    #[default]
    Customer,
    ShopManager,
}
impl Customer {
    pub fn create() -> CreateCustomerBuilder<NoEmail> {
        CreateCustomerBuilder::default()
    }
    pub fn update() -> UpdateCustomerBuilder {
        UpdateCustomerBuilder::default()
    }
}

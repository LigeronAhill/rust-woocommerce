use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    customers::{Billing, Shipping},
    MetaData,
};

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomer {
    /// The email address for the customer
    email: String,
    /// Customer first name.    
    first_name: Option<String>,
    /// Customer last name.
    last_name: Option<String>,
    /// Customer login name.
    username: Option<String>,
    // Customer password.
    password: Option<String>,
    /// List of billing address data.    
    billing: Option<Billing>,
    /// List of shipping address data.
    shipping: Option<Shipping>,
    /// Meta data.
    meta_data: Option<Vec<MetaData>>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NoEmail;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WithEmail(String);
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateCustomerBuilder<T> {
    /// The email address for the customer
    email: T,
    /// Customer first name.    
    first_name: Option<String>,
    /// Customer last name.
    last_name: Option<String>,
    /// Customer login name.
    username: Option<String>,
    // Customer password.
    password: Option<String>,
    /// List of billing address data.    
    billing: Option<Billing>,
    /// List of shipping address data.
    shipping: Option<Shipping>,
    /// Meta data.
    meta_data: Option<Vec<MetaData>>,
}

impl CreateCustomerBuilder<WithEmail> {
    pub fn build(self) -> CreateCustomer {
        CreateCustomer {
            email: self.email.0,
            first_name: self.first_name,
            last_name: self.last_name,
            username: self.username,
            password: self.password,
            billing: self.billing,
            shipping: self.shipping,
            meta_data: self.meta_data,
        }
    }
}
impl<T> CreateCustomerBuilder<T> {
    /// The email address for the customer
    pub fn email(self, email: impl Into<String>) -> CreateCustomerBuilder<WithEmail> {
        CreateCustomerBuilder {
            email: WithEmail(email.into()),
            first_name: self.first_name,
            last_name: self.last_name,
            username: self.username,
            password: self.password,
            billing: self.billing,
            shipping: self.shipping,
            meta_data: self.meta_data,
        }
    }
    /// Customer first name.    
    pub fn first_name(mut self, first_name: impl Into<String>) -> Self {
        let _ = self.first_name.insert(first_name.into());
        self
    }
    /// Customer last name.
    pub fn last_name(mut self, last_name: impl Into<String>) -> Self {
        let _ = self.last_name.insert(last_name.into());
        self
    }
    /// Customer login name.
    pub fn username(mut self, username: impl Into<String>) -> Self {
        let _ = self.username.insert(username.into());
        self
    }
    // Customer password.
    pub fn password(mut self, password: impl Into<String>) -> Self {
        let _ = self.password.insert(password.into());
        self
    }
    /// billing first name.
    pub fn billing_first_name(mut self, first_name: impl Into<String>) -> Self {
        self.billing.get_or_insert(Billing::default()).first_name = first_name.into();
        self
    }
    /// billing last name.
    pub fn billing_last_name(mut self, last_name: impl Into<String>) -> Self {
        self.billing.get_or_insert(Billing::default()).last_name = last_name.into();
        self
    }
    /// billing company name.
    pub fn billing_company(mut self, company: impl Into<String>) -> Self {
        self.billing.get_or_insert(Billing::default()).company = company.into();
        self
    }
    /// billing address line 1
    pub fn billing_address_1(mut self, address_1: impl Into<String>) -> Self {
        self.billing.get_or_insert(Billing::default()).address_1 = address_1.into();
        self
    }
    /// billing address line 2
    pub fn billing_address_2(mut self, address_2: impl Into<String>) -> Self {
        self.billing.get_or_insert(Billing::default()).address_2 = address_2.into();
        self
    }
    /// billing city name.
    pub fn billing_city(mut self, city: impl Into<String>) -> Self {
        self.billing.get_or_insert(Billing::default()).city = city.into();
        self
    }
    /// billing ISO code or name of the state, province or district.
    pub fn billing_state(mut self, state: impl Into<String>) -> Self {
        self.billing.get_or_insert(Billing::default()).state = state.into();
        self
    }
    /// billing postal code.
    pub fn billing_postcode(mut self, postcode: impl Into<String>) -> Self {
        self.billing.get_or_insert(Billing::default()).postcode = postcode.into();
        self
    }
    /// billing ISO code of the country.
    pub fn billing_country(mut self, country: impl Into<String>) -> Self {
        self.billing.get_or_insert(Billing::default()).country = country.into();
        self
    }
    /// billing email address.
    pub fn billing_email(mut self, email: impl Into<String>) -> Self {
        self.billing.get_or_insert(Billing::default()).email = email.into();
        self
    }
    /// billing phone number.
    pub fn billing_phone(mut self, phone: impl Into<String>) -> Self {
        self.billing.get_or_insert(Billing::default()).phone = phone.into();
        self
    }
    /// shipping first name.
    pub fn shipping_first_name(mut self, first_name: impl Into<String>) -> Self {
        self.shipping.get_or_insert(Shipping::default()).first_name = first_name.into();
        self
    }
    /// shipping last name.
    pub fn shipping_last_name(mut self, last_name: impl Into<String>) -> Self {
        self.shipping.get_or_insert(Shipping::default()).last_name = last_name.into();
        self
    }
    /// shipping company name.
    pub fn shipping_company(mut self, company: impl Into<String>) -> Self {
        self.shipping.get_or_insert(Shipping::default()).company = company.into();
        self
    }
    /// shipping address line 1
    pub fn shipping_address_1(mut self, address_1: impl Into<String>) -> Self {
        self.shipping.get_or_insert(Shipping::default()).address_1 = address_1.into();
        self
    }
    /// shipping address line 2
    pub fn shipping_address_2(mut self, address_2: impl Into<String>) -> Self {
        self.shipping.get_or_insert(Shipping::default()).address_2 = address_2.into();
        self
    }
    /// shipping city name.
    pub fn shipping_city(mut self, city: impl Into<String>) -> Self {
        self.shipping.get_or_insert(Shipping::default()).city = city.into();
        self
    }
    /// shipping ISO code or name of the state, province or district.
    pub fn shipping_state(mut self, state: impl Into<String>) -> Self {
        self.shipping.get_or_insert(Shipping::default()).state = state.into();
        self
    }
    /// shipping postal code.
    pub fn shipping_postcode(mut self, postcode: impl Into<String>) -> Self {
        self.shipping.get_or_insert(Shipping::default()).postcode = postcode.into();
        self
    }
    /// shipping ISO code of the country.
    pub fn shipping_country(mut self, country: impl Into<String>) -> Self {
        self.shipping.get_or_insert(Shipping::default()).country = country.into();
        self
    }
    /// Meta data.
    pub fn meta_data(mut self, key: impl Into<String>, value: impl serde::Serialize) -> Self {
        self.meta_data.get_or_insert(vec![]).push(MetaData {
            id: None,
            key: key.into(),
            value: serde_json::json!(value),
        });
        self
    }
}
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCustomer {
    /// Unique identifier for the resource
    id: Option<i32>,
    /// The email address for the customer
    email: Option<String>,
    /// Customer first name.    
    first_name: Option<String>,
    /// Customer last name.
    last_name: Option<String>,
    /// Customer login name.
    username: Option<String>,
    // Customer password.
    password: Option<String>,
    /// List of billing address data.    
    billing: Option<Billing>,
    /// List of shipping address data.
    shipping: Option<Shipping>,
    /// Meta data.
    meta_data: Option<Vec<MetaData>>,
}
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BillingUpdate {
    /// First name.
    first_name: Option<String>,
    /// Last name.
    last_name: Option<String>,
    /// Company name.
    company: Option<String>,
    /// Address line 1
    address_1: Option<String>,
    /// Address line 2
    address_2: Option<String>,
    /// City name.
    city: Option<String>,
    /// ISO code or name of the state, province or district.
    state: Option<String>,
    /// Postal code.
    postcode: Option<String>,
    /// ISO code of the country.
    country: Option<String>,
    /// Email address.
    email: Option<String>,
    /// Phone number.
    phone: Option<String>,
}
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ShippingUpdate {
    /// First name.
    first_name: Option<String>,
    /// Last name.
    last_name: Option<String>,
    /// Company name.
    company: Option<String>,
    /// Address line 1
    address_1: Option<String>,
    /// Address line 2
    address_2: Option<String>,
    /// City name.
    city: Option<String>,
    /// ISO code or name of the state, province or district.
    state: Option<String>,
    /// Postal code.
    postcode: Option<String>,
    /// ISO code of the country.
    country: Option<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateCustomerBuilder {
    /// Unique identifier for the resource
    id: Option<i32>,
    /// The email address for the customer
    email: Option<String>,
    /// Customer first name.    
    first_name: Option<String>,
    /// Customer last name.
    last_name: Option<String>,
    /// Customer login name.
    username: Option<String>,
    // Customer password.
    password: Option<String>,
    /// List of billing address data.    
    billing: Option<Billing>,
    /// List of shipping address data.
    shipping: Option<Shipping>,
    /// Meta data.
    meta_data: Option<Vec<MetaData>>,
}
impl UpdateCustomerBuilder {
    pub fn build(&mut self) -> UpdateCustomer {
        UpdateCustomer {
            id: self.id,
            email: self.email.clone(),
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            username: self.username.clone(),
            password: self.password.clone(),
            billing: self.billing.clone(),
            shipping: self.shipping.clone(),
            meta_data: self.meta_data.clone(),
        }
    }
    /// Unique identifier for the resource
    pub fn id(&mut self, id: i32) -> &mut Self {
        let _ = self.id.insert(id);
        self
    }
    /// The email address for the customer
    pub fn email(&mut self, email: impl Into<String>) -> &mut Self {
        let _ = self.email.insert(email.into());
        self
    }
    /// Customer first name.    
    pub fn first_name(&mut self, first_name: impl Into<String>) -> &mut Self {
        let _ = self.first_name.insert(first_name.into());
        self
    }
    /// Customer last name.
    pub fn last_name(&mut self, last_name: impl Into<String>) -> &mut Self {
        let _ = self.last_name.insert(last_name.into());
        self
    }
    /// Customer login name.
    pub fn username(&mut self, username: impl Into<String>) -> &mut Self {
        let _ = self.username.insert(username.into());
        self
    }
    // Customer password.
    pub fn password(&mut self, password: impl Into<String>) -> &mut Self {
        let _ = self.password.insert(password.into());
        self
    }
    /// billing first name.
    pub fn billing_first_name(&mut self, first_name: impl Into<String>) -> &mut Self {
        self.billing.get_or_insert(Billing::default()).first_name = first_name.into();
        self
    }
    /// billing last name.
    pub fn billing_last_name(&mut self, last_name: impl Into<String>) -> &mut Self {
        self.billing.get_or_insert(Billing::default()).last_name = last_name.into();
        self
    }
    /// billing company name.
    pub fn billing_company(&mut self, company: impl Into<String>) -> &mut Self {
        self.billing.get_or_insert(Billing::default()).company = company.into();
        self
    }
    /// billing address line 1
    pub fn billing_address_1(&mut self, address_1: impl Into<String>) -> &mut Self {
        self.billing.get_or_insert(Billing::default()).address_1 = address_1.into();
        self
    }
    /// billing address line 2
    pub fn billing_address_2(&mut self, address_2: impl Into<String>) -> &mut Self {
        self.billing.get_or_insert(Billing::default()).address_2 = address_2.into();
        self
    }
    /// billing city name.
    pub fn billing_city(&mut self, city: impl Into<String>) -> &mut Self {
        self.billing.get_or_insert(Billing::default()).city = city.into();
        self
    }
    /// billing ISO code or name of the state, province or district.
    pub fn billing_state(&mut self, state: impl Into<String>) -> &mut Self {
        self.billing.get_or_insert(Billing::default()).state = state.into();
        self
    }
    /// billing postal code.
    pub fn billing_postcode(&mut self, postcode: impl Into<String>) -> &mut Self {
        self.billing.get_or_insert(Billing::default()).postcode = postcode.into();
        self
    }
    /// billing ISO code of the country.
    pub fn billing_country(&mut self, country: impl Into<String>) -> &mut Self {
        self.billing.get_or_insert(Billing::default()).country = country.into();
        self
    }
    /// billing email address.
    pub fn billing_email(&mut self, email: impl Into<String>) -> &mut Self {
        self.billing.get_or_insert(Billing::default()).email = email.into();
        self
    }
    /// billing phone number.
    pub fn billing_phone(&mut self, phone: impl Into<String>) -> &mut Self {
        self.billing.get_or_insert(Billing::default()).phone = phone.into();
        self
    }
    /// shipping first name.
    pub fn shipping_first_name(&mut self, first_name: impl Into<String>) -> &mut Self {
        self.shipping.get_or_insert(Shipping::default()).first_name = first_name.into();
        self
    }
    /// shipping last name.
    pub fn shipping_last_name(&mut self, last_name: impl Into<String>) -> &mut Self {
        self.shipping.get_or_insert(Shipping::default()).last_name = last_name.into();
        self
    }
    /// shipping company name.
    pub fn shipping_company(&mut self, company: impl Into<String>) -> &mut Self {
        self.shipping.get_or_insert(Shipping::default()).company = company.into();
        self
    }
    /// shipping address line 1
    pub fn shipping_address_1(&mut self, address_1: impl Into<String>) -> &mut Self {
        self.shipping.get_or_insert(Shipping::default()).address_1 = address_1.into();
        self
    }
    /// shipping address line 2
    pub fn shipping_address_2(&mut self, address_2: impl Into<String>) -> &mut Self {
        self.shipping.get_or_insert(Shipping::default()).address_2 = address_2.into();
        self
    }
    /// shipping city name.
    pub fn shipping_city(&mut self, city: impl Into<String>) -> &mut Self {
        self.shipping.get_or_insert(Shipping::default()).city = city.into();
        self
    }
    /// shipping ISO code or name of the state, province or district.
    pub fn shipping_state(&mut self, state: impl Into<String>) -> &mut Self {
        self.shipping.get_or_insert(Shipping::default()).state = state.into();
        self
    }
    /// shipping postal code.
    pub fn shipping_postcode(&mut self, postcode: impl Into<String>) -> &mut Self {
        self.shipping.get_or_insert(Shipping::default()).postcode = postcode.into();
        self
    }
    /// shipping ISO code of the country.
    pub fn shipping_country(&mut self, country: impl Into<String>) -> &mut Self {
        self.shipping.get_or_insert(Shipping::default()).country = country.into();
        self
    }
    /// Meta data.
    pub fn meta_data(&mut self, key: impl Into<String>, value: impl serde::Serialize) -> &mut Self {
        self.meta_data.get_or_insert(vec![]).push(MetaData {
            id: None,
            key: key.into(),
            value: serde_json::json!(value),
        });
        self
    }
}
#[cfg(test)]
mod tests {
    use crate::{customers::Customer, ApiClient, Entity};

    #[tokio::test]
    async fn test_list_all_customers() {
        let client = ApiClient::from_env().unwrap();
        let result: Vec<Customer> = client.list_all(Entity::Customer).await.unwrap();
        assert!(!result.is_empty());
    }
    #[tokio::test]
    async fn test_retrieve_customer() {
        let id = 4;
        let client = ApiClient::from_env().unwrap();
        let result: Customer = client.retrieve(Entity::Customer, id).await.unwrap();
        assert_eq!(id, result.id);
    }
    #[tokio::test]
    async fn test_search_customer() {
        let search_string = "Хасаншина";
        let client = ApiClient::from_env().unwrap();
        let result: Vec<Customer> = client
            .search(Entity::Customer, search_string)
            .await
            .unwrap();
        assert_eq!(result.first().unwrap().id, 4);
    }
    #[tokio::test]
    async fn test_create_delete_customer() {
        let customer_to_create = Customer::create()
            .email("info@google.com")
            .first_name("John")
            .last_name("Doe")
            .username("johny")
            .password("johnyspassword")
            .billing_first_name("John")
            .billing_last_name("Doe")
            .billing_company("Umbrella Corp")
            .billing_address_1("Leninsky street")
            .billing_address_2("дом 4")
            .billing_city("Moscow")
            .billing_state("Moscow")
            .billing_postcode("117000")
            .billing_country("Russia")
            .billing_email("info@google.com")
            .billing_phone("+79996669966")
            .shipping_first_name("John")
            .shipping_last_name("Doe")
            .shipping_company("Umbrella Corp")
            .shipping_address_1("Leninsky street")
            .shipping_address_2("дом 4")
            .shipping_city("Moscow")
            .shipping_state("Moscow")
            .shipping_postcode("117000")
            .shipping_country("Russia")
            .meta_data("test-meta", "test_value")
            .build();
        let client = ApiClient::from_env().unwrap();
        let created: Customer = client
            .create(Entity::Customer, customer_to_create)
            .await
            .unwrap();
        assert_eq!(created.email, "info@google.com");
        let id = created.id;
        let _deleted: Customer = client.delete(Entity::Customer, id).await.unwrap();
    }
    #[tokio::test]
    async fn test_update_customer() {
        let id = 4;
        let update = Customer::update().email("four@google.com").build();
        let client = ApiClient::from_env().unwrap();
        let updated: Customer = client.update(Entity::Customer, id, update).await.unwrap();
        assert_eq!(updated.id, id)
    }
}

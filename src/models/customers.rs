use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::controllers::customers::{CreateCustomerBuilder, NoEmail, UpdateCustomerBuilder};

use super::MetaData;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    /// Unique identifier for the resource
    pub id: i32,
    /// The date the customer was created, in the site's timezone.
    pub date_created: NaiveDateTime,
    /// The date the customer was created, as GMTdate_created_gmt	date-time	The date the customer was created, as GMT.READ-ONLY
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

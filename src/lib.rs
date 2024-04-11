//! # rust-woocommerce
//!
//! `rust-woocommerce` is a library for woocommerce API.

mod controllers;
mod models;
pub use self::controllers::ApiClient;
pub use models::{
    coupons::*, customers::*, data::*, orders::*, product_attributes::*, product_categories::*,
    product_reviews::*, product_variations::*, products::*, reports::*, webhooks::*, BatchObject,
    MetaData,
};
mod config;
pub use config::Config;

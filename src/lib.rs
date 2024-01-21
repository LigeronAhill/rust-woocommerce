//! # rust-woocommerce
//!
//! `rust-woocommerce` is a library for woocommerce API.

pub mod controllers;
mod error;
mod models;
pub use self::controllers::entities::{Entity, SubEntity};
pub use self::controllers::ApiClient;
pub use self::error::{Error, Result};
pub use self::models::BatchObject;
pub use models::*;

use std::fmt::Display;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::controllers::webhooks::{WebhookCreateBuilder, WebhookUpdateBuilder};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Webhook {
    /// Unique identifier for the resource.
    pub id: i32,
    /// A friendly name for the webhook.
    pub name: String,
    /// Webhook status. Options: active, paused and disabled. Default is active.
    pub status: WebhookStatus,
    /// Webhook topic.
    pub topic: String,
    /// Webhook resource.
    pub resource: Resource,
    /// Webhook event.
    pub event: Event,
    /// WooCommerce action names associated with the webhook.
    pub hooks: Vec<String>,
    /// The URL where the webhook payload is delivered.
    pub delivery_url: String,
    /// Secret key used to generate a hash of the delivered webhook and provided in the request headers. This will default is a MD5 hash from the current user's ID
    pub secret: Option<String>,
    /// The date the webhook was created, in the site's timezone.
    pub date_created: NaiveDateTime,
    /// The date the webhook was created, as GMT.
    pub date_created_gmt: NaiveDateTime,
    /// The date the webhook was last modified, in the site's timezone.
    pub date_modified: Option<NaiveDateTime>,
    /// The date the webhook was last modified, as GMT.
    pub date_modified_gmt: Option<NaiveDateTime>,
}
#[derive(Default)]
pub struct NoResource;
#[derive(Default)]
pub struct NoEvent;
#[derive(Default)]
pub struct NoUrl;
impl Webhook {
    pub fn create() -> WebhookCreateBuilder<NoResource, NoEvent, NoUrl> {
        WebhookCreateBuilder::default()
    }
    pub fn update() -> WebhookUpdateBuilder {
        WebhookUpdateBuilder::default()
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WebhookStatus {
    Active,
    Paused,
    Disabled,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Resource {
    Coupon,
    Customer,
    Order,
    Product,
}
impl Display for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Resource::Coupon => write!(f, "coupon"),
            Resource::Customer => write!(f, "customer"),
            Resource::Order => write!(f, "order"),
            Resource::Product => write!(f, "product"),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Event {
    Created,
    Updated,
    Deleted,
    Restored,
}
impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Event::Created => write!(f, "created"),
            Event::Updated => write!(f, "updated"),
            Event::Deleted => write!(f, "deleted"),
            Event::Restored => write!(f, "restored"),
        }
    }
}

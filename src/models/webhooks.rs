use std::fmt::Display;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::controllers::webhooks::{WebhookCreateBuilder, WebhookUpdateBuilder};
/// ```rust
/// #[cfg(test)]
/// mod tests {
///     use crate::{
///         webhooks::{Event, Resource, Webhook, WebhookStatus},
///         ApiClient, BatchObject, Entity,
///     };
///
///     #[tokio::test]
///     async fn test_create_update_bathc_update_delete_webhook() {
///         let client = ApiClient::from_env().unwrap();
///         let create = Webhook::create()
///             .name("test webhook")
///             .status(WebhookStatus::Disabled)
///             .resource(Resource::Customer)
///             .event(Event::Deleted)
///             .delivery_url("http://api.safira.club")
///             .build();
///         let created: Webhook = client.create(Entity::Webhook, create).await.unwrap();
///         assert_eq!(created.name, "test webhook");
///         let update = Webhook::update()
///             .name("test update webhook")
///             .build()
///             .unwrap();
///         let updated: Webhook = client
///             .update(Entity::Webhook, created.id, update)
///             .await
///             .unwrap();
///         assert_eq!(updated.name, "test update webhook");
///         let b_upd = Webhook::update()
///             .id(created.id)
///             .name("batch test webhook")
///             .build()
///             .unwrap();
///         let batch = BatchObject::builder().add_update(b_upd).build();
///         let batch_updated: BatchObject<Webhook> =
///             client.batch_update(Entity::Webhook, batch).await.unwrap();
///         assert!(batch_updated.update.is_some());
///         let _deleted: Webhook = client.delete(Entity::Webhook, created.id).await.unwrap();
///     }
///     #[tokio::test]
///     async fn test_list_all_webhooks_retrieve_webhook() {
///         let client = ApiClient::from_env().unwrap();
///         let result = client.list_all::<Webhook>(Entity::Webhook).await.unwrap();
///         assert!(!result.is_empty());
///         if let Some(first) = result.first() {
///             let wh: Webhook = client.retrieve(Entity::Webhook, first.id).await.unwrap();
///             assert_eq!(first.id, wh.id);
///         }
///     }
/// }
/// ```
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

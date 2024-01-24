use crate::Result;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::webhooks::{Event, Resource, WebhookStatus};
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookCreate {
    name: Option<String>,
    status: Option<WebhookStatus>,
    topic: String,
    delivery_url: String,
    secret: Option<String>,
}
#[derive(Default)]
pub struct WebhookCreateBuilder<R, E, D> {
    name: Option<String>,
    status: Option<WebhookStatus>,
    resource: R,
    event: E,
    delivery_url: D,
    secret: Option<String>,
}
pub struct WithResource(Resource);
pub struct WithEvent(Event);
pub struct WithUrl(String);
impl<R, E, D> WebhookCreateBuilder<R, E, D> {
    /// A friendly name for the webhook.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        let _ = self.name.insert(name.into());
        self
    }
    /// Webhook status. Options: active, paused and disabled. Default is active.
    pub fn status(mut self, status: WebhookStatus) -> Self {
        let _ = self.status.insert(status);
        self
    }
    /// Webhook resource.
    pub fn resource(self, resource: Resource) -> WebhookCreateBuilder<WithResource, E, D> {
        WebhookCreateBuilder {
            name: self.name,
            status: self.status,
            resource: WithResource(resource),
            event: self.event,
            delivery_url: self.delivery_url,
            secret: self.secret,
        }
    }
    /// Webhook event.
    pub fn event(self, event: Event) -> WebhookCreateBuilder<R, WithEvent, D> {
        WebhookCreateBuilder {
            name: self.name,
            status: self.status,
            resource: self.resource,
            event: WithEvent(event),
            delivery_url: self.delivery_url,
            secret: self.secret,
        }
    }
    /// The URL where the webhook payload is delivered.
    pub fn delivery_url(self, url: impl Into<String>) -> WebhookCreateBuilder<R, E, WithUrl> {
        WebhookCreateBuilder {
            name: self.name,
            status: self.status,
            resource: self.resource,
            event: self.event,
            delivery_url: WithUrl(url.into()),
            secret: self.secret,
        }
    }
    /// Secret key used to generate a hash of the delivered webhook and provided in the request headers. This will default is a MD5 hash from the current user's ID
    pub fn secret(mut self, secret: impl Into<String>) -> Self {
        let _ = self.secret.insert(secret.into());
        self
    }
}
impl WebhookCreateBuilder<WithResource, WithEvent, WithUrl> {
    pub fn build(self) -> WebhookCreate {
        let topic = format!("{}.{}", self.resource.0, self.event.0);
        WebhookCreate {
            name: self.name,
            status: self.status,
            topic,
            delivery_url: self.delivery_url.0,
            secret: self.secret,
        }
    }
}
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookUpdate {
    id: Option<i32>,
    name: Option<String>,
    status: Option<WebhookStatus>,
    topic: Option<String>,
    delivery_url: Option<String>,
    secret: Option<String>,
}
#[derive(Default)]
pub struct WebhookUpdateBuilder {
    id: Option<i32>,
    name: Option<String>,
    status: Option<WebhookStatus>,
    resource: Option<Resource>,
    event: Option<Event>,
    delivery_url: Option<String>,
    secret: Option<String>,
}
impl WebhookUpdateBuilder {
    /// Unique identifier for the resource.
    pub fn id(mut self, id: i32) -> Self {
        let _ = self.id.insert(id);
        self
    }
    /// A friendly name for the webhook.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        let _ = self.name.insert(name.into());
        self
    }
    /// Webhook status. Options: active, paused and disabled. Default is active.
    pub fn status(mut self, status: WebhookStatus) -> Self {
        let _ = self.status.insert(status);
        self
    }
    /// Webhook resource.
    pub fn resource(mut self, resource: Resource) -> Self {
        let _ = self.resource.insert(resource);
        self
    }
    /// Webhook event.
    pub fn event(mut self, event: Event) -> Self {
        let _ = self.event.insert(event);
        self
    }
    /// The URL where the webhook payload is delivered.
    pub fn delivery_url(mut self, url: impl Into<String>) -> Self {
        let _ = self.delivery_url.insert(url.into());
        self
    }
    /// Secret key used to generate a hash of the delivered webhook and provided in the request headers. This will default is a MD5 hash from the current user's ID
    pub fn secret(mut self, secret: impl Into<String>) -> Self {
        let _ = self.secret.insert(secret.into());
        self
    }
    pub fn build(self) -> Result<WebhookUpdate> {
        if let Some(resource) = self.resource {
            let Some(event) = self.event else {
                return Err("resourse set, but event not set!".into());
            };
            let topic = Some(format!("{resource}.{event}"));
            Ok(WebhookUpdate {
                id: self.id,
                name: self.name,
                status: self.status,
                topic,
                delivery_url: self.delivery_url,
                secret: self.secret,
            })
        } else {
            Ok(WebhookUpdate {
                id: self.id,
                name: self.name,
                status: self.status,
                topic: None,
                delivery_url: self.delivery_url,
                secret: self.secret,
            })
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::{
        webhooks::{Event, Resource, Webhook, WebhookStatus},
        ApiClient, BatchObject, Entity,
    };

    #[tokio::test]
    async fn test_create_update_bathc_update_delete_webhook() {
        let client = ApiClient::from_env().unwrap();
        let create = Webhook::create()
            .name("test webhook")
            .status(WebhookStatus::Disabled)
            .resource(Resource::Customer)
            .event(Event::Deleted)
            .delivery_url("http://api.safira.club")
            .build();
        let created: Webhook = client.create(Entity::Webhook, create).await.unwrap();
        assert_eq!(created.name, "test webhook");
        let update = Webhook::update()
            .name("test update webhook")
            .build()
            .unwrap();
        let updated: Webhook = client
            .update(Entity::Webhook, created.id, update)
            .await
            .unwrap();
        assert_eq!(updated.name, "test update webhook");
        let b_upd = Webhook::update()
            .id(created.id)
            .name("batch test webhook")
            .build()
            .unwrap();
        let batch = BatchObject::builder().add_update(b_upd).build();
        let batch_updated: BatchObject<Webhook> =
            client.batch_update(Entity::Webhook, batch).await.unwrap();
        assert!(batch_updated.update.is_some());
        let _deleted: Webhook = client.delete(Entity::Webhook, created.id).await.unwrap();
    }
    #[tokio::test]
    async fn test_list_all_webhooks_retrieve_webhook() {
        let client = ApiClient::from_env().unwrap();
        let result = client.list_all::<Webhook>(Entity::Webhook).await.unwrap();
        assert!(!result.is_empty());
        if let Some(first) = result.first() {
            let wh: Webhook = client.retrieve(Entity::Webhook, first.id).await.unwrap();
            assert_eq!(first.id, wh.id);
        }
    }
}

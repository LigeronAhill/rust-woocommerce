use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingClassCreate {
    name: String,
    slug: Option<String>,
    description: Option<String>,
}
#[derive(Default)]
pub struct WithName(String);
#[derive(Default)]
pub struct NoName;
#[derive(Default)]
pub struct ShippingClassCreateBuilder<N> {
    name: N,
    slug: Option<String>,
    description: Option<String>,
}
impl<N> ShippingClassCreateBuilder<N> {
    /// Shipping class name.
    pub fn name(self, name: impl Into<String>) -> ShippingClassCreateBuilder<WithName> {
        ShippingClassCreateBuilder {
            name: WithName(name.into()),
            slug: self.slug,
            description: self.description,
        }
    }
    /// An alphanumeric identifier for the resource unique to its type.
    pub fn slug(mut self, slug: impl Into<String>) -> Self {
        let _ = self.slug.insert(slug.into());
        self
    }
    /// HTML description of the resource.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        let _ = self.description.insert(description.into());
        self
    }
}
impl ShippingClassCreateBuilder<WithName> {
    pub fn build(self) -> ShippingClassCreate {
        ShippingClassCreate {
            name: self.name.0,
            slug: self.slug,
            description: self.description,
        }
    }
}
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingClassUpdate {
    id: Option<i32>,
    name: Option<String>,
    slug: Option<String>,
    description: Option<String>,
}
#[derive(Default)]
pub struct ShippingClassUpdateBuilder {
    id: Option<i32>,
    name: Option<String>,
    slug: Option<String>,
    description: Option<String>,
}
impl ShippingClassUpdateBuilder {
    /// Unique identifier for the resource.
    pub fn id(mut self, id: i32) -> Self {
        let _ = self.id.insert(id);
        self
    }
    /// Shipping class name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        let _ = self.name.insert(name.into());
        self
    }
    /// An alphanumeric identifier for the resource unique to its type.
    pub fn slug(mut self, slug: impl Into<String>) -> Self {
        let _ = self.slug.insert(slug.into());
        self
    }
    /// HTML description of the resource.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        let _ = self.description.insert(description.into());
        self
    }
    pub fn build(self) -> ShippingClassUpdate {
        ShippingClassUpdate {
            id: self.id,
            name: self.name,
            slug: self.slug,
            description: self.description,
        }
    }
}

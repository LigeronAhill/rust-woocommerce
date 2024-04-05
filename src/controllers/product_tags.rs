use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
#[derive(Default)]
pub struct WithName(String);
#[derive(Default)]
pub struct NoName;
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductTagCreate {
    /// Tag name.
    pub name: String,
    /// An alphanumeric identifier for the resource unique to its type.
    pub slug: Option<String>,
    /// HTML description of the resource.
    pub description: Option<String>,
}
#[derive(Default)]
pub struct ProductTagCreateBuilder<N> {
    /// Tag name.
    pub name: N,
    /// An alphanumeric identifier for the resource unique to its type.
    pub slug: Option<String>,
    /// HTML description of the resource.
    pub description: Option<String>,
}
impl<N> ProductTagCreateBuilder<N> {
    /// Tag name.
    pub fn name(self, name: impl Into<String>) -> ProductTagCreateBuilder<WithName> {
        ProductTagCreateBuilder {
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
impl ProductTagCreateBuilder<WithName> {
    pub fn build(self) -> ProductTagCreate {
        ProductTagCreate {
            name: self.name.0,
            slug: self.slug,
            description: self.description,
        }
    }
}
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductTagUpdate {
    /// Unique identifier for the resource.
    pub id: Option<i32>,
    /// Tag name.
    pub name: Option<String>,
    /// An alphanumeric identifier for the resource unique to its type.
    pub slug: Option<String>,
    /// HTML description of the resource.
    pub description: Option<String>,
}
#[derive(Default)]
pub struct ProductTagUpdateBuilder {
    /// Unique identifier for the resource.
    pub id: Option<i32>,
    /// Tag name.
    pub name: Option<String>,
    /// An alphanumeric identifier for the resource unique to its type.
    pub slug: Option<String>,
    /// HTML description of the resource.
    pub description: Option<String>,
}
impl ProductTagUpdateBuilder {
    /// Unique identifier for the resource.
    pub fn id(mut self, id: i32) -> Self {
        let _ = self.id.insert(id);
        self
    }
    /// Tag name.
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
    pub fn build(self) -> ProductTagUpdate {
        ProductTagUpdate {
            id: self.id,
            name: self.name,
            slug: self.slug,
            description: self.description,
        }
    }
}

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributeTermCreate {
    name: String,
    slug: Option<String>,
    description: Option<String>,
    menu_order: Option<i32>,
}
#[derive(Default)]
pub struct AttributeTermCreateBuilder<N> {
    name: N,
    slug: Option<String>,
    description: Option<String>,
    menu_order: Option<i32>,
}
#[derive(Default)]
pub struct WithName(String);
#[derive(Default)]
pub struct NoName;
impl<N> AttributeTermCreateBuilder<N> {
    /// Term name.
    pub fn name(self, name: impl Into<String>) -> AttributeTermCreateBuilder<WithName> {
        AttributeTermCreateBuilder {
            name: WithName(name.into()),
            slug: self.slug,
            description: self.description,
            menu_order: self.menu_order,
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
    /// Menu order, used to custom sort the resource.
    pub fn menu_order(mut self, menu_order: i32) -> Self {
        let _ = self.menu_order.insert(menu_order);
        self
    }
}
impl AttributeTermCreateBuilder<WithName> {
    pub fn build(self) -> AttributeTermCreate {
        AttributeTermCreate {
            name: self.name.0,
            slug: self.slug,
            description: self.description,
            menu_order: self.menu_order,
        }
    }
}
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributeTermUpdate {
    id: Option<i32>,
    name: Option<String>,
    slug: Option<String>,
    description: Option<String>,
    menu_order: Option<i32>,
}
#[derive(Default)]
pub struct AttributeTermUpdateBuilder {
    id: Option<i32>,
    name: Option<String>,
    slug: Option<String>,
    description: Option<String>,
    menu_order: Option<i32>,
}
impl AttributeTermUpdateBuilder {
    /// Unique identifier for the resource.
    pub fn id(&mut self, id: i32) -> &mut Self {
        let _ = self.id.insert(id);
        self
    }
    /// Term name.
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
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
    /// Menu order, used to custom sort the resource.
    pub fn menu_order(mut self, menu_order: i32) -> Self {
        let _ = self.menu_order.insert(menu_order);
        self
    }
    pub fn build(&self) -> AttributeTermUpdate {
        AttributeTermUpdate {
            id: self.id,
            name: self.name.to_owned(),
            slug: self.slug.to_owned(),
            description: self.description.to_owned(),
            menu_order: self.menu_order,
        }
    }
}

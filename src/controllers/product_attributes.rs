use serde_with::skip_serializing_none;

use serde::{Deserialize, Serialize};

use crate::{AttributeSortOrder, AttributeType};

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributeCreate {
    name: String,
    slug: Option<String>,
    #[serde(rename = "type")]
    attribute_type: Option<AttributeType>,
    order_by: Option<AttributeSortOrder>,
    has_archives: Option<bool>,
}
#[derive(Default)]
pub struct AttributeCreateBuilder<N> {
    name: N,
    slug: Option<String>,
    attribute_type: Option<AttributeType>,
    order_by: Option<AttributeSortOrder>,
    has_archives: Option<bool>,
}
#[derive(Default)]
pub struct WithName(String);
#[derive(Default)]
pub struct NoName;
impl<N> AttributeCreateBuilder<N> {
    /// Attribute name.
    pub fn name(self, name: impl Into<String>) -> AttributeCreateBuilder<WithName> {
        AttributeCreateBuilder {
            name: WithName(name.into()),
            slug: self.slug,
            attribute_type: self.attribute_type,
            order_by: self.order_by,
            has_archives: self.has_archives,
        }
    }
    /// An alphanumeric identifier for the resource unique to its type.
    pub fn slug(mut self, slug: impl Into<String>) -> Self {
        let _ = self.slug.insert(slug.into());
        self
    }
    /// Type of attribute. By default only select is supported.
    pub fn attribute_type(mut self, attribute_type: AttributeType) -> Self {
        let _ = self.attribute_type.insert(attribute_type);
        self
    }
    /// Default sort order. Options: menu_order, name, name_num and id. Default is menu_order.
    pub fn order_by(mut self, order_by: AttributeSortOrder) -> Self {
        let _ = self.order_by.insert(order_by);
        self
    }
    /// Enable/Disable attribute archives. Default is false.
    pub fn enable_archives(mut self) -> Self {
        let _ = self.has_archives.insert(true);
        self
    }
}
impl AttributeCreateBuilder<WithName> {
    pub fn build(self) -> AttributeCreate {
        AttributeCreate {
            name: self.name.0,
            slug: self.slug,
            attribute_type: self.attribute_type,
            order_by: self.order_by,
            has_archives: self.has_archives,
        }
    }
}
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributeUpdate {
    id: Option<i32>,
    name: Option<String>,
    slug: Option<String>,
    #[serde(rename = "type")]
    attribute_type: Option<AttributeType>,
    order_by: Option<AttributeSortOrder>,
    has_archives: Option<bool>,
}
#[derive(Default)]
pub struct AttributeUpdateBuilder {
    id: Option<i32>,
    name: Option<String>,
    slug: Option<String>,
    attribute_type: Option<AttributeType>,
    order_by: Option<AttributeSortOrder>,
    has_archives: Option<bool>,
}
impl AttributeUpdateBuilder {
    /// Unique identifier for the resource.
    pub fn id(&mut self, id: i32) -> &mut Self {
        let _ = self.id.insert(id);
        self
    }
    /// Attribute name.
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        let _ = self.name.insert(name.into());
        self
    }
    /// An alphanumeric identifier for the resource unique to its type.
    pub fn slug(&mut self, slug: impl Into<String>) -> &mut Self {
        let _ = self.slug.insert(slug.into());
        self
    }
    /// Type of attribute. By default only select is supported.
    pub fn attribute_type(&mut self, attribute_type: AttributeType) -> &mut Self {
        let _ = self.attribute_type.insert(attribute_type);
        self
    }
    /// Default sort order. Options: menu_order, name, name_num and id. Default is menu_order.
    pub fn order_by(&mut self, order_by: AttributeSortOrder) -> &mut Self {
        let _ = self.order_by.insert(order_by);
        self
    }
    /// Enable attribute archives.
    pub fn enable_archives(&mut self) -> &mut Self {
        let _ = self.has_archives.insert(true);
        self
    }
    /// Disable attribute archives.
    pub fn disable_archives(&mut self) -> &mut Self {
        let _ = self.has_archives.insert(false);
        self
    }
    pub fn build(&self) -> AttributeUpdate {
        AttributeUpdate {
            id: self.id,
            name: self.name.to_owned(),
            slug: self.slug.to_owned(),
            attribute_type: self.attribute_type.to_owned(),
            order_by: self.order_by.to_owned(),
            has_archives: self.has_archives,
        }
    }
}

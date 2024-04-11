use serde::{Deserialize, Serialize};

use crate::controllers::product_attributes::{
    AttributeCreateBuilder, AttributeUpdateBuilder, NoName,
};
use crate::controllers::products::{AttributeDTOBuilder, NoOptions};

/// Product attribute properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    /// Unique identifier for the resource.
    pub id: i32,
    /// Attribute name.
    pub name: String,
    /// An alphanumeric identifier for the resource unique to its type.
    pub slug: String,
    /// Type of attribute. By default, only select is supported.
    #[serde(rename = "type")]
    pub attribute_type: AttributeType,
    /// Default sort order. Options: menu_order, name, name_num and id. Default is menu_order.
    pub order_by: AttributeSortOrder,
    /// Enable/Disable attribute archives. Default is false.
    pub has_archives: bool,
}
impl Attribute {
    pub fn create() -> AttributeCreateBuilder<NoName> {
        AttributeCreateBuilder::default()
    }
    pub fn update() -> AttributeUpdateBuilder {
        AttributeUpdateBuilder::default()
    }
    pub fn builder() -> AttributeDTOBuilder<NoName, NoOptions> {
        AttributeDTOBuilder::default()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum AttributeType {
    #[default]
    Select,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AttributeSortOrder {
    #[default]
    MenuOrder,
    Name,
    NameNum,
    Id,
}

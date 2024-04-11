use serde::{Deserialize, Serialize};
use crate::controllers::Entity;

use crate::controllers::product_attribute_terms::{
    AttributeTermCreateBuilder, AttributeTermUpdateBuilder, NoName,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributeTerm {
    /// Unique identifier for the resource.
    pub id: i32,
    /// Term name.
    pub name: String,
    /// An alphanumeric identifier for the resource unique to its type.
    pub slug: String,
    /// HTML description of the resource.
    pub description: String,
    /// Menu order, used to custom sort the resource.
    pub menu_order: i32,
    /// Number of published products for the resource.
    pub count: i32,
}
impl Entity for AttributeTerm {
    fn endpoint() -> String {
        String::new()
    }

    fn child_endpoint(parent_id: i32) -> String {
        format!("products/attributes/{parent_id}/terms/")
    }
}
impl AttributeTerm {
    pub fn create() -> AttributeTermCreateBuilder<NoName> {
        AttributeTermCreateBuilder::default()
    }
    pub fn update() -> AttributeTermUpdateBuilder {
        AttributeTermUpdateBuilder::default()
    }
}

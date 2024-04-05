use serde::{Deserialize, Serialize};

use crate::controllers::product_tags::{NoName, ProductTagCreateBuilder, ProductTagUpdateBuilder};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductTag {
    /// Unique identifier for the resource.
    pub id: i32,
    /// Tag name.
    pub name: String,
    /// An alphanumeric identifier for the resource unique to its type.
    pub slug: String,
    /// HTML description of the resource.
    pub description: String,
    /// Number of published products for the resource.
    pub count: i32,
}
impl ProductTag {
    pub fn create() -> ProductTagCreateBuilder<NoName> {
        ProductTagCreateBuilder::default()
    }
    pub fn update() -> ProductTagUpdateBuilder {
        ProductTagUpdateBuilder::default()
    }
}

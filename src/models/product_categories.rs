use crate::controllers::Entity;
use serde::{Deserialize, Serialize};

use crate::controllers::product_categories::{CategoryCreate, CategoryUpdate};

use super::products::ProductImage;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    /// Unique identifier for the resource.
    pub id: i32,
    /// Category name.
    pub name: String,
    /// An alphanumeric identifier for the resource unique to its type.
    pub slug: String,
    /// The ID for the parent of the resource.
    pub parent: i32,
    /// HTML description of the resource.
    pub description: String,
    /// Category archive display type. Options: default, products, subcategories and both. Default is default.
    pub display: DisplayOption,
    /// Image data.
    pub image: Option<ProductImage>,
    /// Menu order, used to custom sort the resource.
    pub menu_order: i32,
    /// Number of published products for the resource. READ-ONLY
    pub count: i32,
}
impl Category {
    pub fn create<T: ToString>(name: T) -> CategoryCreate {
        CategoryCreate::new(name)
    }
    pub fn update() -> CategoryUpdate {
        CategoryUpdate::default()
    }
}

impl Entity for Category {
    fn endpoint() -> String {
        String::from("products/categories/")
    }
    fn child_endpoint(parent_id: i32) -> String {
        let _ = parent_id;
        String::new()
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DisplayOption {
    #[default]
    Default,
    Products,
    Subcategories,
    Both,
}

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::DisplayOption;

use super::products::ImageDTO;

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CategoryCreate {
    name: String,
    slug: Option<String>,
    parent: Option<i32>,
    description: Option<String>,
    display: Option<DisplayOption>,
    image: Option<ImageDTO>,
    menu_order: Option<i32>,
}
impl CategoryCreate {
    pub fn new<T: ToString>(name: T) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }
    /// An alphanumeric identifier for the resource unique to its type.
    pub fn slug(mut self, slug: impl Into<String>) -> Self {
        let _ = self.slug.insert(slug.into());
        self
    }
    /// The ID for the parent of the resource.
    pub fn parent(mut self, parent: i32) -> Self {
        let _ = self.parent.insert(parent);
        self
    }
    /// HTML description of the resource.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        let _ = self.description.insert(description.into());
        self
    }
    /// Category archive display type. Options: default, products, subcategories and both. Default is default.
    pub fn display(mut self, display: DisplayOption) -> Self {
        let _ = self.display.insert(display);
        self
    }
    /// Image data.
    pub fn image(mut self, img_src: impl Into<String>) -> Self {
        let _ = self.image.insert(ImageDTO {
            src: img_src.into(),
        });
        self
    }
    /// Menu order, used to custom sort the resource.
    pub fn menu_order(mut self, menu_order: i32) -> Self {
        let _ = self.menu_order.insert(menu_order);
        self
    }
}
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CategoryUpdate {
    id: Option<i32>,
    name: Option<String>,
    slug: Option<String>,
    parent: Option<i32>,
    description: Option<String>,
    display: Option<DisplayOption>,
    image: Option<ImageDTO>,
    menu_order: Option<i32>,
}
impl CategoryUpdate {
    /// Unique identifier for the resource.
    pub fn id(mut self, id: i32) -> Self {
        let _ = self.id.insert(id);
        self
    }
    /// Category name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        let _ = self.name.insert(name.into());
        self
    }
    /// An alphanumeric identifier for the resource unique to its type.
    pub fn slug(mut self, slug: impl Into<String>) -> Self {
        let _ = self.slug.insert(slug.into());
        self
    }
    /// The ID for the parent of the resource.
    pub fn parent(mut self, parent: i32) -> Self {
        let _ = self.parent.insert(parent);
        self
    }
    /// HTML description of the resource.
    pub fn description(mut self, description: impl Into<String>) -> Self {
        let _ = self.description.insert(description.into());
        self
    }
    /// Category archive display type. Options: default, products, subcategories and both. Default is default.
    pub fn display(mut self, display: DisplayOption) -> Self {
        let _ = self.display.insert(display);
        self
    }
    /// Image data.
    pub fn image(mut self, img_src: impl Into<String>) -> Self {
        let _ = self.image.insert(ImageDTO {
            src: img_src.into(),
        });
        self
    }
    /// Menu order, used to custom sort the resource.
    pub fn menu_order(mut self, menu_order: i32) -> Self {
        let _ = self.menu_order.insert(menu_order);
        self
    }
}

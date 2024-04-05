use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::product_categories::DisplayOption;

use super::products::ImageDTO;

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryCreate {
    name: String,
    slug: Option<String>,
    parent: Option<i32>,
    description: Option<String>,
    display: Option<DisplayOption>,
    image: Option<ImageDTO>,
    menu_order: Option<i32>,
}
#[derive(Default)]
pub struct WithName(String);
#[derive(Default)]
pub struct NoName;
#[derive(Default)]
pub struct CategoryCreateBuilder<N> {
    name: N,
    slug: Option<String>,
    parent: Option<i32>,
    description: Option<String>,
    display: Option<DisplayOption>,
    image: Option<ImageDTO>,
    menu_order: Option<i32>,
}
impl<N> CategoryCreateBuilder<N> {
    /// Category name.
    pub fn name(self, name: impl Into<String>) -> CategoryCreateBuilder<WithName> {
        CategoryCreateBuilder {
            name: WithName(name.into()),
            slug: self.slug,
            parent: self.parent,
            description: self.description,
            display: self.display,
            image: self.image,
            menu_order: self.menu_order,
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
impl CategoryCreateBuilder<WithName> {
    pub fn build(self) -> CategoryCreate {
        CategoryCreate {
            name: self.name.0,
            slug: self.slug,
            parent: self.parent,
            description: self.description,
            display: self.display,
            image: self.image,
            menu_order: self.menu_order,
        }
    }
}
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Default)]
pub struct CategoryUpdateBuilder {
    id: Option<i32>,
    name: Option<String>,
    slug: Option<String>,
    parent: Option<i32>,
    description: Option<String>,
    display: Option<DisplayOption>,
    image: Option<ImageDTO>,
    menu_order: Option<i32>,
}
impl CategoryUpdateBuilder {
    /// Unique identifier for the resource.
    pub fn id(&mut self, id: i32) -> &mut Self {
        let _ = self.id.insert(id);
        self
    }
    /// Category name.
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        let _ = self.name.insert(name.into());
        self
    }
    /// An alphanumeric identifier for the resource unique to its type.
    pub fn slug(&mut self, slug: impl Into<String>) -> &mut Self {
        let _ = self.slug.insert(slug.into());
        self
    }
    /// The ID for the parent of the resource.
    pub fn parent(&mut self, parent: i32) -> &mut Self {
        let _ = self.parent.insert(parent);
        self
    }
    /// HTML description of the resource.
    pub fn description(&mut self, description: impl Into<String>) -> &mut Self {
        let _ = self.description.insert(description.into());
        self
    }
    /// Category archive display type. Options: default, products, subcategories and both. Default is default.
    pub fn display(&mut self, display: DisplayOption) -> &mut Self {
        let _ = self.display.insert(display);
        self
    }
    /// Image data.
    pub fn image(&mut self, img_src: impl Into<String>) -> &mut Self {
        let _ = self.image.insert(ImageDTO {
            src: img_src.into(),
        });
        self
    }
    /// Menu order, used to custom sort the resource.
    pub fn menu_order(&mut self, menu_order: i32) -> &mut Self {
        let _ = self.menu_order.insert(menu_order);
        self
    }
    pub fn build(&self) -> CategoryUpdate {
        CategoryUpdate {
            id: self.id,
            name: self.name.to_owned(),
            slug: self.slug.to_owned(),
            parent: self.parent,
            description: self.description.to_owned(),
            display: self.display.to_owned(),
            image: self.image.to_owned(),
            menu_order: self.menu_order,
        }
    }
}

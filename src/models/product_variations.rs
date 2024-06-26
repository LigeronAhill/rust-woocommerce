use crate::controllers::product_variations::ProductVariationModifyBuilder;

use super::{
    products::{
        BackordersStatus, Dimensions, Download, ProductDefaultAttribute, ProductImage,
        ProductStatus, StockStatus, TaxStatus,
    },
    MetaData,
};
use crate::controllers::Entity;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductVariation {
    /// Unique identifier for the resource.
    pub id: i32,
    /// The date the variation was created, in the site's timezone.
    pub date_created: NaiveDateTime,
    /// The date the variation was created, as GMT.
    pub date_created_gmt: NaiveDateTime,
    /// The date the variation was last modified, in the site's timezone.
    pub date_modified: NaiveDateTime,
    /// The date the variation was last modified, as GMT.
    pub date_modified_gmt: NaiveDateTime,
    /// Variation description.
    pub description: String,
    /// Variation URL.
    pub permalink: String,
    /// Unique identifier.
    pub sku: String,
    /// Current variation price.
    pub price: String,
    /// Variation regular price.
    pub regular_price: String,
    /// Variation sale price.
    pub sale_price: String,
    /// Start date of sale price, in the site's timezone.
    pub date_on_sale_from: Option<NaiveDateTime>,
    /// Start date of sale price, as GMT.
    pub date_on_sale_from_gmt: Option<NaiveDateTime>,
    /// End date of sale price, in the site's timezone.
    pub date_on_sale_to: Option<NaiveDateTime>,
    /// End date of sale price, as GMT.
    pub date_on_sale_to_gmt: Option<NaiveDateTime>,
    /// Shows if the variation is on sale. READ-ONLY
    pub on_sale: bool,
    /// Variation status. Options: draft, pending, private and publish. Default is publish.
    pub status: ProductStatus,
    /// Shows if the variation can be bought.
    pub purchasable: bool,
    /// If the variation is virtual. Default is false.
    #[serde(rename = "virtual")]
    pub is_virtual: bool,
    /// If the variation is downloadable. Default is false.
    pub downloadable: bool,
    /// List of downloadable files.
    pub downloads: Vec<Download>,
    /// Number of times downloadable files can be downloaded after purchase. Default is -1.
    pub download_limit: i32,
    /// Number of days until access to downloadable files expires. Default is -1.
    pub download_expiry: i32,
    /// Tax status. Options: taxable, shipping and none. Default is taxable.
    pub tax_status: TaxStatus,
    /// Tax class.
    pub tax_class: String,
    /// Stock management at variation level. Default is false.
    pub manage_stock: ManageStock,
    /// Stock quantity.
    pub stock_quantity: Option<i32>,
    /// Controls the stock status of the product. Options: instock, outofstock, onbackorder. Default is instock.
    pub stock_status: StockStatus,
    /// If managing stock, this controls if backorders are allowed. Options: no, notify and yes. Default is no.
    pub backorders: BackordersStatus,
    /// Shows if backorders are allowed.
    pub backorders_allowed: bool,
    /// Shows if the variation is on backordered.
    pub backordered: bool,
    /// Variation weight.
    pub weight: String,
    /// Variation dimensions. See Product variation - Dimensions properties
    pub dimensions: Dimensions,
    /// Shipping class slug.
    pub shipping_class: String,
    /// Shipping class ID.
    pub shipping_class_id: i32,
    /// Variation image data.
    pub image: Option<ProductImage>,
    /// List of attributes.
    pub attributes: Vec<ProductDefaultAttribute>,
    /// Menu order, used to custom sort products.
    pub menu_order: i32,
    /// Meta data.
    pub meta_data: Vec<MetaData>,
}

impl Entity for ProductVariation {
    fn endpoint() -> String {
        String::new()
    }

    fn child_endpoint(parent_id: i32) -> String {
        format!("products/{parent_id}/variations/")
    }
}
impl ProductVariation {
    pub fn builder() -> ProductVariationModifyBuilder {
        ProductVariationModifyBuilder::default()
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ManageStock {
    Bool(bool),
    Parent(String),
}

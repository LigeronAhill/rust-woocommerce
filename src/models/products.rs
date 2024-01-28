use crate::controllers::products::{ProductModify, ProductModifyBuilder};

use super::MetaData;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
/// ```rust
/// #[cfg(test)]
/// mod tests {
///
///     use crate::{products::Product, ApiClient, BatchObject, Entity};
///
///     use super::*;
///     #[tokio::test]
///     async fn test_list_all_products() {
///         let client = ApiClient::from_env().unwrap();
///         let products: Vec<Product> = client.list_all(Entity::Product).await.unwrap();
///         assert!(!products.is_empty());
///     }
///     #[tokio::test]
///     async fn test_retrieve_product() {
///         let client = ApiClient::from_env().unwrap();
///         let products: Vec<Product> = client.list_all(Entity::Product).await.unwrap();
///         let id = products.last().unwrap().id;
///         let product: Product = client.retrieve(Entity::Product, id).await.unwrap();
///         assert_eq!(id, product.id);
///     }
///     #[tokio::test]
///     async fn test_search_product() {
///         let client = ApiClient::from_env().unwrap();
///         let search_string = "AW Marcus 04 4";
///         let search_result: Vec<Product> =
///             client.search(Entity::Product, search_string).await.unwrap();
///         assert_eq!(search_string, search_result[0].sku);
///     }
///     #[tokio::test]
///     async fn test_create_product() {
///         let client = ApiClient::from_env().unwrap();
///         let attribute = AttributeDTO::builder()
///             .name("Тестовый атрибут")
///             .option("69")
///             .build();
///         let product_to_create = Product::create()
///             .name("Тестовый товар")
///             .sku("test product")
///             .regular_price("10000")
///             .attribute(attribute)
///             .build();
///         let created_product: Product = client
///             .create(Entity::Product, product_to_create)
///             .await
///             .unwrap();
///         let id = created_product.id;
///         assert_eq!(created_product.sku, "test product");
///         let _deleted: Product = client.delete(Entity::Product, id).await.unwrap();
///     }
///     #[tokio::test]
///     async fn test_update_product() {
///         let client = ApiClient::from_env().unwrap();
///         let product_to_update = Product::update().regular_price("5000").build();
///         let updated_product: Product = client
///             .update(Entity::Product, 3982, product_to_update)
///             .await
///             .unwrap();
///         assert_eq!(updated_product.id, 3982);
///     }
///     #[tokio::test]
///     async fn test_batch_update_products() {
///         let client = ApiClient::from_env().unwrap();
///         let product_to_update = Product::update().id(3982).regular_price("5000").build();
///         let batch = BatchObject::builder().add_update(product_to_update).build();
///         let updated_products: BatchObject<Product> =
///             client.batch_update(Entity::Product, batch).await.unwrap();
///         assert!(updated_products.update.is_some());
///     }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    /// Unique identifier for the resource.
    pub id: i32,
    /// Product name.
    pub name: String,
    /// Product slug.
    pub slug: String,
    /// Product URL.
    pub permalink: String,
    /// The date the product was created, in the site's timezone.
    pub date_created: NaiveDateTime,
    /// The date the product was created, as GMT.
    pub date_created_gmt: NaiveDateTime,
    /// The date the product was last modified, in the site's timezone.
    pub date_modified: NaiveDateTime,
    /// The date the product was last modified, as GMT.
    pub date_modified_gmt: NaiveDateTime,
    /// Product type, Options: simple, grouped, external and variable. Default is simple.
    #[serde(rename = "type")]
    pub product_type: ProductType,
    /// Product status (post status). Options: draft, pending, private and publish. Default is publish.
    pub status: ProductStatus,
    /// Featured product. Default is false.
    pub featured: bool,
    /// Catalog visibility. Options: visible, catalog, search and hidden. Default is visible.
    pub catalog_visibility: CatalogVisibility,
    /// Product description.
    pub description: String,
    /// Product short description.
    pub short_description: String,
    /// Unique identifier.
    pub sku: String,
    /// Current product price.
    pub price: String,
    /// Product regular price.
    pub regular_price: String,
    /// Product sale price.
    pub sale_price: String,
    /// Start date of sale price, in the site's timezone.
    pub date_on_sale_from: Option<NaiveDateTime>,
    /// Start date of sale price, as GMT.
    pub date_on_sale_from_gmt: Option<NaiveDateTime>,
    /// End date of sale price, in the site's timezone.
    pub date_on_sale_to: Option<NaiveDateTime>,
    /// End date of sale price, as GMT.
    pub date_on_sale_to_gmt: Option<NaiveDateTime>,
    /// Price formatted in HTML.
    pub price_html: String,
    /// Shows if the product is on sale.
    pub on_sale: bool,
    /// Shows if the product can be bought.
    pub purchasable: bool,
    /// Amount of sales.
    pub total_sales: i32,
    /// If the product is virtual. Default is false.
    #[serde(rename = "virtual")]
    pub is_virtual: bool,
    /// If the product is downloadable. Default is false.
    pub downloadable: bool,
    /// List of downloadable files. See Product - Downloads properties
    pub downloads: Vec<Download>,
    /// Number of times downloadable files can be downloaded after purchase. Default is -1.
    pub download_limit: i32,
    /// Number of days until access to downloadable files expires. Default is -1.
    pub download_expiry: i32,
    /// Product external URL. Only for external products.
    pub external_url: String,
    /// Product external button text. Only for external products.
    pub button_text: String,
    /// Tax status. Options: taxable, shipping and none. Default is taxable.
    pub tax_status: TaxStatus,
    /// Tax class.
    pub tax_class: String,
    /// Stock management at product level. Default is false.
    pub manage_stock: bool,
    /// Stock quantity.
    pub stock_quantity: Option<i32>,
    /// Controls the stock status of the product. Options: instock, outofstock, onbackorder. Default is instock.
    pub stock_status: StockStatus,
    /// If managing stock, this controls if backorders are allowed. Options: no, notify and yes. Default is no.
    pub backorders: BackordersStatus,
    /// Shows if backorders are allowed.
    pub backorders_allowed: bool,
    /// Shows if the product is on backordered.
    pub backordered: bool,
    /// Allow one item to be bought in a single order. Default is false.
    pub sold_individually: bool,
    /// Product weight.
    pub weight: String,
    /// Product dimensions.
    pub dimensions: Dimensions,
    /// Shows if the product need to be shipped.
    pub shipping_required: bool,
    /// Shows whether or not the product shipping is taxable.READ-ONLY
    pub shipping_taxable: bool,
    /// Shipping class slug.
    pub shipping_class: String,
    /// Shipping class ID.
    pub shipping_class_id: i32,
    /// Allow reviews. Default is true.
    pub reviews_allowed: bool,
    /// Reviews average rating.
    pub average_rating: String,
    /// Amount of reviews that the product have.
    pub rating_count: i32,
    /// List of related products IDs.
    pub related_ids: Vec<i32>,
    /// List of up-sell products IDs.
    pub upsell_ids: Vec<i32>,
    /// List of cross-sell products IDs.
    pub cross_sell_ids: Vec<i32>,
    /// Product parent ID.
    pub parent_id: i32,
    /// Optional note to send the customer after purchase.
    pub purchase_note: String,
    /// List of categories.
    pub categories: Vec<ProductCategory>,
    /// List of tags.
    pub tags: Vec<ProductTag>,
    /// List of images.
    pub images: Vec<ProductImage>,
    /// List of attributes.
    pub attributes: Vec<ProductAttribute>,
    /// Defaults variation attributes.
    pub default_attributes: Vec<ProductDefaultAttribute>,
    /// List of variations IDs.
    pub variations: Vec<i32>,
    /// List of grouped products ID.
    pub grouped_products: Vec<i32>,
    /// Menu order, used to custom sort products.
    pub menu_order: i32,
    /// Meta data.
    pub meta_data: Vec<MetaData>,
}
impl Product {
    pub fn create() -> ProductModifyBuilder {
        ProductModify::builder()
    }
    pub fn update() -> ProductModifyBuilder {
        ProductModify::builder()
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ProductType {
    #[default]
    Simple,
    Grouped,
    External,
    Variable,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ProductStatus {
    Draft,
    Pending,
    Private,
    #[default]
    Publish,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum CatalogVisibility {
    #[default]
    Visible,
    Catalog,
    Search,
    Hidden,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum TaxStatus {
    #[default]
    Taxable,
    Shipping,
    None,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum StockStatus {
    #[default]
    Instock,
    Outofstock,
    Onbackorder,
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum BackordersStatus {
    #[default]
    No,
    Notify,
    Yes,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Download {
    /// File ID.
    pub id: String,
    /// File name.
    pub name: String,
    /// File URL.
    pub file: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dimensions {
    /// Product length.
    pub length: String,
    /// Product width.
    pub width: String,
    /// Product height.
    pub height: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductCategory {
    /// Category ID.
    pub id: i32,
    /// Category name.
    pub name: String,
    /// Category slug.
    pub slug: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductTag {
    /// Tag ID.
    pub id: i32,
    /// Tag name.
    pub name: String,
    /// Tag slug.
    pub slug: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductImage {
    /// Image ID.
    pub id: i32,
    /// The date the image was created, in the site's timezone.
    pub date_created: NaiveDateTime,
    /// The date the image was created, as GMT.
    pub date_created_gmt: NaiveDateTime,
    /// The date the image was last modified, in the site's timezone.
    pub date_modified: NaiveDateTime,
    /// The date the image was last modified, as GMT.
    pub date_modified_gmt: NaiveDateTime,
    /// Image URL.
    pub src: String,
    /// Image name.
    pub name: String,
    /// Image alternative text.
    pub alt: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductAttribute {
    /// Attribute ID.
    pub id: i32,
    /// Attribute name.
    pub name: String,
    /// Attribute position.
    pub position: i32,
    /// Define if the attribute is visible on the "Additional information" tab in the product's page. Default is false.
    pub visible: bool,
    /// Define if the attribute can be used as variation. Default is false.
    pub variation: bool,
    /// List of available term names of the attribute.
    pub options: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductDefaultAttribute {
    /// Attribute ID.
    pub id: i32,
    /// Attribute name.
    pub name: String,
    /// Selected attribute term name.
    pub option: String,
}

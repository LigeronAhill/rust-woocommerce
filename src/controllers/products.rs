use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    products::{
        BackordersStatus, CatalogVisibility, ProductStatus, ProductType, StockStatus, TaxStatus,
    },
    MetaData,
};
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductModify {
    id: Option<i32>,
    name: Option<String>,
    slug: Option<String>,
    permalink: Option<String>,
    #[serde(rename = "type")]
    product_type: Option<ProductType>,
    status: Option<ProductStatus>,
    featured: Option<bool>,
    catalog_visibility: Option<CatalogVisibility>,
    description: Option<String>,
    short_description: Option<String>,
    sku: Option<String>,
    regular_price: Option<String>,
    sale_price: Option<String>,
    date_on_sale_from: Option<NaiveDateTime>,
    date_on_sale_to: Option<NaiveDateTime>,
    #[serde(rename = "virtual")]
    is_virtual: Option<bool>,
    downloadable: Option<bool>,
    downloads: Option<Vec<DownloadDTO>>,
    download_limit: Option<i32>,
    download_expiry: Option<i32>,
    external_url: Option<String>,
    button_text: Option<String>,
    tax_status: Option<TaxStatus>,
    tax_class: Option<String>,
    manage_stock: Option<bool>,
    stock_quantity: Option<i32>,
    stock_status: Option<StockStatus>,
    backorders: Option<BackordersStatus>,
    sold_individually: Option<bool>,
    weight: Option<String>,
    dimensions: Option<DimensionsDTO>,
    shipping_class: Option<String>,
    reviews_allowed: Option<bool>,
    related_ids: Option<Vec<i32>>,
    upsell_ids: Option<Vec<i32>>,
    cross_sell_ids: Option<Vec<i32>>,
    parent_id: Option<i32>,
    purchase_note: Option<String>,
    categories: Option<Vec<CategoryDTO>>,
    tags: Option<Vec<TagDTO>>,
    images: Option<Vec<ImageDTO>>,
    attributes: Option<Vec<AttributeDTO>>,
    default_attributes: Option<Vec<DefaultAttributeDTO>>,
    grouped_products: Option<Vec<i32>>,
    menu_order: Option<i32>,
    meta_data: Option<Vec<MetaData>>,
}
impl ProductModify {
    pub fn builder() -> ProductModifyBuilder {
        ProductModifyBuilder::default()
    }
}
#[derive(Default)]
pub struct ProductModifyBuilder {
    id: Option<i32>,
    name: Option<String>,
    slug: Option<String>,
    permalink: Option<String>,
    product_type: Option<ProductType>,
    status: Option<ProductStatus>,
    featured: Option<bool>,
    catalog_visibility: Option<CatalogVisibility>,
    description: Option<String>,
    short_description: Option<String>,
    sku: Option<String>,
    regular_price: Option<String>,
    sale_price: Option<String>,
    date_on_sale_from: Option<NaiveDateTime>,
    date_on_sale_to: Option<NaiveDateTime>,
    is_virtual: Option<bool>,
    downloadable: Option<bool>,
    downloads: Option<Vec<DownloadDTO>>,
    download_limit: Option<i32>,
    download_expiry: Option<i32>,
    external_url: Option<String>,
    button_text: Option<String>,
    tax_status: Option<TaxStatus>,
    tax_class: Option<String>,
    manage_stock: Option<bool>,
    stock_quantity: Option<i32>,
    stock_status: Option<StockStatus>,
    backorders: Option<BackordersStatus>,
    sold_individually: Option<bool>,
    weight: Option<String>,
    dimensions: Option<DimensionsDTO>,
    shipping_class: Option<String>,
    reviews_allowed: Option<bool>,
    related_ids: Option<Vec<i32>>,
    upsell_ids: Option<Vec<i32>>,
    cross_sell_ids: Option<Vec<i32>>,
    parent_id: Option<i32>,
    purchase_note: Option<String>,
    categories: Option<Vec<CategoryDTO>>,
    tags: Option<Vec<TagDTO>>,
    images: Option<Vec<ImageDTO>>,
    attributes: Option<Vec<AttributeDTO>>,
    default_attributes: Option<Vec<DefaultAttributeDTO>>,
    grouped_products: Option<Vec<i32>>,
    menu_order: Option<i32>,
    meta_data: Option<Vec<MetaData>>,
}
impl ProductModifyBuilder {
    /// Unique identifier for the resource.
    pub fn id(&mut self, id: i32) -> &mut Self {
        let _ = self.id.insert(id);
        self
    }
    /// Product name.
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        let _ = self.name.insert(name.into());
        self
    }
    /// Product slug.
    pub fn slug(&mut self, slug: impl Into<String>) -> &mut Self {
        let _ = self.slug.insert(slug.into());
        self
    }
    /// Product URL.
    pub fn permalink(&mut self, permalink: impl Into<String>) -> &mut Self {
        let _ = self.permalink.insert(permalink.into());
        self
    }
    /// Product type, Options: simple, grouped, external and variable. Default is simple.
    pub fn product_type(&mut self, product_type: ProductType) -> &mut Self {
        let _ = self.product_type.insert(product_type);
        self
    }
    /// Product status (post status). Options: draft, pending, private and publish. Default is publish.
    pub fn status(&mut self, status: ProductStatus) -> &mut Self {
        let _ = self.status.insert(status);
        self
    }
    /// Featured product. Default is false.
    pub fn featured(&mut self) -> &mut Self {
        let _ = self.featured.insert(true);
        self
    }
    /// Catalog visibility. Options: visible, catalog, search and hidden. Default is visible.
    pub fn catalog_visibility(&mut self, catalog_visibility: CatalogVisibility) -> &mut Self {
        let _ = self.catalog_visibility.insert(catalog_visibility);
        self
    }
    /// Product description.
    pub fn description(&mut self, description: impl Into<String>) -> &mut Self {
        let _ = self.description.insert(description.into());
        self
    }
    /// Product short description.
    pub fn short_description(&mut self, short_description: impl Into<String>) -> &mut Self {
        let _ = self.short_description.insert(short_description.into());
        self
    }
    /// Unique identifier.
    pub fn sku(&mut self, sku: impl Into<String>) -> &mut Self {
        let _ = self.sku.insert(sku.into());
        self
    }
    /// Product regular price.
    pub fn regular_price(&mut self, regular_price: impl Into<String>) -> &mut Self {
        let _ = self.regular_price.insert(regular_price.into());
        self
    }
    /// Product sale price.
    pub fn sale_price(&mut self, sale_price: impl Into<String>) -> &mut Self {
        let _ = self.sale_price.insert(sale_price.into());
        self
    }
    /// Start date of sale price, in the site's timezone.
    pub fn date_on_sale_from(&mut self, year: i32, month: u32, day: u32) -> &mut Self {
        let dt = chrono::NaiveDate::from_ymd_opt(year, month, day)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let _ = self.date_on_sale_from.insert(dt);
        self
    }
    /// End date of sale price, in the site's timezone.
    pub fn date_on_sale_to(&mut self, year: i32, month: u32, day: u32) -> &mut Self {
        let dt = chrono::NaiveDate::from_ymd_opt(year, month, day)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        let _ = self.date_on_sale_to.insert(dt);
        self
    }
    /// If the product is virtual. Default is false.
    pub fn is_virtual(&mut self) -> &mut Self {
        let _ = self.is_virtual.insert(true);
        self
    }
    /// If the product is downloadable. Default is false.
    pub fn downloadable(&mut self) -> &mut Self {
        let _ = self.downloadable.insert(true);
        self
    }
    /// List of downloadable files. See Product - Downloads properties
    pub fn downloads(&mut self, file_src: impl Into<String>) -> &mut Self {
        let f = DownloadDTO {
            file: file_src.into(),
        };
        self.downloads.get_or_insert(vec![]).push(f);
        self
    }
    /// Number of times downloadable files can be downloaded after purchase. Default is -1.
    pub fn download_limit(&mut self, download_limit: i32) -> &mut Self {
        let _ = self.download_limit.insert(download_limit);
        self
    }
    /// Number of days until access to downloadable files expires. Default is -1.
    pub fn download_expiry(&mut self, days: i32) -> &mut Self {
        let _ = self.download_expiry.insert(days);
        self
    }
    /// Product external URL. Only for external products.
    pub fn external_url(&mut self, url: impl Into<String>) -> &mut Self {
        let _ = self.external_url.insert(url.into());
        self
    }
    /// Product external button text. Only for external products.
    pub fn button_text(&mut self, button_text: impl Into<String>) -> &mut Self {
        let _ = self.button_text.insert(button_text.into());
        self
    }
    /// Tax status. Options: taxable, shipping and none. Default is taxable.
    pub fn tax_status(&mut self, tax_status: TaxStatus) -> &mut Self {
        let _ = self.tax_status.insert(tax_status);
        self
    }
    /// Tax class.
    pub fn tax_class(&mut self, tax_class: impl Into<String>) -> &mut Self {
        let _ = self.tax_class.insert(tax_class.into());
        self
    }
    /// Stock management at product level. Default is false.
    pub fn manage_stock(&mut self) -> &mut Self {
        let _ = self.manage_stock.insert(true);
        self
    }
    /// Stock quantity.
    pub fn stock_quantity(&mut self, stock_quantity: i32) -> &mut Self {
        let _ = self.stock_quantity.insert(stock_quantity);
        self
    }
    /// Controls the stock status of the product. Options: instock, outofstock, onbackorder. Default is instock.
    pub fn stock_status(&mut self, stock_status: StockStatus) -> &mut Self {
        let _ = self.stock_status.insert(stock_status);
        self
    }
    /// If managing stock, this controls if backorders are allowed. Options: no, notify and yes. Default is no.
    pub fn backorders(&mut self, backorders: BackordersStatus) -> &mut Self {
        let _ = self.backorders.insert(backorders);
        self
    }
    /// Allow one item to be bought in a single order. Default is false.
    pub fn sold_individually(&mut self) -> &mut Self {
        let _ = self.sold_individually.insert(true);
        self
    }
    /// Product weight.
    pub fn weight(&mut self, weight: impl Into<String>) -> &mut Self {
        let _ = self.weight.insert(weight.into());
        self
    }
    /// Product dimensions.
    pub fn dimensions(
        &mut self,
        length: impl Into<String>,
        width: impl Into<String>,
        height: impl Into<String>,
    ) -> &mut Self {
        let d = DimensionsDTO {
            length: length.into(),
            width: width.into(),
            height: height.into(),
        };
        let _ = self.dimensions.insert(d);
        self
    }
    /// Shipping class slug.
    pub fn shipping_class(&mut self, slug: impl Into<String>) -> &mut Self {
        let _ = self.shipping_class.insert(slug.into());
        self
    }
    /// Allow reviews. Default is true.
    pub fn reviews_allowed_set_false(&mut self) -> &mut Self {
        let _ = self.reviews_allowed.insert(false);
        self
    }
    /// List of related products IDs.
    pub fn related_ids(&mut self, id: i32) -> &mut Self {
        self.related_ids.get_or_insert(vec![]).push(id);
        self
    }
    /// List of up-sell products IDs.
    pub fn upsell_ids(&mut self, id: i32) -> &mut Self {
        self.upsell_ids.get_or_insert(vec![]).push(id);
        self
    }
    /// List of cross-sell products IDs.
    pub fn cross_sell_ids(&mut self, id: i32) -> &mut Self {
        self.cross_sell_ids.get_or_insert(vec![]).push(id);
        self
    }
    /// Product parent ID.
    pub fn parent_id(&mut self, parent_id: i32) -> &mut Self {
        let _ = self.parent_id.insert(parent_id);
        self
    }
    /// Optional note to send the customer after purchase.
    pub fn purchase_note(&mut self, purchase_note: impl Into<String>) -> &mut Self {
        let _ = self.purchase_note.insert(purchase_note.into());
        self
    }
    /// List of categories.
    pub fn categories(&mut self, category_id: i32) -> &mut Self {
        self.categories
            .get_or_insert(vec![])
            .push(CategoryDTO { id: category_id });
        self
    }
    /// List of tags.
    pub fn tags(&mut self, tag_id: i32) -> &mut Self {
        self.tags.get_or_insert(vec![]).push(TagDTO { id: tag_id });
        self
    }
    /// List of images.
    pub fn images(&mut self, img_src: impl Into<String>) -> &mut Self {
        self.images.get_or_insert(vec![]).push(ImageDTO {
            src: img_src.into(),
        });
        self
    }
    /// List of attributes.
    pub fn attribute(&mut self, attribute: AttributeDTO) -> &mut Self {
        self.attributes.get_or_insert(vec![]).push(attribute);
        self
    }
    /// Defaults variation attributes.
    pub fn default_attribute(
        &mut self,
        id: Option<i32>,
        name: impl Into<String>,
        option: impl Into<String>,
    ) -> &mut Self {
        self.default_attributes
            .get_or_insert(vec![])
            .push(DefaultAttributeDTO {
                id,
                name: name.into(),
                option: option.into(),
            });
        self
    }
    /// List of grouped products ID.
    pub fn grouped_product(&mut self, grouped_product_id: i32) -> &mut Self {
        self.grouped_products
            .get_or_insert(vec![])
            .push(grouped_product_id);
        self
    }
    /// Menu order, used to custom sort products.
    pub fn menu_order(&mut self, menu_order: i32) -> &mut Self {
        let _ = self.menu_order.insert(menu_order);
        self
    }
    /// Meta data.
    pub fn meta_data(&mut self, key: impl Into<String>, value: impl Serialize) -> &mut Self {
        self.meta_data.get_or_insert(vec![]).push(MetaData {
            id: None,
            key: key.into(),
            value: serde_json::json!(value),
        });
        self
    }
    pub fn build(&self) -> ProductModify {
        ProductModify {
            id: self.id,
            name: self.name.clone(),
            slug: self.slug.clone(),
            permalink: self.permalink.clone(),
            product_type: self.product_type.clone(),
            status: self.status.clone(),
            featured: self.featured,
            catalog_visibility: self.catalog_visibility.clone(),
            description: self.description.clone(),
            short_description: self.short_description.clone(),
            sku: self.sku.clone(),
            regular_price: self.regular_price.clone(),
            sale_price: self.sale_price.clone(),
            date_on_sale_from: self.date_on_sale_from,
            date_on_sale_to: self.date_on_sale_to,
            is_virtual: self.is_virtual,
            downloadable: self.downloadable,
            downloads: self.downloads.clone(),
            download_limit: self.download_limit,
            download_expiry: self.download_expiry,
            external_url: self.external_url.clone(),
            button_text: self.button_text.clone(),
            tax_status: self.tax_status.clone(),
            tax_class: self.tax_class.clone(),
            manage_stock: self.manage_stock,
            stock_quantity: self.stock_quantity,
            stock_status: self.stock_status.clone(),
            backorders: self.backorders.clone(),
            sold_individually: self.sold_individually,
            weight: self.weight.clone(),
            dimensions: self.dimensions.clone(),
            shipping_class: self.shipping_class.clone(),
            reviews_allowed: self.reviews_allowed,
            related_ids: self.related_ids.clone(),
            upsell_ids: self.upsell_ids.clone(),
            cross_sell_ids: self.cross_sell_ids.clone(),
            parent_id: self.parent_id,
            purchase_note: self.purchase_note.clone(),
            categories: self.categories.clone(),
            tags: self.tags.clone(),
            images: self.images.clone(),
            attributes: self.attributes.clone(),
            default_attributes: self.default_attributes.clone(),
            grouped_products: self.grouped_products.clone(),
            menu_order: self.menu_order,
            meta_data: self.meta_data.clone(),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadDTO {
    // pub id: String,
    // pub name: String,
    pub file: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimensionsDTO {
    /// Product length.
    pub length: String,
    /// Product width.
    pub width: String,
    /// Product height.
    pub height: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryDTO {
    pub id: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagDTO {
    pub id: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageDTO {
    pub src: String,
    // pub name: String,
    // pub alt: String,
}
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AttributeDTO {
    id: Option<i32>,
    name: String,
    position: Option<i32>,
    visible: bool,
    variation: bool,
    options: Vec<String>,
}
impl AttributeDTO {
    pub fn builder() -> AttributeDTOBuilder<NoName, NoOptions> {
        AttributeDTOBuilder::<NoName, NoOptions>::default()
    }
}
#[derive(Default)]
pub struct WithName(String);
#[derive(Default)]
pub struct NoName;
#[derive(Default)]
pub struct Options(Vec<String>);
#[derive(Default)]
pub struct NoOptions;
#[derive(Default)]
pub struct AttributeDTOBuilder<N, O> {
    id: Option<i32>,
    name: N,
    position: Option<i32>,
    visible: Option<bool>,
    variation: Option<bool>,
    options: O,
}
impl<N, O> AttributeDTOBuilder<N, O> {
    /// Attribute ID.
    pub fn id(mut self, id: i32) -> Self {
        let _ = self.id.insert(id);
        self
    }
    /// Attribute name.
    pub fn name(self, name: impl Into<String>) -> AttributeDTOBuilder<WithName, O> {
        AttributeDTOBuilder {
            id: self.id,
            name: WithName(name.into()),
            position: self.position,
            visible: self.visible,
            variation: self.variation,
            options: self.options,
        }
    }
    /// Attribute position.
    pub fn position(mut self, position: i32) -> Self {
        let _ = self.position.insert(position);
        self
    }
    /// Define if the attribute is visible on the "Additional information" tab in the product's page. Default is false.
    pub fn visible(mut self) -> Self {
        let _ = self.visible.insert(true);
        self
    }
    /// Define if the attribute can be used as variation. Default is false.
    pub fn variation(mut self) -> Self {
        let _ = self.variation.insert(true);
        self
    }
    /// List of available term names of the attribute.
    pub fn option(self, option: impl Into<String>) -> AttributeDTOBuilder<N, Options> {
        AttributeDTOBuilder {
            id: self.id,
            name: self.name,
            position: self.position,
            visible: self.visible,
            variation: self.variation,
            options: Options(vec![option.into()]),
        }
    }
    /// List of available term names of the attribute.
    pub fn options(self, options: Vec<String>) -> AttributeDTOBuilder<N, Options> {
        AttributeDTOBuilder {
            id: self.id,
            name: self.name,
            position: self.position,
            visible: self.visible,
            variation: self.variation,
            options: Options(options),
        }
    }
}
impl AttributeDTOBuilder<WithName, Options> {
    pub fn build(self) -> AttributeDTO {
        AttributeDTO {
            id: self.id,
            name: self.name.0,
            position: self.position,
            visible: self.visible.unwrap_or(true),
            variation: self.variation.unwrap_or_default(),
            options: self.options.0,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultAttributeDTO {
    pub id: Option<i32>,
    pub name: String,
    pub option: String,
}
#[cfg(test)]
mod tests {

    use crate::{products::Product, ApiClient, BatchObject, Entity};

    use super::*;
    #[tokio::test]
    async fn test_list_all_products() {
        let client = ApiClient::from_env().unwrap();
        let products: Vec<Product> = client.list_all(Entity::Product).await.unwrap();
        assert!(!products.is_empty());
    }
    #[tokio::test]
    async fn test_retrieve_product() {
        let client = ApiClient::from_env().unwrap();
        let products: Vec<Product> = client.list_all(Entity::Product).await.unwrap();
        let id = products.last().unwrap().id;
        let product: Product = client.retrieve(Entity::Product, id).await.unwrap();
        assert_eq!(id, product.id);
    }
    #[tokio::test]
    async fn test_search_product() {
        let client = ApiClient::from_env().unwrap();
        let search_string = "AW Marcus 04 4";
        let search_result: Vec<Product> =
            client.search(Entity::Product, search_string).await.unwrap();
        assert_eq!(search_string, search_result[0].sku);
    }
    #[tokio::test]
    async fn test_create_product() {
        let client = ApiClient::from_env().unwrap();
        let attribute = AttributeDTO::builder()
            .name("Тестовый атрибут")
            .option("69")
            .build();
        let product_to_create = Product::create()
            .name("Тестовый товар")
            .sku("test product")
            .regular_price("10000")
            .attribute(attribute)
            .build();
        let created_product: Product = client
            .create(Entity::Product, product_to_create)
            .await
            .unwrap();
        let id = created_product.id;
        assert_eq!(created_product.sku, "test product");
        let _deleted: Product = client.delete(Entity::Product, id).await.unwrap();
    }
    #[tokio::test]
    async fn test_update_product() {
        let client = ApiClient::from_env().unwrap();
        let product_to_update = Product::update().regular_price("5000").build();
        let updated_product: Product = client
            .update(Entity::Product, 3982, product_to_update)
            .await
            .unwrap();
        assert_eq!(updated_product.id, 3982);
    }
    #[tokio::test]
    async fn test_batch_update_products() {
        let client = ApiClient::from_env().unwrap();
        let product_to_update = Product::update().id(3982).regular_price("5000").build();
        let batch = BatchObject::builder().add_update(product_to_update).build();
        let updated_products: BatchObject<Product> =
            client.batch_update(Entity::Product, batch).await.unwrap();
        assert!(updated_products.update.is_some());
    }
}

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::models::{
    products::{BackordersStatus, Dimensions, ProductStatus, StockStatus, TaxStatus},
    MetaData,
};

use super::products::{DefaultAttributeDTO, DownloadDTO, ImageDTO};

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductVariationCreate {
    description: Option<String>,
    sku: Option<String>,
    regular_price: Option<String>,
    sale_price: Option<String>,
    date_on_sale_from: Option<NaiveDateTime>,
    date_on_sale_to: Option<NaiveDateTime>,
    status: Option<ProductStatus>,
    #[serde(rename = "virtual")]
    is_virtual: Option<bool>,
    downloadable: Option<bool>,
    downloads: Option<Vec<DownloadDTO>>,
    download_limit: Option<i32>,
    download_expiry: Option<i32>,
    tax_status: Option<TaxStatus>,
    tax_class: Option<String>,
    manage_stock: Option<bool>,
    stock_quantity: Option<i32>,
    stock_status: Option<StockStatus>,
    backorders: Option<BackordersStatus>,
    weight: Option<String>,
    dimensions: Option<Dimensions>,
    shipping_class: Option<String>,
    image: Option<ImageDTO>,
    attributes: Option<Vec<DefaultAttributeDTO>>,
    menu_order: Option<i32>,
    meta_data: Option<Vec<MetaData>>,
}
#[derive(Default)]
pub struct ProductVariationCreateBuilder {
    description: Option<String>,
    sku: Option<String>,
    regular_price: Option<String>,
    sale_price: Option<String>,
    date_on_sale_from: Option<NaiveDateTime>,
    date_on_sale_to: Option<NaiveDateTime>,
    status: Option<ProductStatus>,
    is_virtual: Option<bool>,
    downloadable: Option<bool>,
    downloads: Option<Vec<DownloadDTO>>,
    download_limit: Option<i32>,
    download_expiry: Option<i32>,
    tax_status: Option<TaxStatus>,
    tax_class: Option<String>,
    manage_stock: Option<bool>,
    stock_quantity: Option<i32>,
    stock_status: Option<StockStatus>,
    backorders: Option<BackordersStatus>,
    weight: Option<String>,
    dimensions: Option<Dimensions>,
    shipping_class: Option<String>,
    image: Option<ImageDTO>,
    attributes: Option<Vec<DefaultAttributeDTO>>,
    menu_order: Option<i32>,
    meta_data: Option<Vec<MetaData>>,
}
impl ProductVariationCreateBuilder {
    /// Variation description.
    pub fn description(&mut self, description: impl Into<String>) -> &mut Self {
        let _ = self.description.insert(description.into());
        self
    }
    /// Unique identifier.
    pub fn sku(&mut self, sku: impl Into<String>) -> &mut Self {
        let _ = self.sku.insert(sku.into());
        self
    }
    /// Variation regular price.
    pub fn regular_price(&mut self, regular_price: impl Into<String>) -> &mut Self {
        let _ = self.regular_price.insert(regular_price.into());
        self
    }
    /// Variation sale price.
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
    /// Variation status. Options: draft, pending, private and publish. Default is publish.
    pub fn status(&mut self, status: ProductStatus) -> &mut Self {
        let _ = self.status.insert(status);
        self
    }
    /// If the variation is virtual. Default is false.
    pub fn is_virtual(&mut self) -> &mut Self {
        let _ = self.is_virtual.insert(true);
        self
    }
    /// If the variation is downloadable. Default is false.
    pub fn downloadable(&mut self) -> &mut Self {
        let _ = self.downloadable.insert(true);
        self
    }
    /// List of downloadable files.
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
    pub fn download_expiry(&mut self, download_expiry: i32) -> &mut Self {
        let _ = self.download_expiry.insert(download_expiry);
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
    /// Stock management at variation level. Default is false.
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
    /// Variation weight.
    pub fn weight(&mut self, weight: impl Into<String>) -> &mut Self {
        let _ = self.weight.insert(weight.into());
        self
    }
    /// Variation dimensions. See Product variation - Dimensions properties
    pub fn dimensions(
        &mut self,
        length: impl Into<String>,
        width: impl Into<String>,
        height: impl Into<String>,
    ) -> &mut Self {
        let d = Dimensions {
            length: length.into(),
            width: width.into(),
            height: height.into(),
        };
        let _ = self.dimensions.insert(d);
        self
    }
    /// Shipping class slug.
    pub fn shipping_class(&mut self, shipping_class: impl Into<String>) -> &mut Self {
        let _ = self.shipping_class.insert(shipping_class.into());
        self
    }
    /// Variation image data.
    pub fn image(&mut self, img_src: impl Into<String>) -> &mut Self {
        let _ = self.image.insert(ImageDTO {
            src: img_src.into(),
        });
        self
    }
    /// List of attributes.
    pub fn attributes(
        &mut self,
        id: Option<i32>,
        name: impl Into<String>,
        option: impl Into<String>,
    ) -> &mut Self {
        self.attributes
            .get_or_insert(vec![])
            .push(DefaultAttributeDTO {
                id,
                name: name.into(),
                option: option.into(),
            });
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
    pub fn build(&self) -> ProductVariationCreate {
        ProductVariationCreate {
            description: self.description.to_owned(),
            sku: self.sku.to_owned(),
            regular_price: self.regular_price.to_owned(),
            sale_price: self.sale_price.to_owned(),
            date_on_sale_from: self.date_on_sale_from,
            date_on_sale_to: self.date_on_sale_to,
            status: self.status.to_owned(),
            is_virtual: self.is_virtual,
            downloadable: self.downloadable,
            downloads: self.downloads.to_owned(),
            download_limit: self.download_limit,
            download_expiry: self.download_expiry,
            tax_status: self.tax_status.to_owned(),
            tax_class: self.tax_class.to_owned(),
            manage_stock: self.manage_stock,
            stock_quantity: self.stock_quantity,
            stock_status: self.stock_status.to_owned(),
            backorders: self.backorders.to_owned(),
            weight: self.weight.to_owned(),
            dimensions: self.dimensions.to_owned(),
            shipping_class: self.shipping_class.to_owned(),
            image: self.image.to_owned(),
            attributes: self.attributes.to_owned(),
            menu_order: self.menu_order,
            meta_data: self.meta_data.to_owned(),
        }
    }
}
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductVariationUpdate {
    id: Option<i32>,
    description: Option<String>,
    sku: Option<String>,
    regular_price: Option<String>,
    sale_price: Option<String>,
    date_on_sale_from: Option<NaiveDateTime>,
    date_on_sale_to: Option<NaiveDateTime>,
    status: Option<ProductStatus>,
    #[serde(rename = "virtual")]
    is_virtual: Option<bool>,
    downloadable: Option<bool>,
    downloads: Option<Vec<DownloadDTO>>,
    download_limit: Option<i32>,
    download_expiry: Option<i32>,
    tax_status: Option<TaxStatus>,
    tax_class: Option<String>,
    manage_stock: Option<bool>,
    stock_quantity: Option<i32>,
    stock_status: Option<StockStatus>,
    backorders: Option<BackordersStatus>,
    weight: Option<String>,
    dimensions: Option<Dimensions>,
    shipping_class: Option<String>,
    image: Option<ImageDTO>,
    attributes: Option<Vec<DefaultAttributeDTO>>,
    menu_order: Option<i32>,
    meta_data: Option<Vec<MetaData>>,
}
#[derive(Default)]
pub struct ProductVariationUpdateBuilder {
    id: Option<i32>,
    description: Option<String>,
    sku: Option<String>,
    regular_price: Option<String>,
    sale_price: Option<String>,
    date_on_sale_from: Option<NaiveDateTime>,
    date_on_sale_to: Option<NaiveDateTime>,
    status: Option<ProductStatus>,
    is_virtual: Option<bool>,
    downloadable: Option<bool>,
    downloads: Option<Vec<DownloadDTO>>,
    download_limit: Option<i32>,
    download_expiry: Option<i32>,
    tax_status: Option<TaxStatus>,
    tax_class: Option<String>,
    manage_stock: Option<bool>,
    stock_quantity: Option<i32>,
    stock_status: Option<StockStatus>,
    backorders: Option<BackordersStatus>,
    weight: Option<String>,
    dimensions: Option<Dimensions>,
    shipping_class: Option<String>,
    image: Option<ImageDTO>,
    attributes: Option<Vec<DefaultAttributeDTO>>,
    menu_order: Option<i32>,
    meta_data: Option<Vec<MetaData>>,
}
impl ProductVariationUpdateBuilder {
    /// Unique identifier for the resource.
    pub fn id(&mut self, id: i32) -> &mut Self {
        let _ = self.id.insert(id);
        self
    }
    /// Variation description.
    pub fn description(&mut self, description: impl Into<String>) -> &mut Self {
        let _ = self.description.insert(description.into());
        self
    }
    /// Unique identifier.
    pub fn sku(&mut self, sku: impl Into<String>) -> &mut Self {
        let _ = self.sku.insert(sku.into());
        self
    }
    /// Variation regular price.
    pub fn regular_price(&mut self, regular_price: impl Into<String>) -> &mut Self {
        let _ = self.regular_price.insert(regular_price.into());
        self
    }
    /// Variation sale price.
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
    /// Variation status. Options: draft, pending, private and publish. Default is publish.
    pub fn status(&mut self, status: ProductStatus) -> &mut Self {
        let _ = self.status.insert(status);
        self
    }
    /// If the variation is virtual. Default is false.
    pub fn is_virtual(&mut self) -> &mut Self {
        let _ = self.is_virtual.insert(true);
        self
    }
    /// If the variation is downloadable. Default is false.
    pub fn downloadable(&mut self) -> &mut Self {
        let _ = self.downloadable.insert(true);
        self
    }
    /// List of downloadable files.
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
    pub fn download_expiry(&mut self, download_expiry: i32) -> &mut Self {
        let _ = self.download_expiry.insert(download_expiry);
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
    /// Stock management at variation level. Default is false.
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
    /// Variation weight.
    pub fn weight(&mut self, weight: impl Into<String>) -> &mut Self {
        let _ = self.weight.insert(weight.into());
        self
    }
    /// Variation dimensions. See Product variation - Dimensions properties
    pub fn dimensions(
        &mut self,
        length: impl Into<String>,
        width: impl Into<String>,
        height: impl Into<String>,
    ) -> &mut Self {
        let d = Dimensions {
            length: length.into(),
            width: width.into(),
            height: height.into(),
        };
        let _ = self.dimensions.insert(d);
        self
    }
    /// Shipping class slug.
    pub fn shipping_class(&mut self, shipping_class: impl Into<String>) -> &mut Self {
        let _ = self.shipping_class.insert(shipping_class.into());
        self
    }
    /// Variation image data.
    pub fn image(&mut self, img_src: impl Into<String>) -> &mut Self {
        let _ = self.image.insert(ImageDTO {
            src: img_src.into(),
        });
        self
    }
    /// List of attributes.
    pub fn attributes(
        &mut self,
        id: Option<i32>,
        name: impl Into<String>,
        option: impl Into<String>,
    ) -> &mut Self {
        self.attributes
            .get_or_insert(vec![])
            .push(DefaultAttributeDTO {
                id,
                name: name.into(),
                option: option.into(),
            });
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
    pub fn build(&self) -> ProductVariationUpdate {
        ProductVariationUpdate {
            id: self.id,
            description: self.description.to_owned(),
            sku: self.sku.to_owned(),
            regular_price: self.regular_price.to_owned(),
            sale_price: self.sale_price.to_owned(),
            date_on_sale_from: self.date_on_sale_from,
            date_on_sale_to: self.date_on_sale_to,
            status: self.status.to_owned(),
            is_virtual: self.is_virtual,
            downloadable: self.downloadable,
            downloads: self.downloads.to_owned(),
            download_limit: self.download_limit,
            download_expiry: self.download_expiry,
            tax_status: self.tax_status.to_owned(),
            tax_class: self.tax_class.to_owned(),
            manage_stock: self.manage_stock,
            stock_quantity: self.stock_quantity,
            stock_status: self.stock_status.to_owned(),
            backorders: self.backorders.to_owned(),
            weight: self.weight.to_owned(),
            dimensions: self.dimensions.to_owned(),
            shipping_class: self.shipping_class.to_owned(),
            image: self.image.to_owned(),
            attributes: self.attributes.to_owned(),
            menu_order: self.menu_order,
            meta_data: self.meta_data.to_owned(),
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::{
        controllers::{
            entities::{Entity, SubEntity},
            ApiClient,
        },
        models::product_variations::ProductVariation,
    };

    #[tokio::test]
    async fn test_list_all_product_variations() {
        let client = ApiClient::from_env().unwrap();
        let result: Vec<ProductVariation> = client
            .list_all_subentities(Entity::Product, 3918, SubEntity::ProductVariation)
            .await
            .unwrap();
        assert!(!result.is_empty());
    }
    #[tokio::test]
    async fn test_retrieve_product_variation() {
        let client = ApiClient::from_env().unwrap();
        let result: ProductVariation = client
            .retrieve_subentity(Entity::Product, 3918, SubEntity::ProductVariation, 4058)
            .await
            .unwrap();
        assert_eq!(4058, result.id);
    }
    #[tokio::test]
    async fn test_create_product_variation() {
        let client = ApiClient::from_env().unwrap();
        let variation_to_create = ProductVariation::create().sku("test-sku").build();
        let created: ProductVariation = client
            .create_subentity(
                Entity::Product,
                3918,
                SubEntity::ProductVariation,
                variation_to_create,
            )
            .await
            .unwrap();
        assert_eq!("test-sku", created.sku);
        let id = created.id;
        let _deleted: ProductVariation = client
            .delete_subentity(Entity::Product, 3918, SubEntity::ProductVariation, id)
            .await
            .unwrap();
    }
    #[tokio::test]
    async fn test_update_product_variation() {
        let client = ApiClient::from_env().unwrap();
        let update = ProductVariation::update().regular_price("4000").build();
        let updated: ProductVariation = client
            .update_subentity(
                Entity::Product,
                3918,
                SubEntity::ProductVariation,
                4058,
                update,
            )
            .await
            .unwrap();
        assert_eq!("4000", updated.regular_price);
    }
}

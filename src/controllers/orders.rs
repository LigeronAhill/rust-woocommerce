use crate::models::{
    customers::{Billing, Shipping},
    data::CurrencyISO,
    orders::{OrderStatus, TaxStatus},
    MetaData,
};
pub const ORDER_ID: i32 = 4086;
use crate::Result;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrder {
    parent_id: Option<i32>,
    status: OrderStatus,
    currency: CurrencyISO,
    customer_id: i32,
    customer_note: Option<String>,
    billing: Billing,
    shipping: Shipping,
    payment_method: String,
    payment_method_title: String,
    transaction_id: Option<String>,
    meta_data: Vec<MetaData>,
    line_items: Vec<OrderLineItemCreate>,
    shipping_lines: Vec<ShippingLineCreate>,
    fee_lines: Vec<OrderFeeLineCreate>,
    coupon_lines: Vec<OrderCouponLineCreate>,
    set_paid: bool,
}
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOrder {
    id: Option<i32>,
    parent_id: Option<i32>,
    status: Option<OrderStatus>,
    currency: Option<CurrencyISO>,
    customer_id: Option<i32>,
    customer_note: Option<String>,
    billing: Option<Billing>,
    shipping: Option<Shipping>,
    payment_method: Option<String>,
    payment_method_title: Option<String>,
    transaction_id: Option<String>,
    meta_data: Option<Vec<MetaData>>,
    line_items: Option<Vec<OrderLineItemCreate>>,
    shipping_lines: Option<Vec<ShippingLineCreate>>,
    fee_lines: Option<Vec<OrderFeeLineCreate>>,
    coupon_lines: Option<Vec<OrderCouponLineCreate>>,
    set_paid: Option<bool>,
}
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrderLineItemCreate {
    product_id: i32,
    variation_id: Option<i32>,
    quantity: i32,
    tax_class: Option<String>,
    subtotal: Option<String>,
    total: Option<String>,
    meta_data: Option<Vec<MetaData>>,
    price: Option<f64>,
}
impl OrderLineItemCreate {
    /// new product with id and quantity
    pub fn new() -> Self {
        OrderLineItemCreate::default()
    }
    /// Product ID.
    pub fn product_id(mut self, product_id: i32) -> Self {
        self.product_id = product_id;
        self
    }
    /// Variation ID, if applicable.
    pub fn variation_id(mut self, variation_id: i32) -> Self {
        let _ = self.variation_id.insert(variation_id);
        self
    }
    /// Quantity ordered.
    pub fn quantity(mut self, quantity: i32) -> Self {
        self.quantity = quantity;
        self
    }
    /// Slug of the tax class of product.
    pub fn tax_class(mut self, tax_class: impl Into<String>) -> Self {
        let _ = self.tax_class.insert(tax_class.into());
        self
    }
    /// Line subtotal (before discounts).
    pub fn subtotal(mut self, subtotal: impl Into<String>) -> Self {
        let _ = self.subtotal.insert(subtotal.into());
        self
    }
    /// Line total (after discounts).
    pub fn total(mut self, total: impl Into<String>) -> Self {
        let _ = self.total.insert(total.into());
        self
    }
    /// Meta data.
    pub fn meta_data(mut self, key: impl Into<String>, value: impl serde::Serialize) -> Self {
        self.meta_data.get_or_insert(vec![]).push(MetaData {
            id: None,
            key: key.into(),
            value: serde_json::json!(value),
        });
        self
    }
    /// Product price.
    pub fn price(mut self, price: f64) -> Self {
        let _ = self.price.insert(price);
        self
    }
}
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ShippingLineCreate {
    /// Shipping method name.
    method_title: String,
    /// Shipping method ID.
    method_id: String,
    /// Line total (after discounts).
    total: String,
    /// Meta data.
    meta_data: Option<Vec<MetaData>>,
}
impl ShippingLineCreate {
    /// Shipping lines data.
    pub fn new(
        method_title: impl Into<String>,
        method_id: impl Into<String>,
        total: impl Into<String>,
    ) -> Self {
        Self {
            method_title: method_title.into(),
            method_id: method_id.into(),
            total: total.into(),
            ..Default::default()
        }
    }
    /// Meta data.
    pub fn meta_data(mut self, key: impl Into<String>, value: impl serde::Serialize) -> Self {
        self.meta_data.get_or_insert(vec![]).push(MetaData {
            id: None,
            key: key.into(),
            value: serde_json::json!(value),
        });
        self
    }
}
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderFeeLineCreate {
    name: String,
    tax_class: String,
    tax_status: TaxStatus,
    total: String,
    meta_data: Option<Vec<MetaData>>,
}
impl OrderFeeLineCreate {
    /// Fee name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }
    /// Tax class of fee.
    pub fn tax_class(mut self, tax_class: impl Into<String>) -> Self {
        self.tax_class = tax_class.into();
        self
    }
    /// Tax status of fee. Options: taxable and none.
    pub fn tax_status(mut self, tax_status: TaxStatus) -> Self {
        self.tax_status = tax_status;
        self
    }
    /// Line total (after discounts).
    pub fn total(mut self, total: impl Into<String>) -> Self {
        self.total = total.into();
        self
    }
    /// Meta data.
    pub fn meta_data(mut self, key: impl Into<String>, value: impl Serialize) -> Self {
        let meta_data = MetaData {
            id: None,
            key: key.into(),
            value: serde_json::json!(value),
        };
        self.meta_data.get_or_insert(vec![]).push(meta_data);
        self
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderCouponLineCreate {
    /// Coupon code.
    pub code: String,
}
#[derive(Clone, Default)]
pub struct CreateOrderBuilder {
    parent_id: Option<i32>,
    status: Option<OrderStatus>,
    currency: Option<CurrencyISO>,
    customer_id: Option<i32>,
    customer_note: Option<String>,
    billing: Option<Billing>,
    shipping: Option<Shipping>,
    payment_method: Option<String>,
    payment_method_title: Option<String>,
    transaction_id: Option<String>,
    meta_data: Option<Vec<MetaData>>,
    line_items: Option<Vec<OrderLineItemCreate>>,
    shipping_lines: Option<Vec<ShippingLineCreate>>,
    fee_lines: Option<Vec<OrderFeeLineCreate>>,
    coupon_lines: Option<Vec<OrderCouponLineCreate>>,
    set_paid: Option<bool>,
}
impl CreateOrderBuilder {
    pub fn new() -> Self {
        CreateOrderBuilder::default()
    }
    /// Parent order ID.
    pub fn parent_id(mut self, parent_id: i32) -> Self {
        let _ = self.parent_id.insert(parent_id);
        self
    }
    /// Order status.
    pub fn status(mut self, order_status: OrderStatus) -> Self {
        let _ = self.status.insert(order_status);
        self
    }
    /// Currency the order was created with, in ISO format.
    pub fn currency(mut self, currency: CurrencyISO) -> Self {
        let _ = self.currency.insert(currency);
        self
    }
    /// User ID who owns the order. 0 for guests. Default is 0.
    pub fn customer_id(mut self, customer_id: i32) -> Self {
        let _ = self.customer_id.insert(customer_id);
        self
    }
    /// Note left by customer during checkout.
    pub fn customer_note(mut self, customer_note: impl Into<String>) -> Self {
        let _ = self.customer_note.insert(customer_note.into());
        self
    }
    /// billing first name.
    pub fn billing_first_name(mut self, first_name: impl Into<String>) -> Self {
        self.billing.get_or_insert(Billing::default()).first_name = first_name.into();
        self
    }
    /// billing last name.
    pub fn billing_last_name(mut self, last_name: impl Into<String>) -> Self {
        self.billing.get_or_insert(Billing::default()).last_name = last_name.into();
        self
    }
    /// billing company name.
    pub fn billing_company(mut self, company: impl Into<String>) -> Self {
        self.billing.get_or_insert(Billing::default()).company = company.into();
        self
    }
    /// billing address line 1
    pub fn billing_address_1(mut self, address_1: impl Into<String>) -> Self {
        self.billing.get_or_insert(Billing::default()).address_1 = address_1.into();
        self
    }
    /// billing address line 2
    pub fn billing_address_2(mut self, address_2: impl Into<String>) -> Self {
        self.billing.get_or_insert(Billing::default()).address_2 = address_2.into();
        self
    }
    /// billing city name.
    pub fn billing_city(mut self, city: impl Into<String>) -> Self {
        self.billing.get_or_insert(Billing::default()).city = city.into();
        self
    }
    /// billing ISO code or name of the state, province or district.
    pub fn billing_state(mut self, state: impl Into<String>) -> Self {
        self.billing.get_or_insert(Billing::default()).state = state.into();
        self
    }
    /// billing postal code.
    pub fn billing_postcode(mut self, postcode: impl Into<String>) -> Self {
        self.billing.get_or_insert(Billing::default()).postcode = postcode.into();
        self
    }
    /// billing ISO code of the country.
    pub fn billing_country(mut self, country: impl Into<String>) -> Self {
        self.billing.get_or_insert(Billing::default()).country = country.into();
        self
    }
    /// billing email address.
    pub fn billing_email(mut self, email: impl Into<String>) -> Self {
        self.billing.get_or_insert(Billing::default()).email = email.into();
        self
    }
    /// billing phone number.
    pub fn billing_phone(mut self, phone: impl Into<String>) -> Self {
        self.billing.get_or_insert(Billing::default()).phone = phone.into();
        self
    }
    /// shipping first name.
    pub fn shipping_first_name(mut self, first_name: impl Into<String>) -> Self {
        self.shipping.get_or_insert(Shipping::default()).first_name = first_name.into();
        self
    }
    /// shipping last name.
    pub fn shipping_last_name(mut self, last_name: impl Into<String>) -> Self {
        self.shipping.get_or_insert(Shipping::default()).last_name = last_name.into();
        self
    }
    /// shipping company name.
    pub fn shipping_company(mut self, company: impl Into<String>) -> Self {
        self.shipping.get_or_insert(Shipping::default()).company = company.into();
        self
    }
    /// shipping address line 1
    pub fn shipping_address_1(mut self, address_1: impl Into<String>) -> Self {
        self.shipping.get_or_insert(Shipping::default()).address_1 = address_1.into();
        self
    }
    /// shipping address line 2
    pub fn shipping_address_2(mut self, address_2: impl Into<String>) -> Self {
        self.shipping.get_or_insert(Shipping::default()).address_2 = address_2.into();
        self
    }
    /// shipping city name.
    pub fn shipping_city(mut self, city: impl Into<String>) -> Self {
        self.shipping.get_or_insert(Shipping::default()).city = city.into();
        self
    }
    /// shipping ISO code or name of the state, province or district.
    pub fn shipping_state(mut self, state: impl Into<String>) -> Self {
        self.shipping.get_or_insert(Shipping::default()).state = state.into();
        self
    }
    /// shipping postal code.
    pub fn shipping_postcode(mut self, postcode: impl Into<String>) -> Self {
        self.shipping.get_or_insert(Shipping::default()).postcode = postcode.into();
        self
    }
    /// shipping ISO code of the country.
    pub fn shipping_country(mut self, country: impl Into<String>) -> Self {
        self.shipping.get_or_insert(Shipping::default()).country = country.into();
        self
    }
    /// Payment method ID.
    pub fn payment_method(mut self, payment_method: impl Into<String>) -> Self {
        let _ = self.payment_method.insert(payment_method.into());
        self
    }
    /// Payment method title.
    pub fn payment_method_title(mut self, payment_method_title: impl Into<String>) -> Self {
        let _ = self
            .payment_method_title
            .insert(payment_method_title.into());
        self
    }
    /// Unique transaction ID.
    pub fn transaction_id(mut self, transaction_id: impl Into<String>) -> Self {
        let _ = self.transaction_id.insert(transaction_id.into());
        self
    }
    /// Meta data.
    pub fn meta_data(mut self, key: impl Into<String>, value: impl serde::Serialize) -> Self {
        self.meta_data.get_or_insert(vec![]).push(MetaData {
            id: None,
            key: key.into(),
            value: serde_json::json!(value),
        });
        self
    }
    /// Line items data.
    pub fn line_item(mut self, line_item: OrderLineItemCreate) -> Self {
        self.line_items.get_or_insert(vec![]).push(line_item);
        self
    }
    /// Shipping lines data.
    pub fn shipping_line(mut self, shipping_line: ShippingLineCreate) -> Self {
        self.shipping_lines
            .get_or_insert(vec![])
            .push(shipping_line);
        self
    }
    /// Fee lines data.
    pub fn fee_line(mut self, fee_line: OrderFeeLineCreate) -> Self {
        self.fee_lines.get_or_insert(vec![]).push(fee_line);
        self
    }
    /// Coupons line data.
    pub fn coupon_line(mut self, code: impl Into<String>) -> Self {
        let coupon = OrderCouponLineCreate { code: code.into() };
        self.coupon_lines.get_or_insert(vec![]).push(coupon);
        self
    }
    /// Define if the order is paid. It will set the status to processing and reduce stock items. Default is false.
    pub fn set_paid(mut self, paid: bool) -> Self {
        let _ = self.set_paid.insert(paid);
        self
    }
    pub fn build(self) -> Result<CreateOrder> {
        let Some(billing) = self.billing.clone() else {
            return Err("billing email required!".into());
        };
        if billing.email.is_empty() {
            return Err("billing email required!".into());
        }
        Ok(CreateOrder {
            parent_id: self.parent_id,
            status: self.status.clone().unwrap_or_default(),
            currency: self.currency.clone().unwrap_or_default(),
            customer_id: self.customer_id.unwrap_or(0),
            customer_note: self.customer_note.clone(),
            billing,
            shipping: self.shipping.clone().unwrap_or_default(),
            payment_method: self.payment_method.clone().unwrap_or_default(),
            payment_method_title: self.payment_method_title.clone().unwrap_or_default(),
            transaction_id: self.transaction_id.clone(),
            meta_data: self.meta_data.clone().unwrap_or_default(),
            line_items: self.line_items.clone().unwrap_or_default(),
            shipping_lines: self.shipping_lines.clone().unwrap_or_default(),
            fee_lines: self.fee_lines.clone().unwrap_or_default(),
            coupon_lines: self.coupon_lines.clone().unwrap_or_default(),
            set_paid: self.set_paid.unwrap_or_default(),
        })
    }
}
#[derive(Clone, Default)]
pub struct UpdateOrderBuilder {
    id: Option<i32>,
    parent_id: Option<i32>,
    status: Option<OrderStatus>,
    currency: Option<CurrencyISO>,
    customer_id: Option<i32>,
    customer_note: Option<String>,
    billing: Option<Billing>,
    shipping: Option<Shipping>,
    payment_method: Option<String>,
    payment_method_title: Option<String>,
    transaction_id: Option<String>,
    meta_data: Option<Vec<MetaData>>,
    line_items: Option<Vec<OrderLineItemCreate>>,
    shipping_lines: Option<Vec<ShippingLineCreate>>,
    fee_lines: Option<Vec<OrderFeeLineCreate>>,
    coupon_lines: Option<Vec<OrderCouponLineCreate>>,
    set_paid: Option<bool>,
}
impl UpdateOrderBuilder {
    pub fn new() -> Self {
        UpdateOrderBuilder::default()
    }
    /// Unique identifier for the resource.
    pub fn id(mut self, id: i32) -> Self {
        let _ = self.id.insert(id);
        self
    }
    /// Parent order ID.
    pub fn parent_id(mut self, parent_id: i32) -> Self {
        let _ = self.parent_id.insert(parent_id);
        self
    }
    /// Order status.
    pub fn status(mut self, order_status: OrderStatus) -> Self {
        let _ = self.status.insert(order_status);
        self
    }
    /// Currency the order was created with, in ISO format.
    pub fn currency(mut self, currency: CurrencyISO) -> Self {
        let _ = self.currency.insert(currency);
        self
    }
    /// User ID who owns the order. 0 for guests. Default is 0.
    pub fn customer_id(mut self, customer_id: i32) -> Self {
        let _ = self.customer_id.insert(customer_id);
        self
    }
    /// Note left by customer during checkout.
    pub fn customer_note(mut self, customer_note: impl Into<String>) -> Self {
        let _ = self.customer_note.insert(customer_note.into());
        self
    }
    /// billing first name.
    pub fn billing_first_name(mut self, first_name: impl Into<String>) -> Self {
        self.billing.get_or_insert(Billing::default()).first_name = first_name.into();
        self
    }
    /// billing last name.
    pub fn billing_last_name(mut self, last_name: impl Into<String>) -> Self {
        self.billing.get_or_insert(Billing::default()).last_name = last_name.into();
        self
    }
    /// billing company name.
    pub fn billing_company(mut self, company: impl Into<String>) -> Self {
        self.billing.get_or_insert(Billing::default()).company = company.into();
        self
    }
    /// billing address line 1
    pub fn billing_address_1(mut self, address_1: impl Into<String>) -> Self {
        self.billing.get_or_insert(Billing::default()).address_1 = address_1.into();
        self
    }
    /// billing address line 2
    pub fn billing_address_2(mut self, address_2: impl Into<String>) -> Self {
        self.billing.get_or_insert(Billing::default()).address_2 = address_2.into();
        self
    }
    /// billing city name.
    pub fn billing_city(mut self, city: impl Into<String>) -> Self {
        self.billing.get_or_insert(Billing::default()).city = city.into();
        self
    }
    /// billing ISO code or name of the state, province or district.
    pub fn billing_state(mut self, state: impl Into<String>) -> Self {
        self.billing.get_or_insert(Billing::default()).state = state.into();
        self
    }
    /// billing postal code.
    pub fn billing_postcode(mut self, postcode: impl Into<String>) -> Self {
        self.billing.get_or_insert(Billing::default()).postcode = postcode.into();
        self
    }
    /// billing ISO code of the country.
    pub fn billing_country(mut self, country: impl Into<String>) -> Self {
        self.billing.get_or_insert(Billing::default()).country = country.into();
        self
    }
    /// billing email address.
    pub fn billing_email(mut self, email: impl Into<String>) -> Self {
        self.billing.get_or_insert(Billing::default()).email = email.into();
        self
    }
    /// billing phone number.
    pub fn billing_phone(mut self, phone: impl Into<String>) -> Self {
        self.billing.get_or_insert(Billing::default()).phone = phone.into();
        self
    }
    /// shipping first name.
    pub fn shipping_first_name(mut self, first_name: impl Into<String>) -> Self {
        self.shipping.get_or_insert(Shipping::default()).first_name = first_name.into();
        self
    }
    /// shipping last name.
    pub fn shipping_last_name(mut self, last_name: impl Into<String>) -> Self {
        self.shipping.get_or_insert(Shipping::default()).last_name = last_name.into();
        self
    }
    /// shipping company name.
    pub fn shipping_company(mut self, company: impl Into<String>) -> Self {
        self.shipping.get_or_insert(Shipping::default()).company = company.into();
        self
    }
    /// shipping address line 1
    pub fn shipping_address_1(mut self, address_1: impl Into<String>) -> Self {
        self.shipping.get_or_insert(Shipping::default()).address_1 = address_1.into();
        self
    }
    /// shipping address line 2
    pub fn shipping_address_2(mut self, address_2: impl Into<String>) -> Self {
        self.shipping.get_or_insert(Shipping::default()).address_2 = address_2.into();
        self
    }
    /// shipping city name.
    pub fn shipping_city(mut self, city: impl Into<String>) -> Self {
        self.shipping.get_or_insert(Shipping::default()).city = city.into();
        self
    }
    /// shipping ISO code or name of the state, province or district.
    pub fn shipping_state(mut self, state: impl Into<String>) -> Self {
        self.shipping.get_or_insert(Shipping::default()).state = state.into();
        self
    }
    /// shipping postal code.
    pub fn shipping_postcode(mut self, postcode: impl Into<String>) -> Self {
        self.shipping.get_or_insert(Shipping::default()).postcode = postcode.into();
        self
    }
    /// shipping ISO code of the country.
    pub fn shipping_country(mut self, country: impl Into<String>) -> Self {
        self.shipping.get_or_insert(Shipping::default()).country = country.into();
        self
    }
    /// Payment method ID.
    pub fn payment_method(mut self, payment_method: impl Into<String>) -> Self {
        let _ = self.payment_method.insert(payment_method.into());
        self
    }
    /// Payment method title.
    pub fn payment_method_title(mut self, payment_method_title: impl Into<String>) -> Self {
        let _ = self
            .payment_method_title
            .insert(payment_method_title.into());
        self
    }
    /// Unique transaction ID.
    pub fn transaction_id(mut self, transaction_id: impl Into<String>) -> Self {
        let _ = self.transaction_id.insert(transaction_id.into());
        self
    }
    /// Meta data.
    pub fn meta_data(mut self, key: impl Into<String>, value: impl serde::Serialize) -> Self {
        self.meta_data.get_or_insert(vec![]).push(MetaData {
            id: None,
            key: key.into(),
            value: serde_json::json!(value),
        });
        self
    }
    /// Line items data.
    pub fn line_item(mut self, line_item: OrderLineItemCreate) -> Self {
        self.line_items.get_or_insert(vec![]).push(line_item);
        self
    }
    /// Shipping lines data.
    pub fn shipping_line(mut self, shipping_line: ShippingLineCreate) -> Self {
        self.shipping_lines
            .get_or_insert(vec![])
            .push(shipping_line);
        self
    }
    /// Fee lines data.
    pub fn fee_line(mut self, fee_line: OrderFeeLineCreate) -> Self {
        self.fee_lines.get_or_insert(vec![]).push(fee_line);
        self
    }
    /// Coupons line data.
    pub fn coupon_line(mut self, code: impl Into<String>) -> Self {
        let coupon = OrderCouponLineCreate { code: code.into() };
        self.coupon_lines.get_or_insert(vec![]).push(coupon);
        self
    }
    /// Define if the order is paid. It will set the status to processing and reduce stock items. Default is false.
    pub fn set_paid(mut self, paid: bool) -> Self {
        let _ = self.set_paid.insert(paid);
        self
    }
    pub fn build(self) -> UpdateOrder {
        UpdateOrder {
            id: self.id,
            parent_id: self.parent_id,
            status: self.status,
            currency: self.currency,
            customer_id: self.customer_id,
            customer_note: self.customer_note,
            billing: self.billing,
            shipping: self.shipping,
            payment_method: self.payment_method,
            payment_method_title: self.payment_method_title,
            transaction_id: self.transaction_id,
            meta_data: self.meta_data,
            line_items: self.line_items,
            shipping_lines: self.shipping_lines,
            fee_lines: self.fee_lines,
            coupon_lines: self.coupon_lines,
            set_paid: self.set_paid,
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::{
        controllers::{entities::Entity, ApiClient},
        models::orders::Order,
    };

    use super::*;
    #[tokio::test]
    async fn test_list_all_orders() {
        let client = ApiClient::from_env().unwrap();
        let orders: Vec<Order> = client.list_all(Entity::Order).await.unwrap();
        assert!(!orders.is_empty());
    }
    #[tokio::test]
    async fn test_retrieve_order() {
        let client = ApiClient::from_env().unwrap();
        let orders: Vec<Order> = client.list_all(Entity::Order).await.unwrap();
        let id = orders[0].id;
        let order: Order = client.retrieve(Entity::Order, id).await.unwrap();
        assert_eq!(id, order.id);
    }
    #[tokio::test]
    async fn test_search_order() {
        let client = ApiClient::from_env().unwrap();
        let search_string = "Тестов";
        let search_result: Vec<Order> = client.search(Entity::Order, search_string).await.unwrap();
        assert_eq!(search_string, search_result[0].billing.last_name);
    }
    #[tokio::test]
    async fn create_order() {
        let client = ApiClient::from_env().unwrap();
        let line_item = OrderLineItemCreate::new()
            .product_id(3744)
            .quantity(10)
            .price(5800.0);
        let shipping_line = ShippingLineCreate::new(
            "Доставка по Москве в пределах МКАД до подъезда или терминала ТК",
            "flat_rate",
            "5000",
        );
        let order_to_create = Order::create()
            .line_item(line_item)
            .shipping_line(shipping_line)
            .status(OrderStatus::Pending)
            .billing_first_name("John")
            .billing_country("Zimbabwe")
            .billing_email("president@google.com")
            .billing_last_name("Connor")
            .currency(CurrencyISO::RUB)
            .set_paid(false)
            .build()
            .unwrap();
        let created_order: Order = client.create(Entity::Order, order_to_create).await.unwrap();
        assert_eq!(created_order.status, OrderStatus::Pending);
        let _deleted: Order = client
            .delete(Entity::Order, created_order.id)
            .await
            .unwrap();
    }
    #[tokio::test]
    async fn update_order() {
        let client = ApiClient::from_env().unwrap();
        let orders: Vec<Order> = client.list_all(Entity::Order).await.unwrap();
        let order_to_update = orders.last().unwrap().id;
        let customer_note = "Testing update";
        let update = Order::update().customer_note(customer_note).build();
        let updated_order: Order = client
            .update(Entity::Order, order_to_update, update)
            .await
            .unwrap();
        assert_eq!(updated_order.customer_note, customer_note);
    }
}

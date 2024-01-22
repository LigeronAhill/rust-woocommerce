use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::MetaData;

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundCreate {
    amount: Option<String>,
    reason: Option<String>,
    refunded_by: Option<i32>,
    meta_data: Option<Vec<MetaData>>,
    line_items: Option<Vec<OrderRefundLineItemCreate>>,
    api_refund: Option<bool>,
    api_restock: Option<bool>,
}
impl RefundCreate {
    pub fn builder() -> RefundCreateBuilder<NoAmount, NoItems> {
        RefundCreateBuilder {
            amount: NoAmount,
            line_items: NoItems,
            ..Default::default()
        }
    }
}
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderRefundLineItemCreate {
    id: i32,
    name: Option<String>,
    product_id: Option<i32>,
    variation_id: Option<i32>,
    quantity: i32,
    total: Option<String>,
    total_tax: Option<String>,
    meta_data: Option<Vec<MetaData>>,
    refund_total: Option<f64>,
}
impl OrderRefundLineItemCreate {
    pub fn builder() -> OrderRefundLineItemCreateBuilder<NoId, NoQuantity> {
        OrderRefundLineItemCreateBuilder {
            id: NoId,
            quantity: NoQuantity,
            ..Default::default()
        }
    }
}
#[derive(Default, Clone)]
pub struct RefundCreateBuilder<A, I> {
    amount: A,
    reason: Option<String>,
    refunded_by: Option<i32>,
    meta_data: Option<Vec<MetaData>>,
    line_items: I,
    api_refund: Option<bool>,
    api_restock: Option<bool>,
}
#[derive(Default, Clone)]
pub struct NoAmount;
#[derive(Default, Clone)]
pub struct NoItems;
#[derive(Default, Clone)]
pub struct WithAmount(String);
#[derive(Default, Clone)]
pub struct WithItems(Vec<OrderRefundLineItemCreate>);
#[derive(Default, Clone)]
pub struct NoId;
#[derive(Default, Clone)]
pub struct NoQuantity;
#[derive(Default, Clone)]
pub struct WithId(i32);
#[derive(Default, Clone)]
pub struct WithQuantity(i32);
impl<A, I> RefundCreateBuilder<A, I> {
    /// Total refund amount. Optional. If this parameter is provided, it will take precedence over line item totals, even when total of line items does not matches with this amount.
    pub fn amount(self, amount: impl Into<String>) -> RefundCreateBuilder<WithAmount, I> {
        RefundCreateBuilder {
            amount: WithAmount(amount.into()),
            reason: self.reason,
            refunded_by: self.refunded_by,
            meta_data: self.meta_data,
            line_items: self.line_items,
            api_refund: self.api_refund,
            api_restock: self.api_restock,
        }
    }
    /// Reason for refund.
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        let _ = self.reason.insert(reason.into());
        self
    }
    /// User ID of user who created the refund.    
    pub fn refunded_by(mut self, refunded_by: i32) -> Self {
        let _ = self.refunded_by.insert(refunded_by);
        self
    }
    /// Meta data.    
    pub fn meta_data(mut self, key: impl Into<String>, value: impl Serialize) -> Self {
        self.meta_data.get_or_insert(vec![]).push(MetaData {
            id: None,
            key: key.into(),
            value: serde_json::json!(value),
        });
        self
    }
    /// When true, the payment gateway API is used to generate the refund. Default is true.
    /// This method set false
    pub fn api_refund(mut self) -> Self {
        let _ = self.api_refund.insert(false);
        self
    }
    /// When true, the selected line items are restocked Default is true.
    /// This method set false
    pub fn api_restock(mut self) -> Self {
        let _ = self.api_restock.insert(false);
        self
    }
}
impl<A> RefundCreateBuilder<A, NoItems> {
    /// Line item data.
    pub fn line_item(self, line_item_id: i32, quatnity: i32) -> RefundCreateBuilder<A, WithItems> {
        let l = OrderRefundLineItemCreate::builder()
            .id(line_item_id)
            .quantity(quatnity)
            .build();
        RefundCreateBuilder {
            amount: self.amount,
            reason: self.reason,
            refunded_by: self.refunded_by,
            meta_data: self.meta_data,
            line_items: WithItems(vec![l]),
            api_refund: self.api_refund,
            api_restock: self.api_restock,
        }
    }
}
impl<A> RefundCreateBuilder<A, WithItems> {
    /// Line item data.
    pub fn line_item(mut self, line_item_id: i32, quatnity: i32) -> Self {
        let line_item = OrderRefundLineItemCreate::builder()
            .id(line_item_id)
            .quantity(quatnity)
            .build();
        self.line_items.0.push(line_item);
        self
    }
}
impl RefundCreateBuilder<NoAmount, WithItems> {
    pub fn build(self) -> RefundCreate {
        RefundCreate {
            amount: None,
            reason: self.reason,
            refunded_by: self.refunded_by,
            meta_data: self.meta_data,
            line_items: Some(self.line_items.0),
            api_refund: self.api_refund,
            api_restock: self.api_restock,
        }
    }
}
impl RefundCreateBuilder<WithAmount, WithItems> {
    pub fn build(self) -> RefundCreate {
        RefundCreate {
            amount: Some(self.amount.0),
            reason: self.reason,
            refunded_by: self.refunded_by,
            meta_data: self.meta_data,
            line_items: Some(self.line_items.0),
            api_refund: self.api_refund,
            api_restock: self.api_restock,
        }
    }
}
impl RefundCreateBuilder<WithAmount, NoItems> {
    pub fn build(self) -> RefundCreate {
        RefundCreate {
            amount: Some(self.amount.0),
            reason: self.reason,
            refunded_by: self.refunded_by,
            meta_data: self.meta_data,
            line_items: None,
            api_refund: self.api_refund,
            api_restock: self.api_restock,
        }
    }
}
#[derive(Default, Clone)]
pub struct OrderRefundLineItemCreateBuilder<I, Q> {
    /// Item ID
    pub id: I,
    /// Product name.
    pub name: Option<String>,
    /// Product ID.
    pub product_id: Option<i32>,
    /// Variation ID, if applicable.    
    pub variation_id: Option<i32>,
    /// Quantity ordered.
    pub quantity: Q,
    /// Line total (after discounts).    
    pub total: Option<String>,
    /// Line total tax (after discounts).
    pub total_tax: Option<String>,
    /// Meta data.
    pub meta_data: Option<Vec<MetaData>>,
    // The amount to refund for this line item, excluding taxes.
    pub refund_total: Option<f64>,
}
impl<I, Q> OrderRefundLineItemCreateBuilder<I, Q> {
    /// Item ID
    pub fn id(self, id: i32) -> OrderRefundLineItemCreateBuilder<WithId, Q> {
        OrderRefundLineItemCreateBuilder {
            id: WithId(id),
            name: self.name,
            product_id: self.product_id,
            variation_id: self.variation_id,
            quantity: self.quantity,
            total: self.total,
            total_tax: self.total_tax,
            meta_data: self.meta_data,
            refund_total: self.refund_total,
        }
    }
    /// Product name.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        let _ = self.name.insert(name.into());
        self
    }
    /// Product ID.
    pub fn product_id(mut self, product_id: i32) -> Self {
        let _ = self.product_id.insert(product_id);
        self
    }
    /// Variation ID, if applicable.    
    pub fn variation_id(mut self, variation_id: i32) -> Self {
        let _ = self.variation_id.insert(variation_id);
        self
    }
    /// Quantity ordered.
    pub fn quantity(self, quantity: i32) -> OrderRefundLineItemCreateBuilder<I, WithQuantity> {
        OrderRefundLineItemCreateBuilder {
            id: self.id,
            name: self.name,
            product_id: self.product_id,
            variation_id: self.variation_id,
            quantity: WithQuantity(quantity),
            total: self.total,
            total_tax: self.total_tax,
            meta_data: self.meta_data,
            refund_total: self.refund_total,
        }
    }
    /// Line total (after discounts).    
    pub fn total(mut self, total: impl Into<String>) -> Self {
        let _ = self.total.insert(total.into());
        self
    }
    /// Line total tax (after discounts).
    pub fn total_tax(mut self, total_tax: impl Into<String>) -> Self {
        let _ = self.total_tax.insert(total_tax.into());
        self
    }
    /// Meta data.
    pub fn meta_data(mut self, key: impl Into<String>, value: impl Serialize) -> Self {
        self.meta_data.get_or_insert(vec![]).push(MetaData {
            id: None,
            key: key.into(),
            value: serde_json::json!(value),
        });
        self
    }
    // The amount to refund for this line item, excluding taxes.
    pub fn refund_total(mut self, refund_total: f64) -> Self {
        let _ = self.refund_total.insert(refund_total);
        self
    }
}
impl OrderRefundLineItemCreateBuilder<WithId, WithQuantity> {
    pub fn build(self) -> OrderRefundLineItemCreate {
        OrderRefundLineItemCreate {
            id: self.id.0,
            name: self.name,
            product_id: self.product_id,
            variation_id: self.variation_id,
            quantity: self.quantity.0,
            total: self.total,
            total_tax: self.total_tax,
            meta_data: self.meta_data,
            refund_total: self.refund_total,
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::{controllers::orders::ORDER_ID, refunds::Refund, ApiClient, Entity, SubEntity};

    #[tokio::test]
    async fn test_list_all_refunds() {
        let client = ApiClient::from_env().unwrap();
        let order_refunds: Vec<Refund> = client
            .list_all_subentities(Entity::Order, ORDER_ID, SubEntity::Refund)
            .await
            .unwrap();
        assert!(!order_refunds.is_empty());
    }
    #[tokio::test]
    async fn test_retrieve_refund() {
        let client = ApiClient::from_env().unwrap();
        let order_refunds: Vec<Refund> = client
            .list_all_subentities(Entity::Order, ORDER_ID, SubEntity::Refund)
            .await
            .unwrap();
        let id = order_refunds.last().unwrap().id;
        let order_refund: Refund = client
            .retrieve_subentity(Entity::Order, ORDER_ID, SubEntity::Refund, id)
            .await
            .unwrap();
        assert_eq!(id, order_refund.id);
    }
}

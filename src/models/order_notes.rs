use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::controllers::order_notes::{
    NoId, NoNote, OrderNotesCreateBuilder, OrderNotesUpdateBuilder,
};
/// ```rust
/// #[cfg(test)]
/// mod tests {
///     use crate::{
///         controllers::orders::ORDER_ID, order_notes::OrderNotes, ApiClient, Entity, SubEntity,
///     };
///
///     #[tokio::test]
///     async fn test_list_all_order_notes() {
///         let client = ApiClient::from_env().unwrap();
///         let order_notes: Vec<OrderNotes> = client
///             .list_all_subentities(Entity::Order, ORDER_ID, SubEntity::OrderNote)
///             .await
///             .unwrap();
///         assert!(!order_notes.is_empty());
///     }
///     #[tokio::test]
///     async fn test_retrieve_order_note() {
///         let client = ApiClient::from_env().unwrap();
///         let order_notes: Vec<OrderNotes> = client
///             .list_all_subentities(Entity::Order, ORDER_ID, SubEntity::OrderNote)
///             .await
///             .unwrap();
///         let id = order_notes.last().unwrap().id;
///         let order_note: OrderNotes = client
///             .retrieve_subentity(Entity::Order, ORDER_ID, SubEntity::OrderNote, id)
///             .await
///             .unwrap();
///         assert_eq!(id, order_note.id);
///     }
///     #[tokio::test]
///     async fn test_create_order_note() {
///         let client = ApiClient::from_env().unwrap();
///         let note = OrderNotes::create()
///             .note("Testing note")
///             .customer_note()
///             .added_by_user()
///             .build();
///         let created_note: OrderNotes = client
///             .create_subentity(Entity::Order, ORDER_ID, SubEntity::OrderNote, note)
///             .await
///             .unwrap();
///         assert_eq!(created_note.customer_note, true);
///         let _deleted: OrderNotes = client
///             .delete_subentity(
///                 Entity::Order,
///                 ORDER_ID,
///                 SubEntity::OrderNote,
///                 created_note.id,
///             )
///             .await
///             .unwrap();
///     }
///     #[tokio::test]
///     async fn test_update_order_note() {
///         let client = ApiClient::from_env().unwrap();
///         let order_notes: Vec<OrderNotes> = client
///             .list_all_subentities(Entity::Order, ORDER_ID, SubEntity::OrderNote)
///             .await
///             .unwrap();
///         let id = order_notes.first().unwrap().id;
///         let update = OrderNotes::update().id(id).note("some test note").build();
///         let updated_note: Result<OrderNotes, _> = client
///             .update_subentity(Entity::Order, ORDER_ID, SubEntity::OrderNote, id, update)
///             .await;
///         assert!(updated_note.is_err())
///     }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderNotes {
    /// Unique identifier for the resource.
    pub id: i32,
    /// Order note author    
    pub author: String,
    /// The date the order note was created, in the site's timezone.
    pub date_created: NaiveDateTime,
    /// The date the order note was created, as GMT.
    pub date_created_gmt: NaiveDateTime,
    /// Order note content.
    pub note: String,
    /// If true, the note will be shown to customers and they will be notified. If false, the note will be for admin reference only. Default is false.    
    pub customer_note: bool,
    /// If true, this note will be attributed to the current user. If false, the note will be attributed to the system. Default is false.    
    pub added_by_user: Option<bool>,
}
impl OrderNotes {
    pub fn create() -> OrderNotesCreateBuilder<NoNote> {
        OrderNotesCreateBuilder::<NoNote>::new()
    }
    pub fn update() -> OrderNotesUpdateBuilder<NoId> {
        OrderNotesUpdateBuilder::<NoId>::new()
    }
}

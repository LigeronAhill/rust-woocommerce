use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderNotesCreate {
    note: String,
    customer_note: bool,
    added_by_user: Option<bool>,
}
#[derive(Default)]
pub struct NoNote;
pub struct WithNote(String);
#[derive(Default)]
pub struct OrderNotesCreateBuilder<T> {
    note: T,
    customer_note: Option<bool>,
    added_by_user: Option<bool>,
}
impl<T> OrderNotesCreateBuilder<T> {
    pub fn new() -> OrderNotesCreateBuilder<NoNote> {
        OrderNotesCreateBuilder::default()
    }
    /// Order note content.
    pub fn note(self, note: impl Into<String>) -> OrderNotesCreateBuilder<WithNote> {
        OrderNotesCreateBuilder {
            note: WithNote(note.into()),
            customer_note: self.customer_note,
            added_by_user: self.customer_note,
        }
    }
    /// If true, the note will be shown to customers and they will be notified. If false, the note will be for admin reference only. Default is false.    
    pub fn customer_note(mut self) -> Self {
        let _ = self.customer_note.insert(true);
        self
    }
    /// If true, this note will be attributed to the current user. If false, the note will be attributed to the system. Default is false.    
    pub fn added_by_user(mut self) -> Self {
        let _ = self.added_by_user.insert(true);
        self
    }
}
impl OrderNotesCreateBuilder<WithNote> {
    pub fn build(self) -> OrderNotesCreate {
        OrderNotesCreate {
            note: self.note.0,
            customer_note: self.customer_note.unwrap_or_default(),
            added_by_user: self.added_by_user,
        }
    }
}
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrderNotesUpdate {
    id: i32,
    note: Option<String>,
    customer_note: Option<bool>,
    added_by_user: Option<bool>,
}
#[derive(Default)]
pub struct NoId;
pub struct WithId(i32);
#[derive(Default)]
pub struct OrderNotesUpdateBuilder<I> {
    id: I,
    note: Option<String>,
    customer_note: Option<bool>,
    added_by_user: Option<bool>,
}
impl<I> OrderNotesUpdateBuilder<I> {
    pub fn new() -> OrderNotesUpdateBuilder<NoId> {
        OrderNotesUpdateBuilder::default()
    }
    /// Unique identifier for the resource.
    pub fn id(self, id: i32) -> OrderNotesUpdateBuilder<WithId> {
        OrderNotesUpdateBuilder {
            id: WithId(id),
            note: self.note,
            customer_note: self.customer_note,
            added_by_user: self.added_by_user,
        }
    }
    /// Order note content.
    pub fn note(mut self, note: impl Into<String>) -> Self {
        let _ = self.note.insert(note.into());
        self
    }
    /// If true, the note will be shown to customers and they will be notified. If false, the note will be for admin reference only. Default is false.    
    pub fn customer_note(mut self) -> Self {
        let _ = self.customer_note.insert(true);
        self
    }
    /// If true, this note will be attributed to the current user. If false, the note will be attributed to the system. Default is false.    
    pub fn added_by_user(mut self) -> Self {
        let _ = self.added_by_user.insert(true);
        self
    }
}
impl OrderNotesUpdateBuilder<WithId> {
    pub fn build(self) -> OrderNotesUpdate {
        OrderNotesUpdate {
            id: self.id.0,
            note: self.note,
            customer_note: self.customer_note,
            added_by_user: self.added_by_user,
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::{
        controllers::orders::ORDER_ID, order_notes::OrderNotes, ApiClient, Entity, SubEntity,
    };

    #[tokio::test]
    async fn test_list_all_order_notes() {
        let client = ApiClient::from_env().unwrap();
        let order_notes: Vec<OrderNotes> = client
            .list_all_subentities(Entity::Order, ORDER_ID, SubEntity::OrderNote)
            .await
            .unwrap();
        assert!(!order_notes.is_empty());
    }
    #[tokio::test]
    async fn test_retrieve_order_note() {
        let client = ApiClient::from_env().unwrap();
        let order_notes: Vec<OrderNotes> = client
            .list_all_subentities(Entity::Order, ORDER_ID, SubEntity::OrderNote)
            .await
            .unwrap();
        let id = order_notes.last().unwrap().id;
        let order_note: OrderNotes = client
            .retrieve_subentity(Entity::Order, ORDER_ID, SubEntity::OrderNote, id)
            .await
            .unwrap();
        assert_eq!(id, order_note.id);
    }
    #[tokio::test]
    async fn test_create_order_note() {
        let client = ApiClient::from_env().unwrap();
        let note = OrderNotes::create()
            .note("Testing note")
            .customer_note()
            .added_by_user()
            .build();
        let created_note: OrderNotes = client
            .create_subentity(Entity::Order, ORDER_ID, SubEntity::OrderNote, note)
            .await
            .unwrap();
        assert_eq!(created_note.customer_note, true);
        let _deleted: OrderNotes = client
            .delete_subentity(
                Entity::Order,
                ORDER_ID,
                SubEntity::OrderNote,
                created_note.id,
            )
            .await
            .unwrap();
    }
    #[tokio::test]
    async fn test_update_order_note() {
        let client = ApiClient::from_env().unwrap();
        let order_notes: Vec<OrderNotes> = client
            .list_all_subentities(Entity::Order, ORDER_ID, SubEntity::OrderNote)
            .await
            .unwrap();
        let id = order_notes.first().unwrap().id;
        let update = OrderNotes::update().id(id).note("some test note").build();
        let updated_note: Result<OrderNotes, _> = client
            .update_subentity(Entity::Order, ORDER_ID, SubEntity::OrderNote, id, update)
            .await;
        assert!(updated_note.is_err())
    }
}

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::controllers::order_notes::{
    NoId, NoNote, OrderNotesCreateBuilder, OrderNotesUpdateBuilder,
};
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

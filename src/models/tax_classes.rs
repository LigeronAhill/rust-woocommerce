use crate::controllers::Entity;
use serde::{Deserialize, Serialize};

use crate::controllers::tax_classes::TaxClassCreateBuilder;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxClass {
    /// Unique identifier for the resource.
    pub slug: String,
    /// Tax class name.
    pub name: String,
}
impl Entity for TaxClass {
    fn endpoint() -> String {
        String::from("taxes/classes/")
    }

    fn child_endpoint(parent_id: i32) -> String {
        let _ = parent_id;
        String::new()
    }
}
impl TaxClass {
    pub fn create() -> TaxClassCreateBuilder {
        TaxClassCreateBuilder::default()
    }
}

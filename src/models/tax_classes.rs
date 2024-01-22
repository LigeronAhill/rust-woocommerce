use serde::{Deserialize, Serialize};

use crate::controllers::tax_classes::TaxClassCreateBuilder;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxClass {
    /// Unique identifier for the resource.
    pub slug: String,
    /// Tax class name.
    pub name: String,
}
impl TaxClass {
    pub fn create() -> TaxClassCreateBuilder {
        TaxClassCreateBuilder::default()
    }
}

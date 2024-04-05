use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxClassCreate {
    name: Option<String>,
}
#[derive(Default)]
pub struct TaxClassCreateBuilder {
    /// Tax class name.
    pub name: Option<String>,
}
impl TaxClassCreateBuilder {
    /// Tax class name.
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        let _ = self.name.insert(name.into());
        self
    }
    pub fn build(&self) -> TaxClassCreate {
        TaxClassCreate {
            name: self.name.to_owned(),
        }
    }
}

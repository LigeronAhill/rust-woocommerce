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
#[cfg(test)]
mod tests {
    use crate::{tax_classes::TaxClass, ApiClient, Entity};

    #[tokio::test]
    async fn test_list_all_tax_classes() {
        let client = ApiClient::from_env().unwrap();
        let all_classes: Vec<TaxClass> = client.list_all(Entity::TaxClass).await.unwrap();
        assert!(!all_classes.is_empty());
    }
    #[tokio::test]
    async fn test_create_delete_tax_class() {
        let client = ApiClient::from_env().unwrap();
        let create = TaxClass::create().name("testname").build();
        let created: TaxClass = client.create(Entity::TaxClass, create).await.unwrap();
        assert_eq!(created.slug, "testname");
        let _deleted: TaxClass = client.delete(Entity::TaxClass, created.slug).await.unwrap();
    }
}

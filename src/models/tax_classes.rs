use serde::{Deserialize, Serialize};

use crate::controllers::tax_classes::TaxClassCreateBuilder;
/// ```rust
/// #[cfg(test)]
/// mod tests {
///     use crate::{tax_classes::TaxClass, ApiClient, Entity};
///
///     #[tokio::test]
///     async fn test_list_all_tax_classes() {
///         let client = ApiClient::from_env().unwrap();
///         let all_classes: Vec<TaxClass> = client.list_all(Entity::TaxClass).await.unwrap();
///         assert!(!all_classes.is_empty());
///     }
///     #[tokio::test]
///     async fn test_create_delete_tax_class() {
///         let client = ApiClient::from_env().unwrap();
///         let create = TaxClass::create().name("testname").build();
///         let created: TaxClass = client.create(Entity::TaxClass, create).await.unwrap();
///         assert_eq!(created.slug, "testname");
///         let _deleted: TaxClass = client.delete(Entity::TaxClass, created.slug).await.unwrap();
///     }
/// }
/// ```
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

use serde::{Deserialize, Serialize};

use crate::controllers::product_attribute_terms::{
    AttributeTermCreateBuilder, AttributeTermUpdateBuilder, NoName,
};
/// ```rust
///#[cfg(test)]
///mod tests {
///    use crate::{product_attribute_terms::AttributeTerm, ApiClient, Entity, SubEntity};
///
///    const ATTRIBUTE_ID: i32 = 2;
///    const ATTRIBUTE_TERM_ID: i32 = 240;
///    #[tokio::test]
///    async fn test_list_all_attribute_terms() {
///        let client = ApiClient::from_env().unwrap();
///        let terms = client
///            .list_all_subentities::<AttributeTerm>(
///                Entity::ProductAttribute,
///                ATTRIBUTE_ID,
///                SubEntity::AttributeTerm,
///            )
///            .await
///            .unwrap();
///        assert!(!terms.is_empty());
///    }
///    #[tokio::test]
///    async fn test_retrieve_attribute_term() {
///        let client = ApiClient::from_env().unwrap();
///        let term: AttributeTerm = client
///            .retrieve_subentity(
///                Entity::ProductAttribute,
///                ATTRIBUTE_ID,
///                SubEntity::AttributeTerm,
///                ATTRIBUTE_TERM_ID,
///            )
///            .await
///            .unwrap();
///        assert_eq!(term.id, ATTRIBUTE_TERM_ID);
///    }
///    #[tokio::test]
///    async fn test_create_attribute_term() {
///        let client = ApiClient::from_env().unwrap();
///        let create = AttributeTerm::create().name("test term").build();
///        let created: AttributeTerm = client
///            .create_subentity(
///                Entity::ProductAttribute,
///                ATTRIBUTE_ID,
///                SubEntity::AttributeTerm,
///                create,
///            )
///            .await
///            .unwrap();
///        assert_eq!(created.name, "test term");
///        let _deleted: AttributeTerm = client
///            .delete_subentity(
///                Entity::ProductAttribute,
///                ATTRIBUTE_ID,
///                SubEntity::AttributeTerm,
///                created.id,
///            )
///            .await
///            .unwrap();
///    }
///    #[tokio::test]
///    async fn test_update_attribute_term() {
///        let client = ApiClient::from_env().unwrap();
///        let update = AttributeTerm::update().menu_order(0).build();
///        let updated: AttributeTerm = client
///            .update_subentity(
///                Entity::ProductAttribute,
///                ATTRIBUTE_ID,
///                SubEntity::AttributeTerm,
///                ATTRIBUTE_TERM_ID,
///                update,
///            )
///            .await
///            .unwrap();
///        assert_eq!(updated.menu_order, 0);
///    }
///}
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributeTerm {
    /// Unique identifier for the resource.
    pub id: i32,
    /// Term name.
    pub name: String,
    /// An alphanumeric identifier for the resource unique to its type.
    pub slug: String,
    /// HTML description of the resource.
    pub description: String,
    /// Menu order, used to custom sort the resource.
    pub menu_order: i32,
    /// Number of published products for the resource.
    pub count: i32,
}
impl AttributeTerm {
    pub fn create() -> AttributeTermCreateBuilder<NoName> {
        AttributeTermCreateBuilder::default()
    }
    pub fn update() -> AttributeTermUpdateBuilder {
        AttributeTermUpdateBuilder::default()
    }
}

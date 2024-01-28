use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::controllers::product_reviews::{
    NoEmail, NoId, ProductReviewCreateBuilder, ProductReviewUpdateBuilder,
};
/// ```rust
/// #[cfg(test)]
/// mod tests {
///     use crate::{
///         product_reviews::{ProductReview, ReviewStatus},
///         ApiClient, BatchObject, Entity,
///     };
///
///     #[tokio::test]
///     async fn test_list_all_product_review() {
///         let client = ApiClient::from_env().unwrap();
///         let result = client
///             .list_all::<ProductReview>(Entity::ProductReview)
///             .await
///             .unwrap();
///         assert!(result.is_empty());
///     }
///     #[tokio::test]
///     async fn test_retrieve_product_review() {
///         let _client = ApiClient::from_env().unwrap();
///     }
///     #[tokio::test]
///     async fn test_create_update_batch_update_delete_product_review() {
///         let client = ApiClient::from_env().unwrap();
///         let create = ProductReview::create()
///             .product_id(3982)
///             .reviewer_email("president@world.ua")
///             .review("Cool test review")
///             .rating(5)
///             .status(ReviewStatus::Approved)
///             .reviewer("Zorro")
///             .verified(true)
///             .build();
///         let created: ProductReview = client.create(Entity::ProductReview, create).await.unwrap();
///         assert_eq!(created.reviewer, "Zorro");
///         let update = ProductReview::update().rating(4).build();
///         let update_to_batch = ProductReview::update().rating(5).id(created.id).build();
///         let batch = BatchObject::builder().add_update(update_to_batch).build();
///         let updated: ProductReview = client
///             .update(Entity::ProductReview, created.id, update)
///             .await
///             .unwrap();
///         assert_eq!(updated.rating, 4);
///         let batch_updated: BatchObject<ProductReview> = client
///             .batch_update(Entity::ProductReview, batch)
///             .await
///             .unwrap();
///         assert!(batch_updated.update.is_some());
///         let _deleted: serde_json::Value = client
///             .delete(Entity::ProductReview, created.id)
///             .await
///             .unwrap();
///     }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductReview {
    /// Unique identifier for the resource.
    pub id: i32,
    /// The date the review was created, in the site's timezone.
    pub date_created: NaiveDateTime,
    /// The date the review was created, as GMT.
    pub date_created_gmt: NaiveDateTime,
    /// Unique identifier for the product that the review belongs to.
    pub product_id: i32,
    /// Status of the review. Options: approved, hold, spam, unspam, trash and untrash. Defaults to approved.
    pub status: ReviewStatus,
    /// Reviewer name.
    pub reviewer: String,
    /// Reviewer email.
    pub reviewer_email: String,
    /// The content of the review.
    pub review: String,
    /// Review rating (0 to 5).
    pub rating: i32,
    /// Shows if the reviewer bought the product or not.
    pub verified: bool,
}
impl ProductReview {
    pub fn create() -> ProductReviewCreateBuilder<NoId, NoEmail> {
        ProductReviewCreateBuilder::default()
    }
    pub fn update() -> ProductReviewUpdateBuilder {
        ProductReviewUpdateBuilder::default()
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ReviewStatus {
    #[default]
    Approved,
    Hold,
    Spam,
    Unspam,
    Trash,
    Untrash,
}

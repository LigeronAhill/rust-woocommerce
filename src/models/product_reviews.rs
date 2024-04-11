use crate::controllers::Entity;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::controllers::product_reviews::{
    NoEmail, NoId, ProductReviewCreateBuilder, ProductReviewUpdateBuilder,
};
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
impl Entity for ProductReview {
    fn endpoint() -> String {
        String::from("products/reviews/")
    }

    fn child_endpoint(parent_id: i32) -> String {
        let _ = parent_id;
        String::new()
    }
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

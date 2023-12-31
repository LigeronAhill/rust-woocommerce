use rust_woocommerce::controllers::ApiClient;
use rust_woocommerce::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let client = ApiClient::from_env()?;
    let coupon_b = rust_woocommerce::controllers::coupons::CreateCoupon::builder()
        .code("test-96")
        .discount_type(rust_woocommerce::models::coupons::DiscountType::FixedCart)
        .amount("10000")
        .description("test description")
        .date_expires("2024-01-03")
        .date_expires_gmt("2024-01-03")
        .individual_use()
        .product_id(3748)
        .excluded_product_id(3744)
        .usage_limit(10)
        .usage_limit_per_user(1)
        .limit_usage_to_x_items(5)
        .free_shipping()
        .product_category(342)
        .excluded_product_category(149)
        .exclude_sale_items()
        .minimum_amount("10000")
        .maximum_amount("100000")
        .email_restriction("test@gmail.com")
        .meta_data("test-key", "test-value");
    let coupon_b = coupon_b.product_id(6);
    let coupon_to_create = coupon_b.build();
    let created_coupon_b = client
        .create::<serde_json::Value>(
            rust_woocommerce::controllers::entities::Entity::Coupon,
            coupon_to_create,
        )
        .await?;
    println!("{created_coupon_b:#?}");
    Ok(())
}

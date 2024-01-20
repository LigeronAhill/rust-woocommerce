# rust-woocommerce

Library for working with woocommerce API with Rust programming language.

## Usage

From environment:

```rust
  let client = ApiClient::from_env().unwrap();
  let products: Vec<Product> = client.list_all(Entity::Product).await?;
```

With builder:

```rust
  let site_url = "google.com";
  let customer_key = "super6secret9";
  let customer_secret = "customer4secret2";
  let client = rust_woocommerce::ApiClient::builder()
      .auth(customer_key, customer_secret)
      .site_url(site_url)
      .build();
  let result: ProductVariation = client
      .retrieve_subentity(Entity::Product, 3918, SubEntity::ProductVariation, 4058)
      .await?;
```

Create entity:

```rust
  let attribute = AttributeDTO::builder()
      .name("test attribute")
      .option("69")
      .build();
  let product_to_create = Product::create()
      .name("Тестовый товар")
      .sku("test product")
      .regular_price("10000")
      .attribute(attribute)
      .build();
  let created_product: Product = client
      .create(Entity::Product, product_to_create)
      .await?;
```

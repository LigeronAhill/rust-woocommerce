use rust_woocommerce::{models::BatchObject, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut c = Vec::new();
    let mut u = Vec::new();
    for i in 1..10 {
        c.push(i);
    }
    for i in 1..60 {
        u.push(i);
    }
    let batch = rust_woocommerce::models::BatchObject {
        create: Some(c),
        update: Some(u),
    };
    let r = create_batches(&batch);
    println!("{r:#?}");
    Ok(())
}
fn create_batches<T>(input_batch: &BatchObject<T>) -> Vec<BatchObject<T>>
where
    T: serde::Serialize + Clone,
{
    let mut result_batches = Vec::new();
    let create_pages = {
        if let Some(create) = input_batch.create.to_owned() {
            create
                .chunks(50)
                .map(|slice| slice.to_vec())
                .collect::<Vec<Vec<T>>>()
        } else {
            vec![]
        }
    };
    let update_pages = {
        if let Some(update) = input_batch.update.to_owned() {
            update
                .chunks(50)
                .map(|slice| slice.to_vec())
                .collect::<Vec<Vec<T>>>()
        } else {
            vec![]
        }
    };
    let max_length = std::cmp::max(create_pages.len(), update_pages.len());
    for i in 0..max_length {
        let mut b = BatchObject::builder();
        if let Some(create) = create_pages.get(i) {
            b.extend_create(create.clone());
        }
        if let Some(update) = update_pages.get(i) {
            b.extend_update(update.clone());
        }
        let batch = b.build();
        result_batches.push(batch);
    }
    result_batches
}

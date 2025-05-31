use crate::models::{Item, User};
use mongodb::{Database, Collection};
use futures::stream::TryStreamExt;
use stripe::{Client, CreatePaymentIntent, Currency};
use mongodb::bson::doc;

pub async fn create_item(db: &Database, item: Item) -> mongodb::error::Result<()> {
    let collection: Collection<Item> = db.collection("items");
    collection.insert_one(item, None).await?;
    Ok(())
}

pub async fn list_items(db: &Database) -> mongodb::error::Result<Vec<Item>> {
    let collection: Collection<Item> = db.collection("items");
    let mut cursor = collection.find(None, None).await?;
    let mut items = Vec::new();
    while let Some(item) = cursor.try_next().await? {
        items.push(item);
    }
    Ok(items)
}

pub async fn buy_item(db: &Database, item_id: String) -> mongodb::error::Result<()> {
    let collection: Collection<Item> = db.collection("items");

    // Initialize Stripe client
    let stripe_secret = std::env::var("STRIPE_SECRET_KEY").expect("STRIPE_SECRET_KEY must be set");
    let stripe_client = Client::new(stripe_secret);

    // Fetch the item to get its price
    let filter = doc! { "_id": &item_id };
    let item = collection.find_one(filter.clone(), None).await?
        .ok_or_else(|| mongodb::error::Error::from(std::io::Error::new(std::io::ErrorKind::NotFound, "Item not found")))?;

    // Create a Stripe PaymentIntent
    let mut params = CreatePaymentIntent::new(
        (item.price * 100.0) as i64, // Convert dollars to cents
        Currency::USD,               // Use the enum variant, not `Currency::USD`
    );
    params.payment_method_types = Some(vec!["card".to_string()]);

    let _payment_intent = stripe::PaymentIntent::create(&stripe_client, params)
        .await
        .map_err(|e| mongodb::error::Error::from(std::io::Error::new(std::io::ErrorKind::Other, format!("Stripe error: {}", e))))?;

    // Mark the item as purchased
    let update = doc! { "$set": { "purchased": true } };
    collection.update_one(filter, update, None).await?;

    Ok(())
}

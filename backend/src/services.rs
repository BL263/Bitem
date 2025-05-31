use crate::models::{Item, User};
use mongodb::{Database, bson::doc, Collection};
use futures::stream::TryStreamExt;
use stripe::{Client as StripeClient, PaymentIntent, PaymentIntentStatus};

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

pub async fn buy_item(
    db: &Database,
    item_id: String,
    pay_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let collection: Collection<Item> = db.collection("items");

    // Stripe client
    let stripe_secret = std::env::var("STRIPE_SECRET_KEY")
        .map_err(|_| "STRIPE_SECRET_KEY must be set")?;
    let stripe_client = StripeClient::new(stripe_secret);

    // Get item from DB
    let filter = doc! { "_id": &item_id };
    let item = collection
        .find_one(filter.clone(), None)
        .await?
        .ok_or("Item not found")?;

    if item.price <= 0.0 {
        return Err("Item price must be greater than zero".into());
    }

    if pay_id.trim().is_empty() {
        return Err("Payment ID cannot be empty".into());
    }

    let payment_intent_id = pay_id.parse()?; 
    let payment_intent = PaymentIntent::retrieve(&stripe_client, &payment_intent_id, &[]).await?;

    match payment_intent.status {
        PaymentIntentStatus::Succeeded => {
            // OK
        }
        _ => {
            return Err(format!("Payment was not successful (status: {:?})", payment_intent.status).into());
        }
    }

    // Mark the item as purchased
    let update = doc! { "$set": { "purchased": true } };
    collection.update_one(filter, update, None).await?;

    Ok(())
}
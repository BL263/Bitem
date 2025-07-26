use crate::models::{Item, User};
use std::str::FromStr;
use mongodb::{Database, bson::doc, Collection};
use futures::stream::TryStreamExt;
use mongodb::bson::oid::ObjectId;
use stripe::{
    Client as StripeClient,
    CreatePaymentIntent,
    Currency,
    PaymentMethod,
    PaymentIntent,
    PaymentIntentStatus,
    PaymentIntentConfirmationMethod,
    PaymentMethodId,
    PaymentIntentOffSession
};


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


pub async fn get_item_by_id(db: &Database, item_id: String) -> Result<Option<Item>, Box<dyn std::error::Error>> {
    let collection: Collection<Item> = db.collection("items");

    // Convert String to ObjectId
    let obj_id = ObjectId::parse_str(&item_id)?;

    // Create filter to match _id
    let filter = doc! { "_id": obj_id };

    // Retrieve one item
    let item = collection.find_one(filter, None).await?;

    Ok(item)
}

pub async fn buy_item(
    db: &Database,
    item_id: String,
    pay_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let collection: Collection<Item> = db.collection("items");

    let stripe_secret = std::env::var("STRIPE_SECRET_KEY")
        .map_err(|_| "STRIPE_SECRET_KEY must be set")?;
    let stripe_client = StripeClient::new(stripe_secret);

    // Get item from DB
    let item = get_item_by_id(db,item_id)
        .await?
        .ok_or("Item not found")?;

    let pm_id: PaymentMethodId = pay_id.parse()?; 
    let payment_method = PaymentMethod::retrieve(&stripe_client, &pm_id, &[]).await?;

    let amount_cents = (item.price * 100.0).round() as i64;

    if item.price <= 0.0 {
        return Err("Item price must be greater than zero".into());
    }

    if pay_id.trim().is_empty() {
        return Err("Payment ID cannot be empty".into());
    }

    let create_params = CreatePaymentIntent {
    amount: amount_cents, 
    currency: Currency::USD,
    payment_method: Some(payment_method.id.clone().parse()?), // ✅ now inside the struct
    confirm: Some(true),
    application_fee_amount: None,
    confirmation_method: None, // ✅ fix: still required by Rust, but won't be sent to Stripe
    automatic_payment_methods: Some(stripe::CreatePaymentIntentAutomaticPaymentMethods {
        enabled: true,
        allow_redirects: Some(
            stripe::CreatePaymentIntentAutomaticPaymentMethodsAllowRedirects::Always,
        ),
    }),

    capture_method: None,
    customer: None,
    description: None,
    error_on_requires_action: None,
    expand: &[],
    mandate: None,
    mandate_data: None,
    metadata: None,
    off_session: None,
    on_behalf_of: None,
    payment_method_configuration: None,
    payment_method_data: None,
    payment_method_options: None,
    payment_method_types: None,
    radar_options: None,
    receipt_email: None,
    return_url: Some("https://MytGroup.com/payment-complete"),
    setup_future_usage: None,
    shipping: None,
    statement_descriptor: None,
    statement_descriptor_suffix: None,
    transfer_data: None,
    transfer_group: None,
    use_stripe_sdk: None,
    };


    let payment_intent: PaymentIntent = PaymentIntent::create(&stripe_client, create_params).await?;

    if payment_intent.status == PaymentIntentStatus::Succeeded {
        println!("✅ Payment complete!");
        return Ok(()); // <-- ✅ RETURN Result, not unit ()
    } else {
        println!(
            "⚠️ Payment requires further action: {:?}",
            payment_intent.next_action
        );
        return Ok(()); // <-- ✅ RETURN Result, not unit ()
    } 
     return Ok(()); 
}
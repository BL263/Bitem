use crate::models::{Item, User};
use mongodb::{Database, Collection};
//use mongodb::bson::{doc, oid::ObjectId};
use futures::stream::TryStreamExt;

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
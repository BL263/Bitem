use mongodb::{Client, Database};
use dotenv::dotenv;
use std::env;

pub async fn connect_db() -> Database {
    dotenv().ok();
    let client_uri = env::var("MONGODB_URI").expect("Missing MONGODB_URI");
    let db_name = env::var("DB_NAME").expect("Missing DB_NAME");
    let client = Client::with_uri_str(&client_uri).await.unwrap();
    client.database(&db_name)
}
use actix_web::{ HttpServer, App };
use std::env;
//use std::fs;
mod db;
mod models;
mod services;
mod controllers;
mod routes;
//use dotenv::dotenv;
//use mongodb::bson::oid::ObjectId;
use crate::models::Item;
use actix_cors::Cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    /*     let env_path = "./.env";

    if fs::metadata(env_path).is_ok() {
        println!(".env file FOUND at {}", env_path);
    } else {
        println!(".env file NOT FOUND at {}", env_path);
    }

    match dotenv() {
        Ok(_) => println!(".env loaded ✅"),
        Err(e) => println!("Failed to load .env ❌: {:?}", e),
    }

    dotenv().ok();
    let db = db::connect_db().await;

    let test_item = Item {
        id: None,
        title: "Test Item".into(),
        description: "Test Description".into(),
        price: 42.0,
        seller_id: ObjectId::parse_str("6603b7db7a7a5c9c4b3e77e2").unwrap(), 
    };

    match services::create_item(&db, test_item).await {
        Ok(_) => println!("✅ Item inserted from test!"),
        Err(e) => println!("❌ Failed to insert test item: {}", e),
    } */
    /*let db = db::connect_db().await;

     match services::list_items(&db).await {
        Ok(items) => {
            println!("📦 Found {} items:", items.len());
            for item in items {
                println!("{:?}", item);
            }
        },
        Err(e) => println!("❌ Failed to list items: {}", e),
    } 

    println!("MONGODB_URI = {:?}", env::var("MONGODB_URI"));
    println!("DB_NAME = {:?}", env::var("DB_NAME"));*/
    HttpServer::new(move || { App::new()
        .wrap(
            Cors::default()
                .allowed_origin(&env::var("React_Address_For_CORS")
                .unwrap_or_else(|_| "http://localhost:3000".to_string())) // Replace with your frontend's URL
                .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                .allowed_headers(vec!["Content-Type", "Authorization"])
                .supports_credentials()
        )
        .configure(routes::config) })
    
        .bind("127.0.0.1:8080")?
        .run().await
}

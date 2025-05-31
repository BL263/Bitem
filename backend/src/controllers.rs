use crate::{services, models, db};
use actix_web::{web, HttpResponse, Responder};
use mongodb::{bson::{doc, oid::ObjectId}};


pub async fn create_item(item: web::Json<models::Item>) -> HttpResponse {
    let database = db::connect_db().await;
    match services::create_item(&database, item.into_inner()).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub async fn list_items() -> HttpResponse {
    let database = db::connect_db().await;
    match services::list_items(&database).await {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}


pub async fn get_item(path: web::Path<String>) -> impl Responder {
    let database = db::connect_db().await;

    let id = path.into_inner();

    let obj_id = match ObjectId::parse_str(&id) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid ObjectId"),
    };

    let collection = database.collection::<models::Item>("items");

    match collection.find_one(doc! { "_id": obj_id }, None).await {
        Ok(Some(item)) => HttpResponse::Ok().json(item),
        Ok(None) => HttpResponse::NotFound().body("Item not found"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

pub async fn buy_item(path: web::Path<String>) -> impl Responder {
    let database = db::connect_db().await;

    let input = path.into_inner();
    let parts: Vec<&str> = input.split('-').collect();
    if parts.len() != 2 {
        return HttpResponse::BadRequest().body("Input must be in the format `${itemId}-${productId}`");
    }

    let item_id = parts[0];
    let pay_id = parts[1];

    let obj_id = match ObjectId::parse_str(item_id) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().body("Invalid ObjectId for itemId"),
    };

    match services::buy_item(&database, obj_id.to_string(), pay_id.to_string()).await {
        Ok(result_ids) => HttpResponse::Ok().json(result_ids),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

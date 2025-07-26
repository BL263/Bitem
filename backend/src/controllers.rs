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

pub async fn buy_item(body: web::Json<serde_json::Value>) -> impl Responder {
    let database = db::connect_db().await;
    let body_clone = body.clone();
   
    let body_str = match serde_json::to_string(&body) {
        Ok(s) => s,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to serialize request body"),
    };

    println!("Request body as string: {}", body_str);
    let purchase: models::Purchase = match serde_json::from_value(body_clone) {
        Ok(p) => p,
        Err(_) => return HttpResponse::BadRequest().body("Invalid request body"),
    };

    let item_id = &purchase.i_Id;
    let pay_id = &purchase.p_Id;

    match services::buy_item(&database, item_id.to_string(), pay_id.to_string()).await {
        Ok(result_ids) => HttpResponse::Ok().json(result_ids),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

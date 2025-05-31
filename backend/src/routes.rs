use actix_web::web;
use crate::controllers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/items", web::post().to(controllers::create_item))
            .route("/items", web::get().to(controllers::list_items))   
            .route("/item/{id}", web::get().to(controllers::get_item))
            .route("/buy/{id}", web::get().to(controllers::buy_item)),           
        );
}
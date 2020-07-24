use actix_web::web;

use crate::handlers::*;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::scope("/api/v1")
                .route("/items", web::get().to(get_items))
                .route("/item/{id}", web::get().to(get_item))
                .route("/list/{id}", web::get().to(get_items_of_list))
                .route("/list/{id}", web::post().to(post_item)),
            )
        .service(
            web::scope("/")
                .route("", web::get().to(index)),
        );
}
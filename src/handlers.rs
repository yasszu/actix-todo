use actix_web::{web, Responder, Error, HttpResponse};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use crate::models::*;
use crate::actions;

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub async fn index() -> impl Responder {
    web::HttpResponse::Ok().body("OK")
}

pub async fn get_item(pool: web::Data<DbPool>, id: web::Path<i64>) -> Result<HttpResponse, Error> {
    let id = id.into_inner();
    let conn = pool.get().expect("couldn't get db connection from pool");
    let item = web::block(move || actions::find_item(id, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    Ok(HttpResponse::Ok().json(item))
}

pub async fn post_item(pool: web::Data<DbPool>, list_id: web::Path<i64>, form: web::Form<FormItem>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let result = web::block(move || actions::create_item(list_id.into_inner(), &form.title, &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    Ok(HttpResponse::Ok().json(result))
}

pub async fn get_items(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let items = web::block(move || actions::find_items(&conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    Ok(HttpResponse::Ok().json(items))
}

pub async fn get_items_of_list(pool: web::Data<DbPool>, list_id: web::Path<i64>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");
    let items: Vec<(List, Item)> = web::block(move || actions::find_items_of_list(list_id.into_inner(), &conn))
        .await
        .map_err(|e| {
            eprintln!("{}", e);
            HttpResponse::InternalServerError().finish()
        })?;
    let result: Vec<Item> = items
        .into_iter()
        .map(|(_, item)| item)
        .collect::<Vec<Item>>();
    Ok(HttpResponse::Ok().json(result))
}
use serde::Serialize;
use diesel::Queryable;

#[derive(Serialize)]
pub struct Status {
    pub status: String
}

#[derive(Queryable)]
pub struct List {
    pub id: i32,
    pub title: String,
}

#[derive(Queryable)]
pub struct Item {
    pub id: i32,
    pub title: String,
    pub checked: bool,
    pub list_id: i32,
}

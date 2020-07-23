use serde::Serialize;
use diesel::Queryable;

#[derive(Serialize, Queryable)]
pub struct List {
    pub id: i64,
    pub title: String,
}

#[derive(Serialize, Queryable)]
pub struct Item {
    pub id: i64,
    pub title: String,
    pub checked: i8,
    pub list_id: i64,
}

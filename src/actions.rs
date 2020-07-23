use diesel::prelude::*;
use crate::models::*;

type Error = diesel::result::Error;

pub fn find_item(_id: i64, conn: &MysqlConnection) -> Result<Vec<Item>, Error> {
    use actix_todo::schema::items::dsl::*;
    let item = items
        .filter(id.eq(_id))
        .load::<Item>(conn)
        .expect("Error loading posts");
    Ok(item)
}

pub fn find_items(conn: &MysqlConnection) -> Result<Vec<Item>, Error> {
    use actix_todo::schema::items::dsl::*;
    let item = items
        .load::<Item>(conn)
        .expect("Error loading posts");
    Ok(item)
}
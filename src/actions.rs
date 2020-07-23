use diesel::prelude::*;
use crate::models::*;

pub fn find_item(_id: i64, conn: &MysqlConnection) -> Result<Vec<Item>, diesel::result::Error> {
    use actix_todo::schema::items::dsl::*;
    let item = items
        .filter(id.eq(_id))
        .load::<Item>(conn)
        .expect("Error loading posts");
    Ok(item)
}
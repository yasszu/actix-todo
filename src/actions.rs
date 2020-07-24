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

pub fn find_items_of_list(list_id: i64, conn: &MysqlConnection) -> Result<Vec<(List, Item)>, Error> {
    use actix_todo::schema::items::dsl::items;
    use actix_todo::schema::lists::dsl::*;
    let result = lists
        .filter(id.eq(list_id))
        .inner_join(items)
        .load::<(List, Item)>(conn)
        .expect("Error loading posts");
    Ok(result)
}

pub fn find_items(conn: &MysqlConnection) -> Result<Vec<Item>, Error> {
    use actix_todo::schema::items::dsl::*;
    let item = items
        .load::<Item>(conn)
        .expect("Error loading posts");
    Ok(item)
}
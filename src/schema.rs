table! {
    items (id) {
        id -> Bigint,
        title -> Varchar,
        checked -> Tinyint,
        list_id -> Bigint,
    }
}

table! {
    lists (id) {
        id -> Bigint,
        title -> Varchar,
    }
}

joinable!(items -> lists (list_id));

allow_tables_to_appear_in_same_query!(
    items,
    lists,
);

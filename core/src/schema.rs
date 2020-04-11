table! {
    users (id) {
        id -> Integer,
        email -> Varchar,
        name -> Varchar,
        password -> Varchar,
        create_time -> Timestamp,
        update_time -> Timestamp,
    }
}

table! {
    categories (id) {
        id -> Integer,
        name -> Varchar,
        create_time -> Timestamp,
        update_time -> Timestamp,
    }
}

table! {
    notes (id) {
        id -> Integer,
        user_id -> Integer,
        category_id -> Nullable<Integer>,
        title -> Varchar,
        data -> Text,
        create_time -> Timestamp,
        update_time -> Timestamp,
    }
}

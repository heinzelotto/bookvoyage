// @generated automatically by Diesel CLI.

diesel::table! {
    book_logs (id) {
        id -> Int4,
        book_id -> Int4,
        commenter -> Varchar,
        comment -> Text,
        lat -> Float4,
        lon -> Float4,
        user_id -> Nullable<Int4>,
    }
}

diesel::table! {
    books (id) {
        id -> Int4,
        title -> Varchar,
        author -> Varchar,
        code -> Varchar,
        user_id -> Nullable<Int4>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password_hash -> Varchar,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(book_logs -> users (user_id));
diesel::joinable!(books -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(book_logs, books, users,);

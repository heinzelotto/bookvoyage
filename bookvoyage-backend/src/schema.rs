// @generated automatically by Diesel CLI.

diesel::table! {
    book_logs (id) {
        id -> Int4,
        book_id -> Int4,
        commenter -> Varchar,
        comment -> Text,
        lat -> Float4,
        lon -> Float4,
    }
}

diesel::table! {
    books (id) {
        id -> Int4,
        title -> Varchar,
        author -> Varchar,
        code -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(book_logs, books,);

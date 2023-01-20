use crate::schema::*;
use diesel;
use diesel::prelude::*;
use serde_derive::Serialize;

#[derive(Queryable, Serialize, Debug)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub code: String,
}

#[derive(Insertable)]
#[diesel(table_name = books)]
pub struct NewBook {
    pub title: String,
    pub author: String,
    pub code: String,
}

#[derive(Queryable, Serialize, Debug)]
pub struct BookLog {
    pub id: i32,
    pub bookid: i32,
    pub commenter: String,
    pub comment: String,
    pub lat: f32,
    pub lon: f32,
}

#[derive(Insertable)]
#[diesel(table_name = book_logs)]
pub struct NewBookLog {
    pub book_id: i32,
    pub commenter: String,
    pub comment: String,
    pub lat: f32,
    pub lon: f32,
}

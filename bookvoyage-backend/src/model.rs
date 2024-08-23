use crate::schema::*;
use crate::schema::*;
use chrono::{DateTime, NaiveDateTime, Utc};
use diesel;
use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    #[serde(with = "timestamp_format")]
    pub created_at: Option<NaiveDateTime>,
}

mod timestamp_format {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let dt = DateTime::<Utc>::from_naive_utc_and_offset(date.unwrap(), Utc);
        serializer.serialize_str(&dt.to_rfc3339())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DateTime::parse_from_rfc3339(&s)
            .map(|dt| Some(dt.naive_utc()))
            .map_err(serde::de::Error::custom)
    }
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password_hash: &'a str,
}

#[derive(Queryable, Serialize, Debug)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub code: String,
    pub user_id: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = books)]
pub struct NewBook<'a> {
    pub title: &'a str,
    pub author: &'a str,
    pub code: &'a str,
    pub user_id: Option<i32>, // FIXME: not optional
}

#[derive(Queryable, Serialize, Debug)]
pub struct BookLog {
    pub id: i32,
    pub book_id: i32,
    pub commenter: String, // FIXME: replaced by user_id
    pub comment: String,
    pub lat: f32,
    pub lon: f32,
    pub user_id: Option<i32>, // FIXME: not optional
}

#[derive(Insertable)]
#[diesel(table_name = book_logs)]
pub struct NewBookLog<'a> {
    pub book_id: i32,
    pub commenter: &'a str, // FIXME: replaced by user_id
    pub comment: &'a str,
    pub lat: f32,
    pub lon: f32,
    pub user_id: Option<i32>, // FIXME: not optional
}

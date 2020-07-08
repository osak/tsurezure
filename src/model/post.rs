extern crate chrono;

use chrono::{DateTime, Utc};
use diesel::{Queryable, Insertable};
use crate::schema::posts;

#[derive(Clone, Queryable)]
pub struct Post {
    pub id: i32,
    pub body: String,
    pub posted_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost {
    pub body: String,
    pub posted_at: DateTime<Utc>,
}
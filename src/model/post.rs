extern crate chrono;

use chrono::{DateTime, Utc};
use diesel::Queryable;

#[derive(Clone, Queryable)]
pub struct Post {
    pub id: i32,
    pub body: String,
    pub posted_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
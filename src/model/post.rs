extern crate chrono;

use chrono::{DateTime, Utc};

pub struct Post {
    pub id: i32,
    pub body: String,
    pub posted_at: DateTime<Utc>,
}
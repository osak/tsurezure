extern crate chrono;

use chrono::{Utc};

pub struct Post {
    id: u32,
    body: String,
    posted_at: Utc,
}
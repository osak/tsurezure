use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::model;

#[derive(Serialize)]
pub struct Post {
    pub id: i32,
    pub body: String,
    pub posted_at: DateTime<Utc>,
}

impl Into<Post> for model::Post {
    fn into(self: model::Post) -> Post {
        Post {
            id: self.id,
            body: self.body,
            posted_at: self.posted_at
        }
    }
}
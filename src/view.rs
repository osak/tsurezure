use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::model;

#[derive(Serialize)]
pub struct Post {
    pub id: i32,
    pub body: String,
    pub posted_at: DateTime<Utc>,
}

impl From<model::Post> for Post {
    fn from(other: model::Post) -> Post {
        let formatted_body = other.body.split("\n")
            .into_iter()
            .map(|line| format!("<p>{}</p>", line))
            .collect::<Vec<String>>()
            .join("");
        Post {
            id: other.id,
            body: formatted_body,
            posted_at: other.posted_at
        }
    }
}
use chrono::{DateTime, Utc};
use serde::Serialize;
use pulldown_cmark::{Parser, html};

use crate::model;

#[derive(Serialize)]
pub struct Post {
    pub id: i32,
    pub body: String,
    pub posted_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<model::Post> for Post {
    fn from(other: model::Post) -> Post {
        let parser = Parser::new(&other.body);
        let mut buf = String::new();
        html::push_html(&mut buf, parser);

        Post {
            id: other.id,
            body: buf,
            posted_at: other.posted_at,
            updated_at: other.updated_at,
        }
    }
}

#[derive(Serialize)]
pub struct RawPost {
    pub id: i32,
    pub body: String,
    pub posted_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<model::Post> for RawPost {
    fn from(other: model::Post) -> RawPost {
        RawPost {
            id: other.id,
            body: other.body,
            posted_at: other.posted_at,
            updated_at: other.updated_at,
        }
    }
}
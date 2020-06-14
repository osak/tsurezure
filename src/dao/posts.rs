use deadpool_postgres::{ClientWrapper};
use tokio_postgres::{Row};
use crate::model::Post;
use std::io::Error;

pub async fn find_recent(client: &ClientWrapper, limit: i64) -> Result<Vec<Post>, Error> {
    let rows: Vec<Row> = client.query("SELECT * FROM posts ORDER BY posted_at DESC LIMIT $1", &[&limit]).await.unwrap();
    let posts = rows.into_iter().map(|row| {
        Post {
            id: row.get("id"),
            body: row.get("body"),
            posted_at: row.get("posted_at"),
        }
    }).collect();
    Ok(posts)
}
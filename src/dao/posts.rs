use deadpool_postgres::{ClientWrapper};
use tokio_postgres::{Row};
use crate::model::Post;
use crate::dao::DBError;

fn parse_row(row: &Row) -> Post {
    Post {
        id: row.get("id"),
        body: row.get("body"),
        posted_at: row.get("posted_at"),
    }
}

pub async fn find_recent(client: &ClientWrapper, limit: i64) -> Result<Vec<Post>, DBError> {
    let rows: Vec<Row> = client.query("SELECT * FROM posts ORDER BY posted_at DESC LIMIT $1", &[&limit]).await.unwrap();
    let posts = rows.into_iter().map(|row| parse_row(&row)).collect();
    Ok(posts)
}

pub async fn find(client: &ClientWrapper, from_id: i32, limit: i64) -> Result<Vec<Post>, DBError> {
    let rows: Vec<Row> = client.query("SELECT * FROM posts WHERE id <= $1 ORDER BY posted_at DESC LIMIT $2", &[&from_id, &limit]).await.unwrap();
    let posts = rows.into_iter().map(|row| parse_row(&row)).collect();
    Ok(posts)
}

pub async fn save(client: &ClientWrapper, post: Post) -> Result<i32, DBError> {
    let rows: Vec<Row> = client.query("INSERT INTO posts (body, posted_at) VALUES ($1, $2) RETURNING id", &[&post.body, &post.posted_at])
        .await
        .map_err(|err| DBError::new("Failed to save a post", err))?;
    Ok(rows[0].get(0))
}
use rocket::serde::json::Json;
use rocket::{State, response::status::Custom};
use tokio_postgres::Client;
use crate::models::Category;
use crate::db::execute_query;

#[post("/api/categories", data = "<category>")]
pub async fn add_category(
    conn: &State<Client>,
    category: Json<Category>
) -> Result<Json<Vec<Category>>, Custom<String>> {
    let slug = category.slug.clone().unwrap_or_else(|| {
        category.name.to_lowercase()
            .chars()
            .map(|c| match c {
                'a'..='z' | '0'..='9' => c,
                ' ' => '-',
                _ => '-',
            })
            .collect::<String>()
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>()
            .join("-")
    });
    
    execute_query(
        conn,
        "INSERT INTO categories (name, slug, description, parent_id) VALUES ($1, $2, $3, $4)",
        &[&category.name, &slug, &category.description, &category.parent_id]
    ).await?;
    get_categories(conn).await
}

#[get("/api/categories")]
pub async fn get_categories(conn: &State<Client>) -> Result<Json<Vec<Category>>, Custom<String>> {
    let cats = conn
        .query("SELECT id, name, slug, description, parent_id FROM categories ORDER BY position, name", &[]).await
        .map_err(|e| Custom(rocket::http::Status::InternalServerError, e.to_string()))?
        .iter()
        .map(|row| Category {
            id: Some(row.get(0)),
            name: row.get(1),
            slug: row.get(2),
            description: row.get(3),
            parent_id: row.get(4),
        })
        .collect::<Vec<Category>>();

    Ok(Json(cats))
}

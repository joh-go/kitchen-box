use rocket::serde::json::Json;
use rocket::{State, response::status::Custom, http::Status};
use tokio_postgres::Client;
use serde_json::Value as JsonValue;
use crate::models::Recipe;
use crate::db::execute_query;

#[post("/api/recipes", data = "<recipe>")]
pub async fn add_recipe(
    conn: &State<Client>,
    recipe: Json<Recipe>
) -> Result<Json<Vec<Recipe>>, Custom<String>> {
    let ingredients_str = serde_json::to_string(&recipe.ingredients).map_err(|e| Custom(Status::BadRequest, e.to_string()))?;
    let steps_str = serde_json::to_string(&recipe.steps).map_err(|e| Custom(Status::BadRequest, e.to_string()))?;

    execute_query(
        conn,
        "INSERT INTO recipes (title, slug, short_description, ingredients, steps, prep_minutes, cook_minutes, servings, notes, author_id, is_public) VALUES ($1, $2, $3, $4::jsonb, $5::jsonb, $6, $7, $8, $9, $10, $11)",
        &[&recipe.title, &recipe.slug, &recipe.short_description, &ingredients_str, &steps_str, &recipe.prep_minutes, &recipe.cook_minutes, &recipe.servings, &recipe.notes, &recipe.author_id, &recipe.is_public]
    ).await?;

    get_recipes(conn).await
}

#[get("/api/recipes")]
pub async fn get_recipes(conn: &State<Client>) -> Result<Json<Vec<Recipe>>, Custom<String>> {
    let rows = conn
        .query("SELECT id, title, slug, short_description, ingredients, steps, prep_minutes, cook_minutes, servings, notes, author_id, is_public FROM recipes ORDER BY created_at DESC", &[]).await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    let mut recipes = Vec::new();
    for row in rows.iter() {
        let ingredients_json: String = row.get(4);
        let steps_json: String = row.get(5);
        let ingredients: JsonValue = serde_json::from_str(&ingredients_json).unwrap_or(JsonValue::Null);
        let steps: JsonValue = serde_json::from_str(&steps_json).unwrap_or(JsonValue::Null);

        recipes.push(Recipe {
            id: Some(row.get(0)),
            title: row.get(1),
            slug: row.get(2),
            short_description: row.get(3),
            ingredients,
            steps,
            prep_minutes: row.get(6),
            cook_minutes: row.get(7),
            servings: row.get(8),
            notes: row.get(9),
            author_id: row.get(10),
            is_public: row.get(11),
        });
    }

    Ok(Json(recipes))
}

#[get("/api/recipes/<id>")]
pub async fn get_recipe(conn: &State<Client>, id: i32) -> Result<Json<Recipe>, Custom<String>> {
    let row = conn
        .query_one("SELECT id, title, slug, short_description, ingredients, steps, prep_minutes, cook_minutes, servings, notes, author_id, is_public FROM recipes WHERE id = $1", &[&id]).await
        .map_err(|e| Custom(Status::NotFound, e.to_string()))?;

    let ingredients_json: String = row.get(4);
    let steps_json: String = row.get(5);
    let ingredients: JsonValue = serde_json::from_str(&ingredients_json).unwrap_or(JsonValue::Null);
    let steps: JsonValue = serde_json::from_str(&steps_json).unwrap_or(JsonValue::Null);

    Ok(Json(Recipe {
        id: Some(row.get(0)),
        title: row.get(1),
        slug: row.get(2),
        short_description: row.get(3),
        ingredients,
        steps,
        prep_minutes: row.get(6),
        cook_minutes: row.get(7),
        servings: row.get(8),
        notes: row.get(9),
        author_id: row.get(10),
        is_public: row.get(11),
    }))
}

#[delete("/api/recipes/<id>")]
pub async fn delete_recipe(conn: &State<Client>, id: i32) -> Result<Status, Custom<String>> {
    execute_query(conn, "DELETE FROM recipes WHERE id = $1", &[&id]).await?;
    Ok(Status::NoContent)
}

#[post("/api/recipes/<rid>/categories/<cid>")]
pub async fn assign_category(conn: &State<Client>, rid: i32, cid: i32) -> Result<Status, Custom<String>> {
    execute_query(conn, "INSERT INTO recipe_categories (recipe_id, category_id) VALUES ($1, $2) ON CONFLICT DO NOTHING", &[&rid, &cid]).await?;
    Ok(Status::Created)
}

use crate::db::execute_query;
use crate::models::Recipe;
use postgres_types::Json as PgJson;
use rocket::serde::json::Json;
use rocket::{State, http::Status, response::status::Custom};
use serde_json::Value as JsonValue;
use tokio_postgres::Client;

#[post("/api/recipes", data = "<recipe>")]
pub async fn add_recipe(
    conn: &State<Client>,
    recipe: Json<Recipe>,
) -> Result<Json<Vec<Recipe>>, Custom<String>> {
    let ingredients_val = PgJson(recipe.ingredients.clone());
    let steps_val = PgJson(recipe.steps.clone());

    let slug_val: &str = recipe.slug.as_deref().unwrap_or("");

    execute_query(
        conn,
        "INSERT INTO recipes (title, slug, short_description, ingredients, steps, prep_minutes, cook_minutes, servings, notes, author_id, is_public) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
        &[&recipe.title, &slug_val, &recipe.short_description, &ingredients_val, &steps_val, &recipe.prep_minutes, &recipe.cook_minutes, &recipe.servings, &recipe.notes, &recipe.author_id, &recipe.is_public]
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
        let ingredients_pg: Option<PgJson<JsonValue>> = row.get(4);
        let steps_pg: Option<PgJson<JsonValue>> = row.get(5);
        let ingredients: JsonValue = ingredients_pg.map(|p| p.0).unwrap_or(JsonValue::Null);
        let steps: JsonValue = steps_pg.map(|p| p.0).unwrap_or(JsonValue::Null);

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

    let ingredients_pg: Option<PgJson<JsonValue>> = row.get(4);
    let steps_pg: Option<PgJson<JsonValue>> = row.get(5);
    let ingredients: JsonValue = ingredients_pg.map(|p| p.0).unwrap_or(JsonValue::Null);
    let steps: JsonValue = steps_pg.map(|p| p.0).unwrap_or(JsonValue::Null);

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
pub async fn assign_category(
    conn: &State<Client>,
    rid: i32,
    cid: i32,
) -> Result<Status, Custom<String>> {
    execute_query(conn, "INSERT INTO recipe_categories (recipe_id, category_id) VALUES ($1, $2) ON CONFLICT DO NOTHING", &[&rid, &cid]).await?;
    Ok(Status::Created)
}

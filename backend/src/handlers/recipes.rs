use crate::db::execute_query;
use crate::models::Recipe;
use crate::auth::AuthenticatedUser;
use postgres_types::Json as PgJson;
use rocket::serde::json::Json;
use rocket::{http::Status, response::status::Custom, State};
use serde_json::Value as JsonValue;
use tokio_postgres::Client;

#[post("/api/recipes", data = "<recipe>")]
pub async fn add_recipe(
    conn: &State<Client>,
    auth_user: AuthenticatedUser,
    recipe: Json<Recipe>,
) -> Result<Json<Recipe>, Custom<String>> {
    let ingredients_val = PgJson(recipe.ingredients.clone());
    let steps_val = PgJson(recipe.steps.clone());

    let slug_val: &str = recipe.slug.as_deref().unwrap_or("");

    let row = conn
        .query_one(
            "INSERT INTO recipes (title, slug, short_description, ingredients, steps, prep_minutes, cook_minutes, servings, notes, author_id, is_public) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) RETURNING id, title, slug, short_description, ingredients, steps, prep_minutes, cook_minutes, servings, notes, author_id, is_public",
            &[&recipe.title, &slug_val, &recipe.short_description, &ingredients_val, &steps_val, &recipe.prep_minutes, &recipe.cook_minutes, &recipe.servings, &recipe.notes, &auth_user.user_id, &recipe.is_public],
        ).await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

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

#[get("/api/my-recipes")]
pub async fn get_my_recipes(conn: &State<Client>, auth_user: AuthenticatedUser) -> Result<Json<Vec<Recipe>>, Custom<String>> {
    let rows = conn
        .query("SELECT id, title, slug, short_description, ingredients, steps, prep_minutes, cook_minutes, servings, notes, author_id, is_public FROM recipes WHERE author_id = $1 ORDER BY created_at DESC", &[&auth_user.user_id]).await
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
pub async fn delete_recipe(
    conn: &State<Client>,
    auth_user: AuthenticatedUser,
    id: i32,
) -> Result<Status, Custom<String>> {
    // Check if user owns this recipe
    let recipe_row = conn
        .query_one("SELECT author_id FROM recipes WHERE id = $1", &[&id])
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;
    
    let author_id: i32 = recipe_row.get(0);
    
    if author_id != auth_user.user_id {
        return Err(Custom(Status::Forbidden, "You can only delete your own recipes".to_string()));
    }
    
    execute_query(conn, "DELETE FROM recipes WHERE id = $1", &[&id]).await?;
    Ok(Status::NoContent)
}

#[put("/api/recipes/<id>", data = "<recipe>")]
pub async fn update_recipe(
    conn: &State<Client>,
    auth_user: AuthenticatedUser,
    id: i32,
    recipe: Json<Recipe>,
) -> Result<Json<Recipe>, Custom<String>> {
    println!("DEBUG: Starting update_recipe for id: {}", id);
    
    // Check if user owns this recipe
    let recipe_row = conn
        .query_one("SELECT author_id FROM recipes WHERE id = $1", &[&id])
        .await
        .map_err(|e| {
            println!("DEBUG: Error checking ownership: {}", e);
            Custom(Status::InternalServerError, e.to_string())
        })?;
    
    let author_id: i32 = recipe_row.get(0);
    println!("DEBUG: Recipe author_id: {}, auth_user_id: {}", author_id, auth_user.user_id);
    
    if author_id != auth_user.user_id {
        return Err(Custom(Status::Forbidden, "You can only edit your own recipes".to_string()));
    }
    
    let ingredients_val = PgJson(recipe.ingredients.clone());
    let steps_val = PgJson(recipe.steps.clone());

    // Handle optional fields properly
    let slug_val: &str = recipe.slug.as_deref().unwrap_or("");
    let short_desc_val: &str = recipe.short_description.as_deref().unwrap_or("");
    let prep_val: Option<i32> = recipe.prep_minutes;
    let cook_val: Option<i32> = recipe.cook_minutes;
    let servings_val: Option<i32> = recipe.servings;
    let notes_val: &str = recipe.notes.as_deref().unwrap_or("");
    let is_public_val: bool = recipe.is_public.unwrap_or(true);

    println!("DEBUG: About to execute full UPDATE query");
    let result = conn.execute(
        "UPDATE recipes SET title=$1, slug=$2, short_description=$3, ingredients=$4, steps=$5, prep_minutes=$6, cook_minutes=$7, servings=$8, notes=$9, is_public=$10, updated_at = now() WHERE id=$11",
        &[&recipe.title, &slug_val, &short_desc_val, &ingredients_val, &steps_val, &prep_val, &cook_val, &servings_val, &notes_val, &is_public_val, &id]
    ).await;
    
    match result {
        Ok(rows_affected) => {
            println!("DEBUG: UPDATE successful, rows affected: {}", rows_affected);
        }
        Err(e) => {
            println!("DEBUG: UPDATE failed: {}", e);
            return Err(Custom(Status::InternalServerError, format!("Update failed: {}", e)));
        }
    }

    println!("DEBUG: About to call get_recipe");
    match get_recipe(conn, id).await {
        Ok(recipe) => {
            println!("DEBUG: get_recipe successful");
            Ok(recipe)
        }
        Err(e) => {
            println!("DEBUG: get_recipe failed: {:?}", e);
            Err(Custom(Status::InternalServerError, format!("Get recipe failed: {:?}", e)))
        }
    }
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

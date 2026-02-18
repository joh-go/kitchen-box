use gloo::net::http::Request;
use serde_json::json;
use serde_json::Value as JsonValue;
use shared_types::{Category, Recipe};

const BASE: &str = "http://127.0.0.1:8000";

pub async fn get_recipes() -> Result<Vec<Recipe>, String> {
    let resp = Request::get(&format!("{}/api/recipes", BASE))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    resp.json::<Vec<Recipe>>().await.map_err(|e| e.to_string())
}

pub async fn get_categories() -> Result<Vec<Category>, String> {
    let resp = Request::get(&format!("{}/api/categories", BASE))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    resp.json::<Vec<Category>>()
        .await
        .map_err(|e| e.to_string())
}

pub async fn get_recipe(id: i32) -> Result<Recipe, String> {
    let resp = Request::get(&format!("{}/api/recipes/{}", BASE, id))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    resp.json::<Recipe>().await.map_err(|e| e.to_string())
}

pub async fn create_recipe(recipe: &Recipe) -> Result<(), String> {
    let body = serde_json::to_string(recipe).map_err(|e| e.to_string())?;
    let resp = Request::post(&format!("{}/api/recipes", BASE))
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        Ok(())
    } else {
        Err("server error".into())
    }
}

pub async fn delete_recipe(id: i32) -> Result<(), String> {
    let resp = Request::delete(&format!("{}/api/recipes/{}", BASE, id))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        Ok(())
    } else {
        Err("server error".into())
    }
}

pub async fn assign_category(recipe_id: i32, category_id: i32) -> Result<(), String> {
    let resp = Request::post(&format!(
        "{}/api/recipes/{}/categories/{}",
        BASE, recipe_id, category_id
    ))
    .send()
    .await
    .map_err(|e| e.to_string())?;

    if resp.ok() {
        Ok(())
    } else {
        Err("server error".into())
    }
}

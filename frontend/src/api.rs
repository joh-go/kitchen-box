use gloo::net::http::{Request, Method};
use serde_json::json;
use shared_types::{Category, Recipe, User};
use web_sys::window;

const BASE: &str = "http://127.0.0.1:8000";

// Helper function to get auth token from localStorage
fn get_auth_header() -> Option<String> {
    if let Some(window) = window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(Some(token)) = storage.get_item("auth_token") {
                return Some(format!("Bearer {}", token));
            }
        }
    }
    None
}

// Helper function to check if user is logged in
pub fn is_logged_in() -> bool {
    if let Some(window) = window() {
        if let Ok(Some(storage)) = window.local_storage() {
            return storage.get_item("auth_token").is_ok_and(|t| t.is_some());
        }
    }
    false
}

// Helper function to get current user's ID from localStorage
pub fn get_current_user_id() -> Option<i32> {
    if let Some(window) = window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(Some(id_str)) = storage.get_item("user_id") {
                return id_str.parse::<i32>().ok();
            }
        }
    }
    None
}

// Helper function to get current user's name
pub fn get_current_user_name() -> Option<String> {
    if let Some(window) = window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(Some(name)) = storage.get_item("user_name") {
                return Some(name);
            }
        }
    }
    None
}

// Logout function
pub fn logout() {
    if let Some(window) = window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.remove_item("auth_token");
            let _ = storage.remove_item("user_email");
            let _ = storage.remove_item("user_name");
            let _ = storage.remove_item("user_id");
            // Redirect to home page
            let _ = window.location().set_href("/");
        }
    }
}

pub async fn get_recipes() -> Result<Vec<Recipe>, String> {
    let resp = Request::get(&format!("{}/api/recipes", BASE))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    resp.json::<Vec<Recipe>>().await.map_err(|e| e.to_string())
}

pub async fn get_my_recipes() -> Result<Vec<Recipe>, String> {
    let auth_header = get_auth_header().unwrap_or_else(|| "".to_string());
    let resp = Request::get(&format!("{}/api/my-recipes", BASE))
        .header("Authorization", &auth_header)
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

pub async fn create_category(name: &str) -> Result<serde_json::Value, String> {
    let auth_header = get_auth_header().unwrap_or_else(|| "".to_string());
    let request = Request::new(&format!("{}/api/categories", BASE))
        .method(Method::POST)
        .header("Content-Type", "application/json")
        .header("Authorization", &auth_header)
        .body(json!({"name": name}).to_string());

    let resp = request
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if resp.status() == 200 {
        resp.json()
            .await
            .map_err(|e| format!("JSON parsing error: {}", e))
    } else {
        Err(format!("Category creation failed: {}", resp.status()))
    }
}

pub async fn get_recipe(id: i32) -> Result<Recipe, String> {
    let resp = Request::get(&format!("{}/api/recipes/{}", BASE, id))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    resp.json::<Recipe>().await.map_err(|e| e.to_string())
}

pub async fn create_recipe(recipe: &Recipe) -> Result<Recipe, String> {
    let body = serde_json::to_string(recipe).map_err(|e| e.to_string())?;
    let mut request = Request::post(&format!("{}/api/recipes", BASE))
        .header("Content-Type", "application/json")
        .body(body);
    
    // Add Authorization header if token exists
    if let Some(auth_header) = get_auth_header() {
        request = request.header("Authorization", &auth_header);
    }
    
    let resp = request
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        resp.json::<Recipe>().await.map_err(|e| e.to_string())
    } else {
        Err(format!("Server error: {}", resp.status()))
    }
}

pub async fn update_recipe(id: i32, recipe: &Recipe) -> Result<Recipe, String> {
    let body = serde_json::to_string(recipe).map_err(|e| e.to_string())?;
    let mut request = Request::put(&format!("{}/api/recipes/{}", BASE, id))
        .header("Content-Type", "application/json")
        .body(body);
    
    // Add Authorization header if token exists
    if let Some(auth_header) = get_auth_header() {
        request = request.header("Authorization", &auth_header);
    }
    
    let resp = request
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        resp.json::<Recipe>().await.map_err(|e| e.to_string())
    } else {
        Err(format!("Server error: {}", resp.status()))
    }
}

pub async fn delete_recipe(id: i32) -> Result<(), String> {
    let mut request = Request::delete(&format!("{}/api/recipes/{}", BASE, id));
    
    // Add Authorization header if token exists
    if let Some(auth_header) = get_auth_header() {
        request = request.header("Authorization", &auth_header);
    }
    
    let resp = request
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        Ok(())
    } else {
        Err(format!("Server error: {}", resp.status()))
    }
}

pub async fn assign_category(recipe_id: i32, category_id: i32) -> Result<(), String> {
    let auth_header = get_auth_header().unwrap_or_else(|| "".to_string());
    let url = format!("{}/api/recipes/{}/categories/{}", BASE, recipe_id, category_id);
    
    let resp = Request::post(&url)
    .header("Authorization", &auth_header)
    .send()
    .await
    .map_err(|e| e.to_string())?;

    if resp.ok() {
        Ok(())
    } else {
        Err(format!("Server error: {}", resp.status()))
    }
}

// --- Users API ---
pub async fn get_users() -> Result<Vec<shared_types::User>, String> {
    let resp = Request::get(&format!("{}/api/users", BASE))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    resp.json::<Vec<shared_types::User>>()
        .await
        .map_err(|e| e.to_string())
}

pub async fn login(email: &str, password: &str) -> Result<serde_json::Value, String> {
    let request = Request::new(&format!("{}/api/auth/login", BASE))
        .method(Method::POST)
        .header("Content-Type", "application/json")
        .body(json!({
            "email": email,
            "password": password
        }).to_string());

    let resp = request
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if resp.status() == 200 {
        resp.json()
            .await
            .map_err(|e| format!("JSON parsing error: {}", e))
    } else {
        Err(format!("Login failed: {}", resp.status()))
    }
}

pub async fn create_user(user: &User) -> Result<serde_json::Value, String> {
    let request = Request::new(&format!("{}/api/users", BASE))
        .method(Method::POST)
        .header("Content-Type", "application/json")
        .body(json!(user).to_string());

    let resp = request
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if resp.status() == 200 {
        resp.json()
            .await
            .map_err(|e| format!("JSON parsing error: {}", e))
    } else {
        Err(format!("User creation failed: {}", resp.status()))
    }
}

pub async fn update_profile(name: &str, email: &str, current_password: &str, new_password: &str) -> Result<(), String> {
    let mut body = json!({
        "name": name,
        "email": email
    });
    
    if !current_password.is_empty() && !new_password.is_empty() {
        body["current_password"] = json!(current_password);
        body["new_password"] = json!(new_password);
    }

    let auth_header = get_auth_header().unwrap_or_else(|| "".to_string());
    let request = Request::new(&format!("{}/api/auth/me", BASE))
        .method(Method::PUT)
        .header("Content-Type", "application/json")
        .header("Authorization", &auth_header)
        .body(body.to_string());

    let resp = request
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if resp.status() == 200 {
        Ok(())
    } else {
        let error_text = resp.text().await.unwrap_or_else(|_| "Update failed".to_string());
        Err(format!("Profile update failed: {}", error_text))
    }
}

pub async fn get_current_user() -> Result<serde_json::Value, String> {
    let auth_header = get_auth_header().unwrap_or_else(|| "".to_string());
    let request = Request::new(&format!("{}/api/auth/me", BASE))
        .header("Content-Type", "application/json")
        .header("Authorization", &auth_header);

    let resp = request
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if resp.status() == 200 {
        resp.json()
            .await
            .map_err(|e| format!("JSON parsing error: {}", e))
    } else {
        Err(format!("Failed to get user: {}", resp.status()))
    }
}

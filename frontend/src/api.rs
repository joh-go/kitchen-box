use gloo::net::http::{Request, Method};
use serde_json::json;
use shared_types::{Category, Recipe, RecipeImage, User};
use web_sys::window;

const API_BASE: &str = match option_env!("API_BASE") {
    Some(base) => base,
    None => "http://127.0.0.1:8000",
};

fn get_base_url() -> String {
    API_BASE.to_string()
}

// Helper function to get auth token from localStorage
fn get_auth_header() -> Option<String> {
    let auth = home_hub_shared::restore_or_from_url();
    auth.token.map(|t| format!("Bearer {}", t))
}

// Helper function to check if user is logged in
pub fn is_logged_in() -> bool {
    home_hub_shared::restore_or_from_url().is_authenticated()
}

// Helper function to get current user's ID from localStorage
pub fn get_current_user_id() -> Option<i32> {
    let auth = home_hub_shared::restore_or_from_url();
    auth.user_id.and_then(|id| id.parse::<i32>().ok())
}

// Helper function to get current user's admin status
pub fn is_current_user_admin() -> bool {
    home_hub_shared::restore_or_from_url().is_admin
}

// Helper function to get current user's name
pub fn get_current_user_name() -> Option<String> {
    home_hub_shared::restore_or_from_url().username
}

// Logout function
pub fn logout() {
    let mut auth = home_hub_shared::restore_or_from_url();
    auth.logout();
    if let Some(window) = window() {
        let _ = window.location().set_href("/");
    }
}

pub async fn get_recipes() -> Result<Vec<Recipe>, String> {
    let base = get_base_url();
    let resp = Request::get(&format!("{}/api/recipes", base))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    resp.json::<Vec<Recipe>>().await.map_err(|e| e.to_string())
}

pub async fn get_my_recipes() -> Result<Vec<Recipe>, String> {
    let auth_header = get_auth_header().unwrap_or_else(|| "".to_string());
    let base = get_base_url();
    let resp = Request::get(&format!("{}/api/my-recipes", base))
        .header("Authorization", &auth_header)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    resp.json::<Vec<Recipe>>().await.map_err(|e| e.to_string())
}

pub async fn get_categories() -> Result<Vec<Category>, String> {
    let base = get_base_url();
    let resp = Request::get(&format!("{}/api/categories", base))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    resp.json::<Vec<Category>>()
        .await
        .map_err(|e| e.to_string())
}

pub async fn create_category(name: &str) -> Result<serde_json::Value, String> {
    let auth_header = get_auth_header().unwrap_or_else(|| "".to_string());
    let base = get_base_url();
    let request = Request::new(&format!("{}/api/categories", base))
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
        Err(home_hub_shared::check_auth_error(format!("Category creation failed: {}", resp.status())))
    }
}

pub async fn get_recipe(id: i32) -> Result<Recipe, String> {
    let base = get_base_url();
    let resp = Request::get(&format!("{}/api/recipes/{}", base, id))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    resp.json::<Recipe>().await.map_err(|e| e.to_string())
}

pub async fn create_recipe(recipe: &Recipe) -> Result<Recipe, String> {
    let body = serde_json::to_string(recipe).map_err(|e| e.to_string())?;
    let base = get_base_url();
    let mut request = Request::post(&format!("{}/api/recipes", base))
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
        Err(home_hub_shared::check_auth_error(format!("Server error: {}", resp.status())))
    }
}

pub async fn update_recipe(id: i32, recipe: &Recipe) -> Result<Recipe, String> {
    let body = serde_json::to_string(recipe).map_err(|e| e.to_string())?;
    let base = get_base_url();
    let mut request = Request::put(&format!("{}/api/recipes/{}", base, id))
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
        Err(home_hub_shared::check_auth_error(format!("Server error: {}", resp.status())))
    }
}

pub async fn delete_recipe(id: i32) -> Result<(), String> {
    let base = get_base_url();
    let mut request = Request::delete(&format!("{}/api/recipes/{}", base, id));
    
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
        Err(home_hub_shared::check_auth_error(format!("Server error: {}", resp.status())))
    }
}

pub async fn assign_category(recipe_id: i32, category_id: i32) -> Result<(), String> {
    let auth_header = get_auth_header().unwrap_or_else(|| "".to_string());
    let base = get_base_url();
    let url = format!("{}/api/recipes/{}/categories/{}", base, recipe_id, category_id);
    
    let resp = Request::post(&url)
    .header("Authorization", &auth_header)
    .send()
    .await
    .map_err(|e| e.to_string())?;

    if resp.ok() {
        Ok(())
    } else {
        Err(home_hub_shared::check_auth_error(format!("Server error: {}", resp.status())))
    }
}

pub async fn clear_categories(recipe_id: i32) -> Result<(), String> {
    let auth_header = get_auth_header().unwrap_or_else(|| "".to_string());
    let base = get_base_url();
    let url = format!("{}/api/recipes/{}/categories", base, recipe_id);
    
    let resp = Request::delete(&url)
    .header("Authorization", &auth_header)
    .send()
    .await
    .map_err(|e| e.to_string())?;

    if resp.ok() {
        Ok(())
    } else {
        Err(home_hub_shared::check_auth_error(format!("Server error: {}", resp.status())))
    }
}

// --- Users API ---
pub async fn get_users() -> Result<Vec<shared_types::User>, String> {
    let base = get_base_url();
    let resp = Request::get(&format!("{}/api/users", base))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    resp.json::<Vec<shared_types::User>>()
        .await
        .map_err(|e| e.to_string())
}

pub async fn login(username: &str, password: &str) -> Result<serde_json::Value, String> {
    let base = get_base_url();
    let request = Request::new(&format!("{}/api/auth/login", base))
        .method(Method::POST)
        .header("Content-Type", "application/json")
        .body(json!({
            "username": username,
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
        Err(home_hub_shared::check_auth_error(format!("Login failed: {}", resp.status())))
    }
}

pub async fn create_user(user: &User) -> Result<serde_json::Value, String> {
    let base = get_base_url();
    let request = Request::new(&format!("{}/api/users", base))
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
        Err(home_hub_shared::check_auth_error(format!("User creation failed: {}", resp.status())))
    }
}

pub async fn update_profile(name: &str, current_password: &str, new_password: &str) -> Result<(), String> {
    let mut body = json!({
        "name": name
    });
    
    if !current_password.is_empty() && !new_password.is_empty() {
        body["current_password"] = json!(current_password);
        body["new_password"] = json!(new_password);
    }

    let auth_header = get_auth_header().unwrap_or_else(|| "".to_string());
    let request = Request::new(&format!("{}/api/auth/me", get_base_url()))
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
        Err(home_hub_shared::check_auth_error(format!("Profile update failed: {}", error_text)))
    }
}

pub async fn get_current_user() -> Result<serde_json::Value, String> {
    let auth_header = get_auth_header().unwrap_or_else(|| "".to_string());
    let request = Request::new(&format!("{}/api/auth/me", get_base_url()))
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
        Err(home_hub_shared::check_auth_error(format!("Failed to get user: {}", resp.status())))
    }
}

// --- Images API ---
pub async fn get_recipe_images(recipe_id: i32) -> Result<Vec<RecipeImage>, String> {
    let auth_header = get_auth_header().unwrap_or_else(|| "".to_string());
    let base = get_base_url();
    let resp = Request::get(&format!("{}/api/recipes/{}/images", base, recipe_id))
        .header("Authorization", &auth_header)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    resp.json::<Vec<RecipeImage>>().await.map_err(|e| e.to_string())
}

pub async fn upload_recipe_image(recipe_id: i32, file: &web_sys::File) -> Result<RecipeImage, String> {
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;
    
    let auth_header = get_auth_header().unwrap_or_else(|| "".to_string());
    
    let file_name = file.name();
    
    // Use a simple FileReader implementation that works
    let reader = web_sys::FileReader::new().map_err(|e| format!("Failed to create FileReader: {:?}", e))?;
    
    // Create a promise that resolves when file is read
    let promise = js_sys::Promise::new(&mut |resolve, reject| {
        let file_clone = file.clone();
        let reject_clone = reject.clone();
        
        // Set up onload callback
        let onload = wasm_bindgen::closure::Closure::once(Box::new(move |event: web_sys::Event| {
            let target = event.target().unwrap();
            let reader = target.dyn_into::<web_sys::FileReader>().unwrap();
            let result = reader.result().unwrap();
            
            if result.is_instance_of::<js_sys::ArrayBuffer>() {
                // Convert ArrayBuffer to Uint8Array
                let array_buffer = result.dyn_into::<js_sys::ArrayBuffer>().unwrap();
                let uint8_array = js_sys::Uint8Array::new(&array_buffer);
                
                // Resolve with the Uint8Array directly
                resolve.call1(&wasm_bindgen::JsValue::NULL, &uint8_array).unwrap();
            } else {
                reject.call0(&wasm_bindgen::JsValue::from_str("Failed to read file as ArrayBuffer")).unwrap();
            }
        }));
        
        // Set up onerror callback
        let onerror = wasm_bindgen::closure::Closure::once(Box::new(move |_event: web_sys::Event| {
            reject_clone.call0(&wasm_bindgen::JsValue::from_str("Failed to read file")).unwrap();
        }));
        
        reader.set_onload(Some(onload.as_ref().unchecked_ref()));
        reader.set_onerror(Some(onerror.as_ref().unchecked_ref()));
        
        // Keep closures alive until they're called
        onload.forget();
        onerror.forget();
        
        // Start reading the actual file
        reader.read_as_array_buffer(&file_clone).unwrap();
    });
    
    // Wait for the file to be read
    let result = JsFuture::from(promise).await.map_err(|e| format!("File read error: {:?}", e))?;
    
    // Convert the result back to Vec<u8>
    let uint8_array = result.dyn_into::<js_sys::Uint8Array>()
        .map_err(|e| format!("Failed to convert to Uint8Array: {:?}", e))?;
    
    let mut file_bytes = vec![0u8; uint8_array.length() as usize];
    uint8_array.copy_to(&mut file_bytes);
    
    let base = get_base_url();
    let request = Request::new(&format!("{}/api/recipes/{}/images", base, recipe_id))
        .method(Method::POST)
        .header("Authorization", &auth_header)
        .header("Content-Type", "application/octet-stream")
        .header("X-Filename", &file_name)
        .header("X-File-Size", &file_bytes.len().to_string())
        .body(file_bytes);

    let resp = request
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        resp.json::<RecipeImage>().await.map_err(|e| e.to_string())
    } else {
        Err(home_hub_shared::check_auth_error(format!("Image upload failed: {}", resp.status())))
    }
}

pub async fn set_primary_image(recipe_id: i32, image_id: i32) -> Result<(), String> {
    let auth_header = get_auth_header().unwrap_or_else(|| "".to_string());
    let base = get_base_url();
    let url = format!("{}/api/recipes/{}/images/{}/primary", base, recipe_id, image_id);
    
    let resp = Request::put(&url)
        .header("Authorization", &auth_header)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        Ok(())
    } else {
        Err(home_hub_shared::check_auth_error(format!("Failed to set primary image: {}", resp.status())))
    }
}

pub async fn delete_recipe_image(recipe_id: i32, image_id: i32) -> Result<(), String> {
    let auth_header = get_auth_header().unwrap_or_else(|| "".to_string());
    let base = get_base_url();
    let url = format!("{}/api/recipes/{}/images/{}", base, recipe_id, image_id);
    
    let resp = Request::delete(&url)
        .header("Authorization", &auth_header)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        Ok(())
    } else {
        Err(home_hub_shared::check_auth_error(format!("Failed to delete image: {}", resp.status())))
    }
}

// Admin API functions

// Get all users (admin only)
pub async fn get_admin_users() -> Result<serde_json::Value, String> {
    let auth_header = get_auth_header().unwrap_or_default();
    let resp = Request::new(&format!("{}/api/admin/users", get_base_url()))
        .method(Method::GET)
        .header("Authorization", &auth_header)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        let response: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        Ok(response)
    } else {
        Err(home_hub_shared::check_auth_error(format!("Failed to get users: {}", resp.status())))
    }
}

// Create user (admin only)
pub async fn create_admin_user(user_data: serde_json::Value) -> Result<serde_json::Value, String> {
    let auth_header = get_auth_header().unwrap_or_default();
    let body = serde_json::to_string(&user_data).map_err(|e| e.to_string())?;
    
    let resp = Request::new(&format!("{}/api/admin/users", get_base_url()))
        .method(Method::POST)
        .header("Authorization", &auth_header)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        let response: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        Ok(response)
    } else {
        Err(home_hub_shared::check_auth_error(format!("Failed to create user: {}", resp.status())))
    }
}

// Update user (admin only)
pub async fn update_admin_user(user_id: i32, user_data: serde_json::Value) -> Result<serde_json::Value, String> {
    let auth_header = get_auth_header().unwrap_or_default();
    let body = serde_json::to_string(&user_data).map_err(|e| e.to_string())?;
    
    let resp = Request::new(&format!("{}/api/admin/users/{}", get_base_url(), user_id))
        .method(Method::PUT)
        .header("Authorization", &auth_header)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        let response: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        Ok(response)
    } else {
        Err(home_hub_shared::check_auth_error(format!("Failed to update user: {}", resp.status())))
    }
}

// Delete user (admin only)
pub async fn delete_admin_user(user_id: i32) -> Result<(), String> {
    let auth_header = get_auth_header().unwrap_or_default();
    
    let resp = Request::new(&format!("{}/api/admin/users/{}", get_base_url(), user_id))
        .method(Method::DELETE)
        .header("Authorization", &auth_header)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        Ok(())
    } else {
        Err(home_hub_shared::check_auth_error(format!("Failed to delete user: {}", resp.status())))
    }
}

// Get all recipes (admin only)
pub async fn get_admin_recipes() -> Result<serde_json::Value, String> {
    let auth_header = get_auth_header().unwrap_or_default();
    let base = get_base_url();
    let resp = Request::new(&format!("{}/api/admin/recipes", base))
        .method(Method::GET)
        .header("Authorization", &auth_header)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        let response: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        Ok(response)
    } else {
        Err(home_hub_shared::check_auth_error(format!("Failed to get recipes: {}", resp.status())))
    }
}

// Delete any recipe (admin only)
pub async fn delete_admin_recipe(recipe_id: i32) -> Result<(), String> {
    let auth_header = get_auth_header().unwrap_or_default();
    
    let base = get_base_url();
    let resp = Request::new(&format!("{}/api/admin/recipes/{}", base, recipe_id))
        .method(Method::DELETE)
        .header("Authorization", &auth_header)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        Ok(())
    } else {
        Err(home_hub_shared::check_auth_error(format!("Failed to delete recipe: {}", resp.status())))
    }
}

// Get all categories (admin only)
pub async fn get_admin_categories() -> Result<serde_json::Value, String> {
    let auth_header = get_auth_header().unwrap_or_default();
    let resp = Request::new(&format!("{}/api/admin/categories", get_base_url()))
        .method(Method::GET)
        .header("Authorization", &auth_header)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        let response: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        Ok(response)
    } else {
        Err(home_hub_shared::check_auth_error(format!("Failed to get categories: {}", resp.status())))
    }
}

// Create category (admin only)
pub async fn create_admin_category(category_data: serde_json::Value) -> Result<serde_json::Value, String> {
    let auth_header = get_auth_header().unwrap_or_default();
    let body = serde_json::to_string(&category_data).map_err(|e| e.to_string())?;
    
    let resp = Request::new(&format!("{}/api/admin/categories", get_base_url()))
        .method(Method::POST)
        .header("Authorization", &auth_header)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        let response: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        Ok(response)
    } else {
        Err(home_hub_shared::check_auth_error(format!("Failed to create category: {}", resp.status())))
    }
}

// Delete category (admin only)
pub async fn delete_admin_category(category_id: i32) -> Result<(), String> {
    let auth_header = get_auth_header().unwrap_or_default();
    
    let base = get_base_url();
    let resp = Request::new(&format!("{}/api/admin/categories/{}", base, category_id))
        .method(Method::DELETE)
        .header("Authorization", &auth_header)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        Ok(())
    } else {
        Err(home_hub_shared::check_auth_error(format!("Failed to delete category: {}", resp.status())))
    }
}

// Check if any admin users exist
pub async fn check_admin_exists() -> Result<bool, String> {
    let base = get_base_url();
    let resp = Request::new(&format!("{}/api/admin/check", base))
        .method(Method::GET)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        let response: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        Ok(response["admin_exists"].as_bool().unwrap_or(false))
    } else {
        Err(home_hub_shared::check_auth_error(format!("Failed to check admin status: {}", resp.status())))
    }
}

// Create initial admin user
pub async fn create_initial_admin(name: String, email: String, password: String) -> Result<User, String> {
    let body = json!({
        "name": name,
        "email": email,
        "password": password,
        "is_admin": true
    });

    let base = get_base_url();
    let resp = Request::new(&format!("{}/api/admin/setup", base))
        .method(Method::POST)
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        let user: User = resp
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        Ok(user)
    } else {
        Err(home_hub_shared::check_auth_error(format!("Failed to create admin: {}", resp.status())))
    }
}

// Delete own user account
pub async fn delete_my_account(user_id: i32) -> Result<String, String> {
    let base = get_base_url();
    let resp = Request::delete(&format!("{}/api/users/{}", base, user_id))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        resp.text().await.map_err(|_| "Read error".to_string())
    } else {
        Err(home_hub_shared::check_auth_error(format!("Failed to delete account: {}", resp.status())))
    }
}

// Save user preferences (theme, primary_color)
pub async fn save_prefs(prefs_json: &str) -> Result<String, String> {
    let base = get_base_url();
    let mut req = Request::put(&format!("{}/api/prefs", base))
        .header("Content-Type", "application/json")
        .body(json!({"prefs": prefs_json}).to_string());
    if let Some(auth) = get_auth_header() {
        req = req.header("Authorization", &auth);
    }
    let resp = req.send().await.map_err(|e| e.to_string())?;

    if resp.ok() {
        resp.text().await.map_err(|_| "Read error".to_string())
    } else {
        Err(home_hub_shared::check_auth_error(format!("Failed to save prefs: {}", resp.status())))
    }
}

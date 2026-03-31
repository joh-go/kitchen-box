use gloo::net::http::{Request, Method};
use serde_json::json;
use shared_types::{Category, Recipe, RecipeImage, User};
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

pub async fn clear_categories(recipe_id: i32) -> Result<(), String> {
    let auth_header = get_auth_header().unwrap_or_else(|| "".to_string());
    let url = format!("{}/api/recipes/{}/categories", BASE, recipe_id);
    
    let resp = Request::delete(&url)
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

// --- Images API ---
pub async fn get_recipe_images(recipe_id: i32) -> Result<Vec<RecipeImage>, String> {
    let auth_header = get_auth_header().unwrap_or_else(|| "".to_string());
    let resp = Request::get(&format!("{}/api/recipes/{}/images", BASE, recipe_id))
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
    
    let request = Request::new(&format!("{}/api/recipes/{}/images", BASE, recipe_id))
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
        Err(format!("Image upload failed: {}", resp.status()))
    }
}

pub async fn set_primary_image(recipe_id: i32, image_id: i32) -> Result<(), String> {
    let auth_header = get_auth_header().unwrap_or_else(|| "".to_string());
    let url = format!("{}/api/recipes/{}/images/{}/primary", BASE, recipe_id, image_id);
    
    let resp = Request::put(&url)
        .header("Authorization", &auth_header)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        Ok(())
    } else {
        Err(format!("Failed to set primary image: {}", resp.status()))
    }
}

pub async fn delete_recipe_image(recipe_id: i32, image_id: i32) -> Result<(), String> {
    let auth_header = get_auth_header().unwrap_or_else(|| "".to_string());
    let url = format!("{}/api/recipes/{}/images/{}", BASE, recipe_id, image_id);
    
    let resp = Request::delete(&url)
        .header("Authorization", &auth_header)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if resp.ok() {
        Ok(())
    } else {
        Err(format!("Failed to delete image: {}", resp.status()))
    }
}

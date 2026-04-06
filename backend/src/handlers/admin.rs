use rocket::serde::json::Json;
use rocket::{State, response::status::Custom, http::Status};
use tokio_postgres::Client;
use crate::auth::AuthenticatedUser;
use bcrypt::{hash, DEFAULT_COST};

#[derive(Debug, serde::Deserialize)]
pub struct AdminUserUpdateRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub is_admin: Option<bool>,
}

#[derive(Debug, serde::Deserialize)]
pub struct AdminUserCreateRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub is_admin: bool,
}

// Admin middleware to check if user is admin
pub fn require_admin(auth_user: &AuthenticatedUser) -> Result<(), Custom<String>> {
    if auth_user.is_admin {
        Ok(())
    } else {
        Err(Custom(Status::Forbidden, "Admin access required".to_string()))
    }
}

// Get all users (admin only)
#[get("/api/admin/users")]
pub async fn get_all_users(
    conn: &State<Client>,
    auth_user: AuthenticatedUser,
) -> Result<Json<serde_json::Value>, Custom<String>> {
    require_admin(&auth_user)?;

    let rows = conn
        .query(
            "SELECT id, name, email, is_admin, created_at FROM users ORDER BY created_at DESC",
            &[]
        )
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    let users: Vec<serde_json::Value> = rows.iter().map(|row| {
        serde_json::json!({
            "id": row.get::<_, i32>(0),
            "name": row.get::<_, String>(1),
            "email": row.get::<_, String>(2),
            "is_admin": row.get::<_, bool>(3),
            "created_at": row.get::<_, chrono::DateTime<chrono::Utc>>(4)
        })
    }).collect();

    Ok(Json(serde_json::json!({ "users": users })))
}

// Create user (admin only)
#[post("/api/admin/users", data = "<user_data>")]
pub async fn create_user(
    conn: &State<Client>,
    auth_user: AuthenticatedUser,
    user_data: Json<AdminUserCreateRequest>,
) -> Result<Json<serde_json::Value>, Custom<String>> {
    require_admin(&auth_user)?;

    // Check if email already exists
    let existing = conn
        .query_one("SELECT id FROM users WHERE email = $1", &[&user_data.email])
        .await;

    match existing {
        Ok(_) => return Err(Custom(Status::Conflict, "Email already exists".to_string())),
        Err(_) => {} // Email doesn't exist, continue
    }

    // Hash password
    let hashed_password = hash(&user_data.password, DEFAULT_COST)
        .map_err(|e| Custom(Status::InternalServerError, format!("Password hash error: {}", e)))?;

    // Create user
    let row = conn
        .query_one(
            "INSERT INTO users (name, email, password, is_admin) VALUES ($1, $2, $3, $4) RETURNING id, name, email, is_admin, created_at",
            &[&user_data.name, &user_data.email, &hashed_password, &user_data.is_admin]
        )
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    Ok(Json(serde_json::json!({
        "id": row.get::<_, i32>(0),
        "name": row.get::<_, String>(1),
        "email": row.get::<_, String>(2),
        "is_admin": row.get::<_, bool>(3),
        "created_at": row.get::<_, chrono::DateTime<chrono::Utc>>(4)
    })))
}

// Update user (admin only)
#[put("/api/admin/users/<user_id>", data = "<user_data>")]
pub async fn update_user(
    conn: &State<Client>,
    auth_user: AuthenticatedUser,
    user_id: i32,
    user_data: Json<AdminUserUpdateRequest>,
) -> Result<Json<serde_json::Value>, Custom<String>> {
    require_admin(&auth_user)?;

    // Check if user exists
    let existing = conn
        .query_one("SELECT id FROM users WHERE id = $1", &[&user_id])
        .await;

    match existing {
        Ok(_) => {} // User exists, continue
        Err(_) => return Err(Custom(Status::NotFound, "User not found".to_string())),
    }

    // Check email uniqueness if email is being updated
    if let Some(ref email) = user_data.email {
        let email_check = conn
            .query_one("SELECT id FROM users WHERE email = $1 AND id != $2", &[&email, &user_id])
            .await;

        match email_check {
            Ok(_) => return Err(Custom(Status::Conflict, "Email already exists".to_string())),
            Err(_) => {} // Email is available, continue
        }
    }

    // Handle different update scenarios
    let row = if user_data.name.is_some() && user_data.email.is_some() && user_data.password.is_some() && user_data.is_admin.is_some() {
        // Update all fields
        let hashed_password = hash(user_data.password.as_ref().unwrap(), DEFAULT_COST)
            .map_err(|e| Custom(Status::InternalServerError, format!("Password hash error: {}", e)))?;
        
        conn.query_one(
            "UPDATE users SET name = $1, email = $2, password = $3, is_admin = $4 WHERE id = $5 RETURNING id, name, email, is_admin, created_at",
            &[&user_data.name.as_ref().unwrap(), &user_data.email.as_ref().unwrap(), &hashed_password, &user_data.is_admin.as_ref().unwrap(), &user_id]
        ).await.map_err(|e| Custom(Status::InternalServerError, e.to_string()))?
    } else if user_data.name.is_some() && user_data.email.is_some() && user_data.is_admin.is_some() {
        // Update name, email, and admin status
        conn.query_one(
            "UPDATE users SET name = $1, email = $2, is_admin = $3 WHERE id = $4 RETURNING id, name, email, is_admin, created_at",
            &[&user_data.name.as_ref().unwrap(), &user_data.email.as_ref().unwrap(), &user_data.is_admin.as_ref().unwrap(), &user_id]
        ).await.map_err(|e| Custom(Status::InternalServerError, e.to_string()))?
    } else if user_data.name.is_some() && user_data.email.is_some() {
        // Update name and email
        conn.query_one(
            "UPDATE users SET name = $1, email = $2 WHERE id = $3 RETURNING id, name, email, is_admin, created_at",
            &[&user_data.name.as_ref().unwrap(), &user_data.email.as_ref().unwrap(), &user_id]
        ).await.map_err(|e| Custom(Status::InternalServerError, e.to_string()))?
    } else if user_data.name.is_some() && user_data.password.is_some() {
        // Update name and password
        let hashed_password = hash(user_data.password.as_ref().unwrap(), DEFAULT_COST)
            .map_err(|e| Custom(Status::InternalServerError, format!("Password hash error: {}", e)))?;
        
        conn.query_one(
            "UPDATE users SET name = $1, password = $2 WHERE id = $3 RETURNING id, name, email, is_admin, created_at",
            &[&user_data.name.as_ref().unwrap(), &hashed_password, &user_id]
        ).await.map_err(|e| Custom(Status::InternalServerError, e.to_string()))?
    } else if user_data.email.is_some() && user_data.password.is_some() {
        // Update email and password
        let hashed_password = hash(user_data.password.as_ref().unwrap(), DEFAULT_COST)
            .map_err(|e| Custom(Status::InternalServerError, format!("Password hash error: {}", e)))?;
        
        conn.query_one(
            "UPDATE users SET email = $1, password = $2 WHERE id = $3 RETURNING id, name, email, is_admin, created_at",
            &[&user_data.email.as_ref().unwrap(), &hashed_password, &user_id]
        ).await.map_err(|e| Custom(Status::InternalServerError, e.to_string()))?
    } else if user_data.name.is_some() {
        // Update only name
        conn.query_one(
            "UPDATE users SET name = $1 WHERE id = $2 RETURNING id, name, email, is_admin, created_at",
            &[&user_data.name.as_ref().unwrap(), &user_id]
        ).await.map_err(|e| Custom(Status::InternalServerError, e.to_string()))?
    } else if user_data.email.is_some() {
        // Update only email
        conn.query_one(
            "UPDATE users SET email = $1 WHERE id = $2 RETURNING id, name, email, is_admin, created_at",
            &[&user_data.email.as_ref().unwrap(), &user_id]
        ).await.map_err(|e| Custom(Status::InternalServerError, e.to_string()))?
    } else if user_data.password.is_some() {
        // Update only password
        let hashed_password = hash(user_data.password.as_ref().unwrap(), DEFAULT_COST)
            .map_err(|e| Custom(Status::InternalServerError, format!("Password hash error: {}", e)))?;
        
        conn.query_one(
            "UPDATE users SET password = $1 WHERE id = $2 RETURNING id, name, email, is_admin, created_at",
            &[&hashed_password, &user_id]
        ).await.map_err(|e| Custom(Status::InternalServerError, e.to_string()))?
    } else if user_data.is_admin.is_some() {
        // Update only admin status
        conn.query_one(
            "UPDATE users SET is_admin = $1 WHERE id = $2 RETURNING id, name, email, is_admin, created_at",
            &[&user_data.is_admin.as_ref().unwrap(), &user_id]
        ).await.map_err(|e| Custom(Status::InternalServerError, e.to_string()))?
    } else {
        return Err(Custom(Status::BadRequest, "No fields to update".to_string()));
    };

    Ok(Json(serde_json::json!({
        "id": row.get::<_, i32>(0),
        "name": row.get::<_, String>(1),
        "email": row.get::<_, String>(2),
        "is_admin": row.get::<_, bool>(3),
        "created_at": row.get::<_, chrono::DateTime<chrono::Utc>>(4)
    })))
}

// Delete user (admin only)
#[delete("/api/admin/users/<user_id>")]
pub async fn delete_user(
    conn: &State<Client>,
    auth_user: AuthenticatedUser,
    user_id: i32,
) -> Result<Status, Custom<String>> {
    require_admin(&auth_user)?;

    // Prevent admin from deleting themselves
    if user_id == auth_user.user_id {
        return Err(Custom(Status::BadRequest, "Cannot delete your own account".to_string()));
    }

    // Check if user exists
    let existing = conn
        .query_one("SELECT id FROM users WHERE id = $1", &[&user_id])
        .await;

    match existing {
        Ok(_) => {} // User exists, continue
        Err(_) => return Err(Custom(Status::NotFound, "User not found".to_string())),
    }

    // Delete user (cascade will handle recipes and other data)
    conn.execute("DELETE FROM users WHERE id = $1", &[&user_id])
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    Ok(Status::Ok)
}

// Get all recipes (admin only)
#[get("/api/admin/recipes")]
pub async fn get_all_recipes(
    conn: &State<Client>,
    auth_user: AuthenticatedUser,
) -> Result<Json<serde_json::Value>, Custom<String>> {
    require_admin(&auth_user)?;

    let rows = conn
        .query(
            "SELECT r.id, r.title, r.short_description, r.author_id, u.name as author_name, u.email as author_email, 
                    r.is_public, r.created_at, r.updated_at
             FROM recipes r 
             LEFT JOIN users u ON r.author_id = u.id 
             ORDER BY r.created_at DESC",
            &[]
        )
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    let recipes: Vec<serde_json::Value> = rows.iter().map(|row| {
        serde_json::json!({
            "id": row.get::<_, Option<i32>>(0),
            "title": row.get::<_, String>(1),
            "short_description": row.get::<_, Option<String>>(2),
            "author_id": row.get::<_, Option<i32>>(3),
            "author_name": row.get::<_, Option<String>>(4),
            "author_email": row.get::<_, Option<String>>(5),
            "is_public": row.get::<_, bool>(6),
            "created_at": row.get::<_, chrono::DateTime<chrono::Utc>>(7),
            "updated_at": row.get::<_, chrono::DateTime<chrono::Utc>>(8)
        })
    }).collect();

    Ok(Json(serde_json::json!({ "recipes": recipes })))
}

// Delete any recipe (admin only)
#[delete("/api/admin/recipes/<recipe_id>")]
pub async fn delete_any_recipe(
    conn: &State<Client>,
    auth_user: AuthenticatedUser,
    recipe_id: i32,
) -> Result<Status, Custom<String>> {
    require_admin(&auth_user)?;

    // Check if recipe exists
    let existing = conn
        .query_one("SELECT id FROM recipes WHERE id = $1", &[&recipe_id])
        .await;

    match existing {
        Ok(_) => {} // Recipe exists, continue
        Err(_) => return Err(Custom(Status::NotFound, "Recipe not found".to_string())),
    }

    // Delete recipe (cascade will handle images, categories, etc.)
    conn.execute("DELETE FROM recipes WHERE id = $1", &[&recipe_id])
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    Ok(Status::Ok)
}

// Get all categories (admin only)
#[get("/api/admin/categories")]
pub async fn get_all_categories(
    conn: &State<Client>,
    auth_user: AuthenticatedUser,
) -> Result<Json<serde_json::Value>, Custom<String>> {
    require_admin(&auth_user)?;

    let rows = conn
        .query("SELECT id, name, created_at FROM categories ORDER BY name", &[])
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    let categories: Vec<serde_json::Value> = rows.iter().map(|row| {
        serde_json::json!({
            "id": row.get::<_, i32>(0),
            "name": row.get::<_, String>(1),
            "created_at": row.get::<_, chrono::DateTime<chrono::Utc>>(2)
        })
    }).collect();

    Ok(Json(serde_json::json!({ "categories": categories })))
}

// Delete any category (admin only)
#[delete("/api/admin/categories/<category_id>")]
pub async fn delete_category(
    conn: &State<Client>,
    auth_user: AuthenticatedUser,
    category_id: i32,
) -> Result<Status, Custom<String>> {
    require_admin(&auth_user)?;

    // Check if category exists
    let existing = conn
        .query_one("SELECT id FROM categories WHERE id = $1", &[&category_id])
        .await;

    match existing {
        Ok(_) => {} // Category exists, continue
        Err(_) => return Err(Custom(Status::NotFound, "Category not found".to_string())),
    }

    // Delete category (cascade will handle recipe_category associations)
    conn.execute("DELETE FROM categories WHERE id = $1", &[&category_id])
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    Ok(Status::Ok)
}

use rocket::serde::json::Json;
use rocket::{State, response::status::Custom, http::Status};
use tokio_postgres::Client;
use crate::auth::{authenticate_user, generate_token, LoginRequest, LoginResponse, UserInfo, AuthenticatedUser};
use bcrypt::{hash, DEFAULT_COST};

#[derive(Debug, serde::Deserialize)]
pub struct UpdateProfileRequest {
    pub name: String,
    pub email: String,
    pub current_password: Option<String>,
    pub new_password: Option<String>,
}

#[post("/api/auth/login", data = "<login>")]
pub async fn login(
    conn: &State<Client>,
    login: Json<LoginRequest>,
) -> Result<Json<LoginResponse>, Custom<String>> {
    match authenticate_user(conn, &login.email, &login.password).await {
        Ok(Some(user)) => {
            // Generate JWT token
            let token = generate_token(user.id).map_err(|e| Custom(Status::InternalServerError, e))?;
            
            // Return token and user info
            Ok(Json(LoginResponse {
                token,
                user: UserInfo {
                    id: user.id,
                    name: user.name,
                    email: user.email,
                },
            }))
        }
        Ok(None) => Err(Custom(Status::Unauthorized, "Invalid email or password".to_string())),
        Err(e) => Err(Custom(Status::InternalServerError, e)),
    }
}

#[post("/api/auth/logout")]
pub async fn logout() -> Result<Status, Custom<String>> {
    // In a real implementation, you might want to:
    // 1. Invalidate the token (blacklist it)
    // 2. Remove the token from client-side storage
    // For now, we'll just return success
    Ok(Status::Ok)
}

#[get("/api/auth/me")]
pub async fn get_current_user(
    conn: &State<Client>,
    auth_user: AuthenticatedUser,
) -> Result<Json<UserInfo>, Custom<String>> {
    // Fetch full user info from database
    let rows = conn
        .query("SELECT id, name, email FROM users WHERE id = $1", &[&auth_user.user_id])
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;
    
    if let Some(row) = rows.iter().next() {
        Ok(Json(UserInfo {
            id: row.get(0),
            name: row.get(1),
            email: row.get(2),
        }))
    } else {
        Err(Custom(Status::NotFound, "User not found".to_string()))
    }
}

#[put("/api/auth/me", data = "<update>")]
pub async fn update_current_user(
    conn: &State<Client>,
    auth_user: AuthenticatedUser,
    update: Json<UpdateProfileRequest>,
) -> Result<Json<UserInfo>, Custom<String>> {
    // Fetch user's current data
    let user_rows = conn
        .query("SELECT name, email FROM users WHERE id = $1", &[&auth_user.user_id])
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;
    
    let (current_name, current_email): (String, String) = user_rows
        .iter()
        .next()
        .map(|row| (row.get(0), row.get(1)))
        .ok_or_else(|| Custom(Status::NotFound, "User not found".to_string()))?;
    
    // Use provided values or fall back to current values
    let new_name = if update.name.trim().is_empty() {
        current_name.clone()
    } else {
        update.name.clone()
    };
    
    let new_email = if update.email.trim().is_empty() {
        current_email.clone()
    } else {
        update.email.clone()
    };
    
    // If changing password, verify current password first
    if let Some(ref current) = update.current_password {
        if !current.trim().is_empty() {
            if let Some(ref new) = update.new_password {
                if !new.trim().is_empty() {
                    // Authenticate with current password using the stored email
                    match authenticate_user(conn, &current_email, current).await {
                        Ok(Some(_)) => {
                            // Current password is correct, proceed with update
                        }
                        Ok(None) => {
                            return Err(Custom(Status::Forbidden, "Current password is incorrect".to_string()));
                        }
                        Err(e) => {
                            return Err(Custom(Status::InternalServerError, e));
                        }
                    }
                    
                    // Hash new password
                    let hashed = hash(new, DEFAULT_COST)
                        .map_err(|e| Custom(Status::InternalServerError, format!("Password hash error: {}", e)))?;
                    
                    // Update user with new password
                    conn.execute(
                        "UPDATE users SET name = $1, email = $2, password = $3 WHERE id = $4",
                        &[&new_name, &new_email, &hashed, &auth_user.user_id]
                    ).await
                    .map_err(|e| Custom(Status::InternalServerError, format!("Update failed: {}", e)))?;
                } else {
                    // New password is empty, just update name and email
                    conn.execute(
                        "UPDATE users SET name = $1, email = $2 WHERE id = $3",
                        &[&new_name, &new_email, &auth_user.user_id]
                    ).await
                    .map_err(|e| Custom(Status::InternalServerError, format!("Update failed: {}", e)))?;
                }
            } else {
                // No new password provided, just update name and email
                conn.execute(
                    "UPDATE users SET name = $1, email = $2 WHERE id = $3",
                    &[&new_name, &new_email, &auth_user.user_id]
                ).await
                .map_err(|e| Custom(Status::InternalServerError, format!("Update failed: {}", e)))?;
            }
        } else {
            // Current password is empty, just update name and email without password change
            conn.execute(
                "UPDATE users SET name = $1, email = $2 WHERE id = $3",
                &[&new_name, &new_email, &auth_user.user_id]
            ).await
            .map_err(|e| Custom(Status::InternalServerError, format!("Update failed: {}", e)))?;
        }
    } else {
        // No password change, just update name and email
        conn.execute(
            "UPDATE users SET name = $1, email = $2 WHERE id = $3",
            &[&new_name, &new_email, &auth_user.user_id]
        ).await
        .map_err(|e| Custom(Status::InternalServerError, format!("Update failed: {}", e)))?;
    }
    
    // Return updated user info
    let rows = conn
        .query("SELECT id, name, email FROM users WHERE id = $1", &[&auth_user.user_id])
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;
    
    if let Some(row) = rows.iter().next() {
        Ok(Json(UserInfo {
            id: row.get(0),
            name: row.get(1),
            email: row.get(2),
        }))
    } else {
        Err(Custom(Status::NotFound, "User not found".to_string()))
    }
}

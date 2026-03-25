use rocket::serde::json::Json;
use rocket::{State, response::status::Custom, http::Status};
use tokio_postgres::Client;
use crate::auth::{authenticate_user, generate_token, LoginRequest, LoginResponse};

#[post("/api/auth/login", data = "<login>")]
pub async fn login(
    conn: &State<Client>,
    login: Json<LoginRequest>,
) -> Result<Json<LoginResponse>, Custom<String>> {
    match authenticate_user(conn, &login.email, &login.password).await {
        Ok(Some(user)) => {
            match generate_token(user.id) {
                Ok(token) => Ok(Json(LoginResponse {
                    token,
                    user,
                })),
                Err(e) => Err(Custom(Status::InternalServerError, format!("Failed to generate token: {}", e))),
            }
        }
        Ok(None) => Err(Custom(Status::Unauthorized, "Invalid email or password".to_string())),
        Err(e) => Err(Custom(Status::InternalServerError, format!("Database error: {}", e))),
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
    auth_user: crate::auth::AuthenticatedUser,
) -> Result<Json<crate::auth::UserInfo>, Custom<String>> {
    // In a real implementation, you'd fetch full user info from database
    // For now, return basic info
    Ok(Json(crate::auth::UserInfo {
        id: auth_user.user_id,
        name: "User".to_string(), // Would fetch from DB
        email: auth_user.user_email.clone(),
    }))
}

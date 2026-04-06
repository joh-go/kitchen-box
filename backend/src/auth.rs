use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rocket::{State, response::status::Custom, http::Status, request::{FromRequest, Outcome}};
use tokio_postgres::Client;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use bcrypt::verify;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (user ID)
    pub exp: usize,  // Expiration time
    pub iat: usize,  // Issued at
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub is_admin: bool,
}

pub struct AuthenticatedUser {
    pub user_id: i32,
    pub user_email: String,
    pub is_admin: bool,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = Custom<String>;

    async fn from_request(request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        // Get Authorization header
        let auth_header = request.headers().get_one("Authorization");
        
        if let Some(auth_header) = auth_header {
            // Extract token from "Bearer <token>"
            if let Some(token) = auth_header.strip_prefix("Bearer ") {
                // Decode and validate token
                let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string());
                let key = DecodingKey::from_secret(secret.as_ref());
                
                match decode::<Claims>(token, &key, &Validation::default()) {
                    Ok(token_data) => {
                        let claims = token_data.claims;
                        let user_id: i32 = claims.sub.parse().unwrap_or(0);
                        
                        // Get database connection from request state
                        let conn = request.guard::<&State<Client>>().await.unwrap();
                        
                        // Fetch user info including admin status from database
                        let rows = conn
                            .query("SELECT id, name, email, is_admin FROM users WHERE id = $1", &[&user_id])
                            .await;
                        
                        match rows {
                            Ok(rows) => {
                                if let Some(row) = rows.iter().next() {
                                    return Outcome::Success(AuthenticatedUser {
                                        user_id,
                                        user_email: row.get(2),
                                        is_admin: row.get(3),
                                    });
                                } else {
                                    return Outcome::Error((Status::Unauthorized, Custom(Status::Unauthorized, "User not found".to_string())));
                                }
                            }
                            Err(e) => {
                                return Outcome::Error((Status::InternalServerError, Custom(Status::InternalServerError, format!("Database error: {}", e))));
                            }
                        }
                    }
                    Err(_) => {
                        return Outcome::Error((Status::Unauthorized, Custom(Status::Unauthorized, "Invalid token".to_string())));
                    }
                }
            }
        }
        
        Outcome::Error((Status::Unauthorized, Custom(Status::Unauthorized, "No authorization header".to_string())))
    }
}

pub fn generate_token(user_id: i32) -> Result<String, String> {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string());
    let key = EncodingKey::from_secret(secret.as_ref());
    
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_secs() as usize;
    
    let exp = now + 24 * 60 * 60; // 24 hours from now
    
    let claims = Claims {
        sub: user_id.to_string(),
        exp,
        iat: now,
    };
    
    encode(&Header::default(), &claims, &key)
        .map_err(|e| e.to_string())
}

pub async fn authenticate_user(
    conn: &Client,
    email: &str,
    password: &str,
) -> Result<Option<UserInfo>, String> {
    // Get user by email
    let rows = conn
        .query("SELECT id, name, email, password, is_admin FROM users WHERE email = $1", &[&email])
        .await
        .map_err(|e| e.to_string())?;
    
    if let Some(row) = rows.iter().next() {
        let stored_hash: String = row.get(3);
        
        // Verify password against stored hash
        match verify(password, &stored_hash) {
            Ok(true) => {
                Ok(Some(UserInfo {
                    id: row.get(0),
                    name: row.get(1),
                    email: row.get(2),
                    is_admin: row.get(4),
                }))
            }
            Ok(false) => Ok(None), // Password doesn't match
            Err(e) => Err(format!("Password verification error: {}", e)),
        }
    } else {
        Ok(None) // User not found
    }
}

pub async fn get_user_by_id(conn: &Client, user_id: i32) -> Result<Option<UserInfo>, String> {
    let rows = conn
        .query("SELECT id, name, email, is_admin FROM users WHERE id = $1", &[&user_id])
        .await
        .map_err(|e| e.to_string())?;
    
    if let Some(row) = rows.iter().next() {
        Ok(Some(UserInfo {
            id: row.get(0),
            name: row.get(1),
            email: row.get(2),
            is_admin: row.get(3),
        }))
    } else {
        Ok(None)
    }
}

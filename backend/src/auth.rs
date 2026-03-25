use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::State;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio_postgres::Client;
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
}

pub struct AuthenticatedUser {
    pub user_id: i32,
    pub user_email: String,
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
                        return Outcome::Success(AuthenticatedUser {
                            user_id: claims.sub.parse().unwrap_or(0),
                            user_email: "".to_string(), // We'll need to fetch this from DB if needed
                        });
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
        .query("SELECT id, name, email, password FROM users WHERE email = $1", &[&email])
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
        .query("SELECT id, name, email FROM users WHERE id = $1", &[&user_id])
        .await
        .map_err(|e| e.to_string())?;
    
    if let Some(row) = rows.iter().next() {
        Ok(Some(UserInfo {
            id: row.get(0),
            name: row.get(1),
            email: row.get(2),
        }))
    } else {
        Ok(None)
    }
}

use crate::db::DbConn;
use crate::schema::users::dsl::*;
use diesel::prelude::*;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use serde::{Deserialize, Serialize};

static JWT_SECRET: Lazy<String> = Lazy::new(|| {
    std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "your-secret-key-change-in-production".to_string())
});

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub username: Option<String>,
    pub is_admin: Option<bool>,
    pub exp: usize,
    pub iat: usize,
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
    #[allow(dead_code)]
    pub user_email: String,
    pub is_admin: bool,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = request.headers().get_one("Authorization");

        match auth_header {
            Some(header) if header.starts_with("Bearer ") => {
                let token = &header[7..];
                let secret = JWT_SECRET.as_bytes();
                let key = DecodingKey::from_secret(secret);

                match decode::<Claims>(token, &key, &Validation::default()) {
                    Ok(token_data) => {
                        let claims = token_data.claims;
                        let user_id_val: i32 = claims.sub.parse().unwrap_or(0);

                        let mut db = match request.guard::<DbConn>().await {
                            Outcome::Success(db) => db,
                            _ => return Outcome::Forward(Status::InternalServerError),
                        };

                        let user = users
                            .select((id, name, email, is_admin))
                            .filter(id.eq(&user_id_val))
                            .first::<(i32, String, String, bool)>(&mut *db)
                            .ok();

                        match user {
                            Some((uid, _uname, uemail, uadmin)) => {
                                Outcome::Success(AuthenticatedUser {
                                    user_id: uid,
                                    user_email: uemail,
                                    is_admin: uadmin,
                                })
                            }
                            None => {
                                let claims_username = claims.username.unwrap_or_default();
                                let claims_admin = claims.is_admin.unwrap_or(false);
                                if claims_username.is_empty() {
                                    return Outcome::Forward(Status::Unauthorized);
                                }
                                let user_email = format!("{}@home-hub.local", claims_username);
                                let placeholder_pw = uuid::Uuid::new_v4().to_string();
                                let hashed = match bcrypt::hash(&placeholder_pw, bcrypt::DEFAULT_COST) {
                                    Ok(h) => h,
                                    Err(_) => return Outcome::Forward(Status::InternalServerError),
                                };
                                let result = diesel::sql_query(
                                    "INSERT INTO users (id, name, email, password, is_admin) VALUES ($1, $2, $3, $4, $5)",
                                )
                                .bind::<diesel::sql_types::Integer, _>(user_id_val)
                                .bind::<diesel::sql_types::Text, _>(&claims_username)
                                .bind::<diesel::sql_types::Text, _>(&user_email)
                                .bind::<diesel::sql_types::Text, _>(&hashed)
                                .bind::<diesel::sql_types::Bool, _>(claims_admin)
                                .execute(&mut *db);
                                match result {
                                    Ok(_) => {
                                        Outcome::Success(AuthenticatedUser {
                                            user_id: user_id_val,
                                            user_email,
                                            is_admin: claims_admin,
                                        })
                                    }
                                    Err(_) => Outcome::Forward(Status::InternalServerError),
                                }
                            }
                        }
                    }
                    Err(_) => Outcome::Forward(Status::Unauthorized),
                }
            }
            _ => Outcome::Forward(Status::Unauthorized),
        }
    }
}

pub fn generate_token(user_id: i32) -> Result<String, String> {
    let secret = JWT_SECRET.as_bytes();
    let key = EncodingKey::from_secret(secret);

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_secs() as usize;

    let exp = now + 24 * 60 * 60;

    let claims = Claims {
        sub: user_id.to_string(),
        username: None,
        is_admin: None,
        exp,
        iat: now,
    };

    encode(&Header::default(), &claims, &key).map_err(|e| e.to_string())
}

pub fn authenticate_user(
    conn: &mut diesel::PgConnection,
    login_input: &str,
    password_input: &str,
) -> Result<Option<UserInfo>, String> {
    // Try email first, then username
    let user = users
        .filter(email.eq(login_input).or(name.eq(login_input)))
        .select((id, name, email, password, is_admin))
        .first::<(i32, String, String, String, bool)>(conn)
        .ok();

    match user {
        Some((uid, uname, uemail, pwhash, uadmin)) => {
            match bcrypt::verify(password_input, &pwhash) {
                Ok(true) => Ok(Some(UserInfo {
                    id: uid,
                    name: uname,
                    email: uemail,
                    is_admin: uadmin,
                })),
                Ok(false) => Ok(None),
                Err(e) => Err(format!("Password verification error: {}", e)),
            }
        }
        None => Ok(None),
    }
}

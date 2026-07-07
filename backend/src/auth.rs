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
                            None => Outcome::Forward(Status::Unauthorized),
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
        exp,
        iat: now,
    };

    encode(&Header::default(), &claims, &key).map_err(|e| e.to_string())
}

pub fn authenticate_user(
    conn: &mut diesel::PgConnection,
    email_input: &str,
    password_input: &str,
) -> Result<Option<UserInfo>, String> {
    let user = users
        .filter(email.eq(email_input))
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

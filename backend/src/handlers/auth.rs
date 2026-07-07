use crate::auth::{authenticate_user, generate_token, AuthenticatedUser};
use crate::db::DbConn;
use crate::models::{LoginRequest, LoginResponse, UpdateProfileRequest, UserResponse};
use crate::schema::users::dsl::*;
use bcrypt::{hash, DEFAULT_COST};
use diesel::prelude::*;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post, put};

#[post("/auth/login", format = "json", data = "<login>")]
pub fn login(
    mut db: DbConn,
    login: Json<LoginRequest>,
) -> Result<Json<LoginResponse>, Custom<String>> {
    match authenticate_user(&mut *db, &login.email, &login.password) {
        Ok(Some(user)) => {
            let token =
                generate_token(user.id).map_err(|e| Custom(Status::InternalServerError, e))?;
            Ok(Json(LoginResponse {
                token,
                user: UserResponse {
                    id: user.id,
                    name: user.name,
                    email: user.email,
                    is_admin: user.is_admin,
                },
            }))
        }
        Ok(None) => Err(Custom(Status::Unauthorized, "Invalid email or password".to_string())),
        Err(e) => Err(Custom(Status::InternalServerError, e)),
    }
}

#[post("/auth/logout")]
pub fn logout() -> Result<Status, Custom<String>> {
    Ok(Status::Ok)
}

#[get("/auth/me")]
pub fn get_current_user(
    mut db: DbConn,
    auth_user: AuthenticatedUser,
) -> Result<Json<UserResponse>, Custom<String>> {
    let user = users
        .select((id, name, email, is_admin))
        .filter(id.eq(&auth_user.user_id))
        .first::<(i32, String, String, bool)>(&mut *db)
        .map_err(|e| Custom(Status::NotFound, format!("User not found: {}", e)))?;

    Ok(Json(UserResponse {
        id: user.0,
        name: user.1,
        email: user.2,
        is_admin: user.3,
    }))
}

#[put("/auth/me", format = "json", data = "<update>")]
pub fn update_current_user(
    mut db: DbConn,
    auth_user: AuthenticatedUser,
    update: Json<UpdateProfileRequest>,
) -> Result<Json<UserResponse>, Custom<String>> {
    let current = users
        .select((name, email))
        .filter(id.eq(&auth_user.user_id))
        .first::<(String, String)>(&mut *db)
        .map_err(|e| Custom(Status::NotFound, format!("User not found: {}", e)))?;

    let new_name = if update.name.trim().is_empty() {
        current.0.clone()
    } else {
        update.name.clone()
    };

    let new_email = if update.email.trim().is_empty() {
        current.1.clone()
    } else {
        update.email.clone()
    };

    if let Some(ref current_pw) = update.current_password {
        if !current_pw.trim().is_empty() {
            if let Some(ref new_pw) = update.new_password {
                if !new_pw.trim().is_empty() {
                    match authenticate_user(&mut *db, &current.1, current_pw) {
                        Ok(Some(_)) => {}
                        Ok(None) => {
                            return Err(Custom(
                                Status::Forbidden,
                                "Current password is incorrect".to_string(),
                            ));
                        }
                        Err(e) => {
                            return Err(Custom(Status::InternalServerError, e));
                        }
                    }

                    let hashed = hash(new_pw, DEFAULT_COST)
                        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

                    diesel::update(users.filter(id.eq(&auth_user.user_id)))
                        .set((name.eq(&new_name), email.eq(&new_email), password.eq(&hashed)))
                        .execute(&mut *db)
                        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;
                } else {
                    diesel::update(users.filter(id.eq(&auth_user.user_id)))
                        .set((name.eq(&new_name), email.eq(&new_email)))
                        .execute(&mut *db)
                        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;
                }
            } else {
                diesel::update(users.filter(id.eq(&auth_user.user_id)))
                    .set((name.eq(&new_name), email.eq(&new_email)))
                    .execute(&mut *db)
                    .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;
            }
        } else {
            diesel::update(users.filter(id.eq(&auth_user.user_id)))
                .set((name.eq(&new_name), email.eq(&new_email)))
                .execute(&mut *db)
                .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;
        }
    } else {
        diesel::update(users.filter(id.eq(&auth_user.user_id)))
            .set((name.eq(&new_name), email.eq(&new_email)))
            .execute(&mut *db)
            .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;
    }

    let updated = users
        .select((id, name, email, is_admin))
        .filter(id.eq(&auth_user.user_id))
        .first::<(i32, String, String, bool)>(&mut *db)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    Ok(Json(UserResponse {
        id: updated.0,
        name: updated.1,
        email: updated.2,
        is_admin: updated.3,
    }))
}

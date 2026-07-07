use crate::auth::AuthenticatedUser;
use crate::db::DbConn;
use bcrypt::{hash, DEFAULT_COST};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put};
use serde_json::json;

use crate::schema::{categories, images, recipes, users};

#[derive(serde::Deserialize)]
pub struct AdminUserUpdateRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub is_admin: Option<bool>,
}

#[derive(serde::Deserialize)]
pub struct AdminUserCreateRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub is_admin: bool,
}

fn require_admin(auth_user: &AuthenticatedUser) -> Result<(), Custom<String>> {
    if auth_user.is_admin {
        Ok(())
    } else {
        Err(Custom(Status::Forbidden, "Admin access required".to_string()))
    }
}

#[get("/admin/users")]
pub fn get_all_users(
    mut db: DbConn,
    auth_user: AuthenticatedUser,
) -> Result<Json<serde_json::Value>, Custom<String>> {
    require_admin(&auth_user)?;

    let rows = users::table
        .select((users::id, users::name, users::email, users::is_admin, users::created_at))
        .order(users::created_at.desc())
        .load::<(i32, String, String, bool, DateTime<Utc>)>(&mut *db)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    let result: Vec<serde_json::Value> = rows
        .into_iter()
        .map(|(uid, uname, uemail, uadmin, ucreated)| {
            json!({
                "id": uid,
                "name": uname,
                "email": uemail,
                "is_admin": uadmin,
                "created_at": ucreated
            })
        })
        .collect();

    Ok(Json(json!({ "users": result })))
}

#[get("/admin/check")]
pub fn check_admin_exists(mut db: DbConn) -> Result<Json<serde_json::Value>, Custom<String>> {
    let count = users::table
        .filter(users::is_admin.eq(true))
        .count()
        .get_result::<i64>(&mut *db)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    Ok(Json(json!({
        "admin_exists": count > 0,
        "admin_count": count
    })))
}

#[post("/admin/setup", format = "json", data = "<user_data>")]
pub fn setup_initial_admin(
    mut db: DbConn,
    user_data: Json<AdminUserCreateRequest>,
) -> Result<Json<serde_json::Value>, Custom<String>> {
    let count = users::table
        .filter(users::is_admin.eq(true))
        .count()
        .get_result::<i64>(&mut *db)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    if count > 0 {
        return Err(Custom(Status::Forbidden, "Admin user already exists".to_string()));
    }

    let existing = users::table
        .filter(users::email.eq(&user_data.email))
        .select(users::id)
        .first::<i32>(&mut *db)
        .ok();

    if existing.is_some() {
        return Err(Custom(Status::Conflict, "Email already exists".to_string()));
    }

    let hashed_password = hash(&user_data.password, DEFAULT_COST)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    diesel::insert_into(users::table)
        .values((
            users::name.eq(&user_data.name),
            users::email.eq(&user_data.email),
            users::password.eq(&hashed_password),
            users::is_admin.eq(&user_data.is_admin),
        ))
        .returning((users::id, users::name, users::email, users::is_admin, users::created_at))
        .get_result::<(i32, String, String, bool, DateTime<Utc>)>(&mut *db)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))
        .map(|(uid, uname, uemail, uadmin, ucreated)| {
            Json(json!({
                "id": uid,
                "name": uname,
                "email": uemail,
                "is_admin": uadmin,
                "created_at": ucreated
            }))
        })
}

#[post("/admin/users", format = "json", data = "<user_data>")]
pub fn create_user(
    mut db: DbConn,
    auth_user: AuthenticatedUser,
    user_data: Json<AdminUserCreateRequest>,
) -> Result<Json<serde_json::Value>, Custom<String>> {
    require_admin(&auth_user)?;

    let existing = users::table
        .filter(users::email.eq(&user_data.email))
        .select(users::id)
        .first::<i32>(&mut *db)
        .ok();

    if existing.is_some() {
        return Err(Custom(Status::Conflict, "Email already exists".to_string()));
    }

    let hashed_password = hash(&user_data.password, DEFAULT_COST)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    diesel::insert_into(users::table)
        .values((
            users::name.eq(&user_data.name),
            users::email.eq(&user_data.email),
            users::password.eq(&hashed_password),
            users::is_admin.eq(&user_data.is_admin),
        ))
        .returning((users::id, users::name, users::email, users::is_admin, users::created_at))
        .get_result::<(i32, String, String, bool, DateTime<Utc>)>(&mut *db)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))
        .map(|(uid, uname, uemail, uadmin, ucreated)| {
            Json(json!({
                "id": uid,
                "name": uname,
                "email": uemail,
                "is_admin": uadmin,
                "created_at": ucreated
            }))
        })
}

#[put("/admin/users/<uid>", format = "json", data = "<user_data>")]
pub fn update_user(
    mut db: DbConn,
    auth_user: AuthenticatedUser,
    uid: i32,
    user_data: Json<AdminUserUpdateRequest>,
) -> Result<Json<serde_json::Value>, Custom<String>> {
    require_admin(&auth_user)?;

    users::table
        .filter(users::id.eq(uid))
        .select(users::id)
        .first::<i32>(&mut *db)
        .map_err(|_| Custom(Status::NotFound, "User not found".to_string()))?;

    if let Some(ref email_val) = user_data.email {
        let email_taken = users::table
            .filter(users::email.eq(email_val))
            .filter(users::id.ne(uid))
            .select(users::id)
            .first::<i32>(&mut *db)
            .ok();
        if email_taken.is_some() {
            return Err(Custom(Status::Conflict, "Email already exists".to_string()));
        }
    }

    let result = if user_data.name.is_some()
        && user_data.email.is_some()
        && user_data.password.is_some()
        && user_data.is_admin.is_some()
    {
        let hashed = hash(user_data.password.as_ref().unwrap(), DEFAULT_COST)
            .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;
        diesel::update(users::table.filter(users::id.eq(uid)))
            .set((
                users::name.eq(user_data.name.as_ref().unwrap()),
                users::email.eq(user_data.email.as_ref().unwrap()),
                users::password.eq(&hashed),
                users::is_admin.eq(user_data.is_admin.as_ref().unwrap()),
            ))
            .returning((users::id, users::name, users::email, users::is_admin, users::created_at))
            .get_result::<(i32, String, String, bool, DateTime<Utc>)>(&mut *db)
    } else if user_data.name.is_some() && user_data.email.is_some() && user_data.is_admin.is_some() {
        diesel::update(users::table.filter(users::id.eq(uid)))
            .set((
                users::name.eq(user_data.name.as_ref().unwrap()),
                users::email.eq(user_data.email.as_ref().unwrap()),
                users::is_admin.eq(user_data.is_admin.as_ref().unwrap()),
            ))
            .returning((users::id, users::name, users::email, users::is_admin, users::created_at))
            .get_result::<(i32, String, String, bool, DateTime<Utc>)>(&mut *db)
    } else if user_data.name.is_some() && user_data.email.is_some() {
        diesel::update(users::table.filter(users::id.eq(uid)))
            .set((
                users::name.eq(user_data.name.as_ref().unwrap()),
                users::email.eq(user_data.email.as_ref().unwrap()),
            ))
            .returning((users::id, users::name, users::email, users::is_admin, users::created_at))
            .get_result::<(i32, String, String, bool, DateTime<Utc>)>(&mut *db)
    } else if user_data.name.is_some() && user_data.password.is_some() {
        let hashed = hash(user_data.password.as_ref().unwrap(), DEFAULT_COST)
            .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;
        diesel::update(users::table.filter(users::id.eq(uid)))
            .set((users::name.eq(user_data.name.as_ref().unwrap()), users::password.eq(&hashed)))
            .returning((users::id, users::name, users::email, users::is_admin, users::created_at))
            .get_result::<(i32, String, String, bool, DateTime<Utc>)>(&mut *db)
    } else if user_data.email.is_some() && user_data.password.is_some() {
        let hashed = hash(user_data.password.as_ref().unwrap(), DEFAULT_COST)
            .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;
        diesel::update(users::table.filter(users::id.eq(uid)))
            .set((users::email.eq(user_data.email.as_ref().unwrap()), users::password.eq(&hashed)))
            .returning((users::id, users::name, users::email, users::is_admin, users::created_at))
            .get_result::<(i32, String, String, bool, DateTime<Utc>)>(&mut *db)
    } else if user_data.name.is_some() {
        diesel::update(users::table.filter(users::id.eq(uid)))
            .set(users::name.eq(user_data.name.as_ref().unwrap()))
            .returning((users::id, users::name, users::email, users::is_admin, users::created_at))
            .get_result::<(i32, String, String, bool, DateTime<Utc>)>(&mut *db)
    } else if user_data.email.is_some() {
        diesel::update(users::table.filter(users::id.eq(uid)))
            .set(users::email.eq(user_data.email.as_ref().unwrap()))
            .returning((users::id, users::name, users::email, users::is_admin, users::created_at))
            .get_result::<(i32, String, String, bool, DateTime<Utc>)>(&mut *db)
    } else if user_data.password.is_some() {
        let hashed = hash(user_data.password.as_ref().unwrap(), DEFAULT_COST)
            .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;
        diesel::update(users::table.filter(users::id.eq(uid)))
            .set(users::password.eq(&hashed))
            .returning((users::id, users::name, users::email, users::is_admin, users::created_at))
            .get_result::<(i32, String, String, bool, DateTime<Utc>)>(&mut *db)
    } else if user_data.is_admin.is_some() {
        diesel::update(users::table.filter(users::id.eq(uid)))
            .set(users::is_admin.eq(user_data.is_admin.as_ref().unwrap()))
            .returning((users::id, users::name, users::email, users::is_admin, users::created_at))
            .get_result::<(i32, String, String, bool, DateTime<Utc>)>(&mut *db)
    } else {
        return Err(Custom(Status::BadRequest, "No fields to update".to_string()));
    };

    result
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))
        .map(|(uid_res, uname, uemail, uadmin, ucreated)| {
            Json(json!({
                "id": uid_res,
                "name": uname,
                "email": uemail,
                "is_admin": uadmin,
                "created_at": ucreated
            }))
        })
}

#[delete("/admin/users/<uid>")]
pub fn delete_user(
    mut db: DbConn,
    auth_user: AuthenticatedUser,
    uid: i32,
) -> Result<Status, Custom<String>> {
    require_admin(&auth_user)?;

    if uid == auth_user.user_id {
        return Err(Custom(Status::BadRequest, "Cannot delete your own account".to_string()));
    }

    users::table
        .filter(users::id.eq(uid))
        .select(users::id)
        .first::<i32>(&mut *db)
        .map_err(|_| Custom(Status::NotFound, "User not found".to_string()))?;

    diesel::delete(users::table.filter(users::id.eq(uid)))
        .execute(&mut *db)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    Ok(Status::Ok)
}

#[get("/admin/recipes")]
pub fn get_all_recipes(
    mut db: DbConn,
    auth_user: AuthenticatedUser,
) -> Result<Json<serde_json::Value>, Custom<String>> {
    require_admin(&auth_user)?;

    let rows = recipes::table
        .left_join(users::table)
        .select((
            recipes::id,
            recipes::title,
            recipes::short_description,
            recipes::author_id,
            users::name.nullable(),
            users::email.nullable(),
            recipes::is_public,
            recipes::created_at,
            recipes::updated_at,
        ))
        .order(recipes::created_at.desc())
        .load::<(
            i32,
            String,
            Option<String>,
            Option<i32>,
            Option<String>,
            Option<String>,
            Option<bool>,
            DateTime<Utc>,
            DateTime<Utc>,
        )>(&mut *db)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    let result: Vec<serde_json::Value> = rows
        .into_iter()
        .map(
            |(rid, rtitle, rshort, rauthor_id, rauthor_name, rauthor_email, ris_public, rcreated, rupdated)| {
                json!({
                    "id": rid,
                    "title": rtitle,
                    "short_description": rshort,
                    "author_id": rauthor_id,
                    "author_name": rauthor_name,
                    "author_email": rauthor_email,
                    "is_public": ris_public,
                    "created_at": rcreated,
                    "updated_at": rupdated
                })
            },
        )
        .collect();

    Ok(Json(json!({ "recipes": result })))
}

#[delete("/admin/recipes/<recipe_rid>")]
pub fn delete_any_recipe(
    mut db: DbConn,
    auth_user: AuthenticatedUser,
    recipe_rid: i32,
) -> Result<Status, Custom<String>> {
    require_admin(&auth_user)?;

    recipes::table
        .filter(recipes::id.eq(recipe_rid))
        .select(recipes::id)
        .first::<i32>(&mut *db)
        .map_err(|_| Custom(Status::NotFound, "Recipe not found".to_string()))?;

    let image_rows: Vec<String> = images::table
        .select(images::file_path)
        .filter(images::recipe_id.eq(recipe_rid))
        .load::<String>(&mut *db)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    diesel::delete(recipes::table.filter(recipes::id.eq(recipe_rid)))
        .execute(&mut *db)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    use std::fs;
    use std::path::Path;
    for file_path in image_rows {
        if Path::new(&file_path).exists() {
            if let Err(e) = fs::remove_file(&file_path) {
                eprintln!("Warning: Failed to delete file {}: {}", file_path, e);
            }
        }
    }

    Ok(Status::Ok)
}

#[get("/admin/categories")]
pub fn get_all_categories(
    mut db: DbConn,
    auth_user: AuthenticatedUser,
) -> Result<Json<serde_json::Value>, Custom<String>> {
    require_admin(&auth_user)?;

    let rows = categories::table
        .select((categories::id, categories::name, categories::created_at))
        .order(categories::name.asc())
        .load::<(i32, String, DateTime<Utc>)>(&mut *db)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    let result: Vec<serde_json::Value> = rows
        .into_iter()
        .map(|(cid, cname, ccreated)| {
            json!({
                "id": cid,
                "name": cname,
                "created_at": ccreated
            })
        })
        .collect();

    Ok(Json(json!({ "categories": result })))
}

#[post("/admin/categories", format = "json", data = "<category_data>")]
pub fn create_category(
    mut db: DbConn,
    auth_user: AuthenticatedUser,
    category_data: Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, Custom<String>> {
    require_admin(&auth_user)?;

    let name_val = category_data
        .get("name")
        .and_then(|n| n.as_str())
        .ok_or_else(|| Custom(Status::BadRequest, "Category name is required".to_string()))?;

    if name_val.trim().is_empty() {
        return Err(Custom(Status::BadRequest, "Category name cannot be empty".to_string()));
    }

    let existing = categories::table
        .filter(categories::name.eq(name_val))
        .select(categories::id)
        .first::<i32>(&mut *db)
        .ok();
    if existing.is_some() {
        return Err(Custom(Status::Conflict, "Category already exists".to_string()));
    }

    diesel::insert_into(categories::table)
        .values((
            categories::name.eq(name_val),
            categories::slug.eq(name_val.to_lowercase().replace(' ', "-")),
        ))
        .returning((categories::id, categories::name, categories::created_at))
        .get_result::<(i32, String, DateTime<Utc>)>(&mut *db)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))
        .map(|(cid, cname, ccreated)| {
            Json(json!({
                "id": cid,
                "name": cname,
                "created_at": ccreated
            }))
        })
}

#[delete("/admin/categories/<category_id>")]
pub fn delete_category(
    mut db: DbConn,
    auth_user: AuthenticatedUser,
    category_id: i32,
) -> Result<Status, Custom<String>> {
    require_admin(&auth_user)?;

    categories::table
        .filter(categories::id.eq(category_id))
        .select(categories::id)
        .first::<i32>(&mut *db)
        .map_err(|_| Custom(Status::NotFound, "Category not found".to_string()))?;

    diesel::delete(categories::table.filter(categories::id.eq(category_id)))
        .execute(&mut *db)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    Ok(Status::Ok)
}

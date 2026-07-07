use crate::db::DbConn;
use crate::schema::users::dsl::*;
use bcrypt::{hash, DEFAULT_COST};
use diesel::prelude::*;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserInput {
    pub name: String,
    pub email: String,
    pub password: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UserResponse {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
}

#[post("/users", format = "json", data = "<user>")]
pub fn add_user(mut db: DbConn, user: Json<UserInput>) -> Result<Json<Vec<UserResponse>>, Custom<String>> {
    let pwd = user
        .password
        .as_ref()
        .ok_or_else(|| Custom(Status::BadRequest, "Password is required".to_string()))?;
    let hashed_password =
        hash(pwd, DEFAULT_COST).map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    diesel::insert_into(users)
        .values((
            name.eq(&user.name),
            email.eq(&user.email),
            password.eq(&hashed_password),
        ))
        .execute(&mut *db)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    get_users_internal(&mut *db).map(Json)
}

#[get("/users")]
pub fn get_users(mut db: DbConn) -> Result<Json<Vec<UserResponse>>, Custom<String>> {
    get_users_internal(&mut *db).map(Json)
}

fn get_users_internal(db: &mut diesel::PgConnection) -> Result<Vec<UserResponse>, Custom<String>> {
    let results = users
        .select((id, name, email))
        .load::<(i32, String, String)>(&mut *db)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    Ok(results
        .into_iter()
        .map(|(uid, uname, uemail)| UserResponse {
            id: Some(uid),
            name: uname,
            email: uemail,
        })
        .collect())
}

#[put("/users/<uid>", format = "json", data = "<user>")]
pub fn update_user(
    mut db: DbConn,
    uid: i32,
    user: Json<UserInput>,
) -> Result<Json<Vec<UserResponse>>, Custom<String>> {
    diesel::update(users.filter(id.eq(uid)))
        .set((name.eq(&user.name), email.eq(&user.email)))
        .execute(&mut *db)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    get_users_internal(&mut *db).map(Json)
}

#[delete("/users/<uid>")]
pub fn delete_user(mut db: DbConn, uid: i32) -> Result<Status, Custom<String>> {
    diesel::delete(users.filter(id.eq(uid)))
        .execute(&mut *db)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;
    Ok(Status::NoContent)
}

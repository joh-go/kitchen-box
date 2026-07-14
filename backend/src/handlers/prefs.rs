use crate::auth::AuthenticatedUser;
use crate::db::DbConn;
use crate::schema::user_prefs::dsl::*;
use diesel::prelude::*;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::{get, put};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PrefsResponse {
    pub prefs: String,
}

#[get("/prefs")]
pub fn get_prefs(auth: AuthenticatedUser, mut db: DbConn) -> Result<Json<PrefsResponse>, status::Custom<String>> {
    let result = user_prefs
        .filter(user_id.eq(auth.user_id))
        .select(prefs)
        .first::<String>(&mut *db)
        .ok();

    match result {
        Some(p) => Ok(Json(PrefsResponse { prefs: p })),
        None => Ok(Json(PrefsResponse { prefs: "{}".to_string() })),
    }
}

#[derive(Debug, Deserialize)]
pub struct PrefsUpdate {
    pub prefs: String,
}

#[put("/prefs", format = "json", data = "<body>")]
pub fn save_prefs(
    body: Json<PrefsUpdate>,
    auth: AuthenticatedUser,
    mut db: DbConn,
) -> Result<&'static str, status::Custom<String>> {
    let uid = auth.user_id;

    diesel::insert_into(user_prefs)
        .values((user_id.eq(uid), prefs.eq(&body.prefs)))
        .on_conflict(user_id)
        .do_update()
        .set(prefs.eq(&body.prefs))
        .execute(&mut *db)
        .map_err(|e| status::Custom(Status::InternalServerError, format!("DB error: {}", e)))?;

    Ok("ok")
}

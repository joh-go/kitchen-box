use crate::auth::AuthenticatedUser;
use crate::db::{DbConn, DbPool};
use crate::models::NewImage;
use diesel::prelude::*;
use rocket::data::ToByteUnit;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put, Data, State};
use shared_types::RecipeImage;
use std::fs;
use std::path::Path;
use uuid::Uuid;

#[post("/recipes/<recipe_rid>/images", data = "<data>")]
pub async fn upload_image<'r>(
    pool: &State<DbPool>,
    auth_user: AuthenticatedUser,
    recipe_rid: i32,
    data: Data<'_>,
) -> Result<Json<RecipeImage>, Custom<String>> {
    let mut conn = pool.inner().get().map_err(|e| Custom(Status::ServiceUnavailable, e.to_string()))?;

    let owner = crate::schema::recipes::table
        .select(crate::schema::recipes::author_id)
        .filter(crate::schema::recipes::id.eq(recipe_rid))
        .first::<Option<i32>>(&mut *conn)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    if owner != Some(auth_user.user_id) {
        return Err(Custom(Status::Forbidden, "You don't own this recipe".to_string()));
    }

    let file_extension = "jpg";
    let uuid = Uuid::new_v4();
    let filename = format!("{}.{}", uuid, file_extension);

    let upload_dir = format!("uploads/recipes/{}", recipe_rid);
    fs::create_dir_all(&upload_dir)
        .map_err(|e| Custom(Status::InternalServerError, format!("Failed to create upload directory: {}", e)))?;

    let file_path = format!("{}/{}", upload_dir, filename);
    let data_slice = data
        .open(100.megabytes())
        .into_bytes()
        .await
        .map_err(|e| Custom(Status::InternalServerError, format!("Failed to read upload data: {}", e)))?
        .into_inner();
    fs::write(&file_path, &data_slice)
        .map_err(|e| Custom(Status::InternalServerError, format!("Failed to save file: {}", e)))?;

    let file_size = data_slice.len() as i32;

    let max_pos = crate::schema::images::table
        .select(diesel::dsl::max(crate::schema::images::position))
        .filter(crate::schema::images::recipe_id.eq(recipe_rid))
        .first::<Option<i32>>(&mut *conn)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    let position = max_pos.unwrap_or(0) + 1;

    let new_image = NewImage {
        recipe_id: recipe_rid,
        filename: filename.clone(),
        original_filename: Some(format!("image.{}", file_extension)),
        file_path: file_path.clone(),
        file_size,
        mime_type: Some(format!("image/{}", file_extension)),
        alt: None,
        is_primary: false,
        position,
    };

    diesel::insert_into(crate::schema::images::table)
        .values(&new_image)
        .returning((
            crate::schema::images::id,
            crate::schema::images::recipe_id,
            crate::schema::images::filename,
            crate::schema::images::original_filename,
            crate::schema::images::file_path,
            crate::schema::images::file_size,
            crate::schema::images::mime_type,
            crate::schema::images::alt,
            crate::schema::images::is_primary,
            crate::schema::images::position,
            crate::schema::images::uploaded_at,
        ))
        .get_result::<crate::models::Image>(&mut *conn)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))
        .map(|img| {
            Json(RecipeImage {
                id: Some(img.id),
                filename: img.filename,
                original_filename: img.original_filename,
                file_path: img.file_path,
                file_size: img.file_size,
                mime_type: img.mime_type,
                alt: img.alt,
                is_primary: img.is_primary,
                position: img.position,
                uploaded_at: Some(img.uploaded_at.to_string()),
            })
        })
}

#[get("/recipes/<recipe_rid>/images")]
pub fn get_recipe_images(
    mut db: DbConn,
    recipe_rid: i32,
) -> Result<Json<Vec<RecipeImage>>, Custom<String>> {
    let rows = crate::schema::images::table
        .filter(crate::schema::images::recipe_id.eq(recipe_rid))
        .order((crate::schema::images::position.asc(), crate::schema::images::uploaded_at.asc()))
        .load::<crate::models::Image>(&mut *db)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    let result: Vec<RecipeImage> = rows
        .into_iter()
        .map(|img| RecipeImage {
            id: Some(img.id),
            filename: img.filename,
            original_filename: img.original_filename,
            file_path: img.file_path,
            file_size: img.file_size,
            mime_type: img.mime_type,
            alt: img.alt,
            is_primary: img.is_primary,
            position: img.position,
            uploaded_at: Some(img.uploaded_at.to_string()),
        })
        .collect();

    Ok(Json(result))
}

#[put("/recipes/<recipe_rid>/images/<image_id>/primary")]
pub fn set_primary_image(
    mut db: DbConn,
    auth_user: AuthenticatedUser,
    recipe_rid: i32,
    image_id: i32,
) -> Result<Status, Custom<String>> {
    let owner = crate::schema::recipes::table
        .select(crate::schema::recipes::author_id)
        .filter(crate::schema::recipes::id.eq(recipe_rid))
        .first::<Option<i32>>(&mut *db)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    if owner != Some(auth_user.user_id) {
        return Err(Custom(Status::Forbidden, "You don't own this recipe".to_string()));
    }

    diesel::update(crate::schema::images::table.filter(crate::schema::images::recipe_id.eq(recipe_rid)))
        .set(crate::schema::images::is_primary.eq(false))
        .execute(&mut *db)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    diesel::update(
        crate::schema::images::table
            .filter(crate::schema::images::id.eq(image_id))
            .filter(crate::schema::images::recipe_id.eq(recipe_rid)),
    )
    .set(crate::schema::images::is_primary.eq(true))
    .execute(&mut *db)
    .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    Ok(Status::Ok)
}

#[delete("/recipes/<recipe_rid>/images/<image_id>")]
pub fn delete_image(
    mut db: DbConn,
    auth_user: AuthenticatedUser,
    recipe_rid: i32,
    image_id: i32,
) -> Result<Status, Custom<String>> {
    let owner = crate::schema::recipes::table
        .select(crate::schema::recipes::author_id)
        .filter(crate::schema::recipes::id.eq(recipe_rid))
        .first::<Option<i32>>(&mut *db)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    if owner != Some(auth_user.user_id) {
        return Err(Custom(Status::Forbidden, "You don't own this recipe".to_string()));
    }

    let file_path_val = crate::schema::images::table
        .select(crate::schema::images::file_path)
        .filter(crate::schema::images::id.eq(image_id))
        .filter(crate::schema::images::recipe_id.eq(recipe_rid))
        .first::<String>(&mut *db)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    diesel::delete(
        crate::schema::images::table
            .filter(crate::schema::images::id.eq(image_id))
            .filter(crate::schema::images::recipe_id.eq(recipe_rid)),
    )
    .execute(&mut *db)
    .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    if Path::new(&file_path_val).exists() {
        if let Err(e) = fs::remove_file(&file_path_val) {
            eprintln!("Warning: Failed to delete file {}: {}", file_path_val, e);
        }
    }

    Ok(Status::Ok)
}

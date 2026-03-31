use crate::db::execute_query;
use crate::models::RecipeImage;
use crate::auth::AuthenticatedUser;
use rocket::serde::json::Json;
use rocket::{http::Status, response::status::Custom, Data, State};
use rocket::data::ToByteUnit;
use tokio_postgres::Client;
use std::fs;
use std::path::Path;
use uuid::Uuid;
use chrono;

#[post("/api/recipes/<recipe_id>/images", data = "<data>")]
pub async fn upload_image<'r>(
    conn: &'r State<Client>,
    auth_user: AuthenticatedUser,
    recipe_id: i32,
    data: Data<'_>,
) -> Result<Json<RecipeImage>, Custom<String>> {
    // Check if user owns the recipe
    let ownership_check = conn
        .query_one("SELECT author_id FROM recipes WHERE id = $1", &[&recipe_id])
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    let author_id: i32 = ownership_check.get(0);

    if author_id != auth_user.user_id {
        return Err(Custom(Status::Forbidden, "You don't own this recipe".to_string()));
    }

    // Generate unique filename - for now use jpg, but we should get this from the frontend
    let file_extension = "jpg"; // TODO: Get actual file extension from frontend
    let uuid = Uuid::new_v4();
    let filename = format!("{}.{}", uuid, file_extension);
    
    // Create upload directory if it doesn't exist
    let upload_dir = format!("uploads/recipes/{}", recipe_id);
    fs::create_dir_all(&upload_dir)
        .map_err(|e| Custom(Status::InternalServerError, format!("Failed to create upload directory: {}", e)))?;
    
    // Save file to disk
    let file_path = format!("{}/{}", upload_dir, filename);
    let data_slice = data.open(1.megabytes()).into_bytes().await.unwrap().into_inner();
    fs::write(&file_path, &data_slice)
        .map_err(|e| Custom(Status::InternalServerError, format!("Failed to save file: {}", e)))?;

    // Get file size
    let file_size = data_slice.len() as i32;
    
    // Get the next position for this recipe's images
    let position_row = conn
        .query_one("SELECT COALESCE(MAX(position), 0) + 1 FROM images WHERE recipe_id = $1", &[&recipe_id])
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;
    
    let position: i32 = position_row.get(0);

    // Insert into database
    let row = conn
        .query_one(
            "INSERT INTO images (recipe_id, filename, original_filename, file_path, file_size, mime_type, alt, is_primary, position) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) 
             RETURNING id, filename, original_filename, file_path, file_size, mime_type, alt, is_primary, position, uploaded_at",
            &[
                &recipe_id,
                &filename,
                &format!("image.{}", file_extension),
                &file_path,
                &file_size,
                &format!("image/{}", file_extension),
                &None::<String>, // alt text
                &false,         // is_primary (first image uploaded is not automatically primary)
                &position,
            ],
        )
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    Ok(Json(RecipeImage {
        id: Some(row.get(0)),
        filename: row.get(1),
        original_filename: Some(row.get(2)),
        file_path: row.get(3),
        file_size: Some(row.get(4)),
        mime_type: Some(row.get(5)),
        alt: row.get(6),
        is_primary: Some(row.get(7)),
        position: Some(row.get(8)),
        uploaded_at: Some(row.get::<_, chrono::DateTime<chrono::Utc>>(9).to_string()),
    }))
}

#[get("/api/recipes/<recipe_id>/images")]
pub async fn get_recipe_images(
    conn: &State<Client>,
    recipe_id: i32,
) -> Result<Json<Vec<RecipeImage>>, Custom<String>> {
    let rows = conn
        .query(
            "SELECT id, filename, original_filename, file_path, file_size, mime_type, alt, is_primary, position, uploaded_at 
             FROM images WHERE recipe_id = $1 ORDER BY position ASC, uploaded_at ASC",
            &[&recipe_id],
        )
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    let mut images = Vec::new();
    for row in rows.iter() {
        let uploaded_at: Option<String> = row.get::<_, Option<chrono::DateTime<chrono::Utc>>>(9).map(|dt| dt.to_string());
        images.push(RecipeImage {
            id: Some(row.get(0)),
            filename: row.get(1),
            original_filename: Some(row.get(2)),
            file_path: row.get(3),
            file_size: Some(row.get(4)),
            mime_type: Some(row.get(5)),
            alt: row.get(6),
            is_primary: Some(row.get(7)),
            position: Some(row.get(8)),
            uploaded_at: uploaded_at,
        });
    }

    Ok(Json(images))
}

#[put("/api/recipes/<recipe_id>/images/<image_id>/primary")]
pub async fn set_primary_image(
    conn: &State<Client>,
    auth_user: AuthenticatedUser,
    recipe_id: i32,
    image_id: i32,
) -> Result<Status, Custom<String>> {
    // Check if user owns the recipe
    let ownership_check = conn
        .query_one("SELECT author_id FROM recipes WHERE id = $1", &[&recipe_id])
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    let author_id:i32= ownership_check.get(0);

    if author_id != auth_user.user_id {
        return Err(Custom(Status::Forbidden, "You don't own this recipe".to_string()));
    }

    // Remove primary flag from all images for this recipe
    execute_query(
        conn,
        "UPDATE images SET is_primary = false WHERE recipe_id = $1",
        &[&recipe_id],
    )
    .await?;

    // Set primary flag on the selected image
    execute_query(
        conn,
        "UPDATE images SET is_primary = true WHERE id = $1 AND recipe_id = $2",
        &[&image_id, &recipe_id],
    )
    .await?;

    Ok(Status::Ok)
}

#[delete("/api/recipes/<recipe_id>/images/<image_id>")]
pub async fn delete_image(
    conn: &State<Client>,
    auth_user: AuthenticatedUser,
    recipe_id: i32,
    image_id: i32,
) -> Result<Status, Custom<String>> {
    // Check if user owns the recipe
    let ownership_check = conn
        .query_one("SELECT author_id FROM recipes WHERE id = $1", &[&recipe_id])
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    let author_id : i32= ownership_check.get(0);

    if author_id != auth_user.user_id {
        return Err(Custom(Status::Forbidden, "You don't own this recipe".to_string()));
    }

    // Get file path before deleting
    let file_row = conn
        .query_one("SELECT file_path FROM images WHERE id = $1 AND recipe_id = $2", &[&image_id, &recipe_id])
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    let file_path: String = file_row.get(0);
    
    // Delete from database
    execute_query(
        conn,
        "DELETE FROM images WHERE id = $1 AND recipe_id = $2",
        &[&image_id, &recipe_id],
    )
    .await?;

    // Delete file from disk
    if Path::new(&file_path).exists() {
        fs::remove_file(&file_path)
            .map_err(|e| Custom(Status::InternalServerError, format!("Failed to delete file: {}", e)))?;
    }
    

    Ok(Status::Ok)
}

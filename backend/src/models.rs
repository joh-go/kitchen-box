use crate::schema::{categories, images, recipes};
use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub is_admin: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub is_admin: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateProfileRequest {
    pub name: String,
    pub current_password: Option<String>,
    pub new_password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
#[diesel(table_name = categories)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub parent_id: Option<i32>,
    pub position: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = categories)]
pub struct NewCategory {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub parent_id: Option<i32>,
    pub position: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
#[diesel(table_name = recipes)]
pub struct Recipe {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub short_description: Option<String>,
    pub ingredients: JsonValue,
    pub steps: JsonValue,
    pub prep_minutes: Option<i32>,
    pub cook_minutes: Option<i32>,
    pub servings: Option<i32>,
    pub notes: Option<String>,
    pub author_id: Option<i32>,
    pub is_public: Option<bool>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = recipes)]
pub struct NewRecipe {
    pub title: String,
    pub slug: String,
    pub short_description: Option<String>,
    pub ingredients: JsonValue,
    pub steps: JsonValue,
    pub prep_minutes: Option<i32>,
    pub cook_minutes: Option<i32>,
    pub servings: Option<i32>,
    pub notes: Option<String>,
    pub author_id: i32,
    pub is_public: Option<bool>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
#[diesel(table_name = recipe_categories)]
pub struct RecipeCategory {
    pub recipe_id: i32,
    pub category_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
#[diesel(table_name = images)]
pub struct Image {
    pub id: i32,
    pub recipe_id: Option<i32>,
    pub filename: String,
    pub original_filename: Option<String>,
    pub file_path: String,
    pub file_size: Option<i32>,
    pub mime_type: Option<String>,
    pub alt: Option<String>,
    pub is_primary: Option<bool>,
    pub position: Option<i32>,
    pub uploaded_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = images)]
pub struct NewImage {
    pub recipe_id: i32,
    pub filename: String,
    pub original_filename: Option<String>,
    pub file_path: String,
    pub file_size: i32,
    pub mime_type: Option<String>,
    pub alt: Option<String>,
    pub is_primary: bool,
    pub position: i32,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeWithDetails {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub short_description: Option<String>,
    pub ingredients: Vec<shared_types::Ingredient>,
    pub steps: JsonValue,
    pub prep_minutes: Option<i32>,
    pub cook_minutes: Option<i32>,
    pub servings: Option<i32>,
    pub notes: Option<String>,
    pub author_id: Option<i32>,
    pub is_public: Option<bool>,
    pub categories: Vec<shared_types::Category>,
    pub images: Vec<shared_types::RecipeImage>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeListItem {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub short_description: Option<String>,
    pub ingredients: Vec<shared_types::Ingredient>,
    pub steps: JsonValue,
    pub prep_minutes: Option<i32>,
    pub cook_minutes: Option<i32>,
    pub servings: Option<i32>,
    pub notes: Option<String>,
    pub author_id: Option<i32>,
    pub is_public: Option<bool>,
    pub categories: Vec<shared_types::Category>,
    pub images: Vec<shared_types::RecipeImage>,
}

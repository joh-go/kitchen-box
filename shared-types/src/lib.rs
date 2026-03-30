use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
    pub password: Option<String>, // Optional for API responses, required for creation
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Category {
    pub id: Option<i32>,
    pub name: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub parent_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Ingredient {
    pub name: String,
    pub amount: f64,
    pub unit: String,
    pub notes: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Recipe {
    pub id: Option<i32>,
    pub title: String,
    pub slug: Option<String>,
    pub short_description: Option<String>,
    pub ingredients: Vec<Ingredient>,
    pub steps: serde_json::Value,
    pub prep_minutes: Option<i32>,
    pub cook_minutes: Option<i32>,
    pub servings: Option<i32>,
    pub notes: Option<String>,
    pub author_id: Option<i32>,
    pub is_public: Option<bool>,
    pub categories: Vec<Category>,
    pub images: Vec<RecipeImage>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RecipeImage {
    pub id: Option<i32>,
    pub filename: String,
    pub original_filename: Option<String>,
    pub file_path: String,
    pub file_size: Option<i32>,
    pub mime_type: Option<String>,
    pub alt: Option<String>,
    pub is_primary: Option<bool>,
    pub position: Option<i32>,
    pub uploaded_at: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct NewRecipeImage {
    pub recipe_id: i32,
    pub filename: String,
    pub original_filename: String,
    pub file_path: String,
    pub file_size: i32,
    pub mime_type: String,
    pub alt: Option<String>,
    pub is_primary: bool,
    pub position: i32,
}

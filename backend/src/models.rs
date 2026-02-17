use rocket::serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Category {
    pub id: Option<i32>,
    pub name: String,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub parent_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Recipe {
    pub id: Option<i32>,
    pub title: String,
    pub slug: Option<String>,
    pub short_description: Option<String>,
    pub ingredients: JsonValue,
    pub steps: JsonValue,
    pub prep_minutes: Option<i32>,
    pub cook_minutes: Option<i32>,
    pub servings: Option<i32>,
    pub notes: Option<String>,
    pub author_id: Option<i32>,
    pub is_public: Option<bool>,
}

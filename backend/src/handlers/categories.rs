use crate::db::DbConn;
use crate::models::{Category, NewCategory};
use crate::schema::categories::dsl::*;
use diesel::prelude::*;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};

#[post("/categories", format = "json", data = "<category>")]
pub fn add_category(
    mut db: DbConn,
    category: Json<shared_types::Category>,
) -> Result<Json<Vec<shared_types::Category>>, Custom<String>> {
    let slug_val = category.slug.clone().unwrap_or_else(|| {
        category
            .name
            .to_lowercase()
            .chars()
            .map(|c| match c {
                'a'..='z' | '0'..='9' => c,
                ' ' => '-',
                _ => '-',
            })
            .collect::<String>()
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>()
            .join("-")
    });

    let new_cat = NewCategory {
        name: category.name.clone(),
        slug: slug_val,
        description: category.description.clone(),
        parent_id: category.parent_id,
        position: 0,
    };

    diesel::insert_into(categories)
        .values(&new_cat)
        .execute(&mut *db)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    get_categories_internal(&mut *db).map(Json)
}

#[get("/categories")]
pub fn get_categories(mut db: DbConn) -> Result<Json<Vec<shared_types::Category>>, Custom<String>> {
    get_categories_internal(&mut *db).map(Json)
}

fn get_categories_internal(
    db: &mut diesel::PgConnection,
) -> Result<Vec<shared_types::Category>, Custom<String>> {
    let results = categories
        .order((position.asc(), name.asc()))
        .load::<Category>(&mut *db)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    Ok(results
        .into_iter()
        .map(|c| shared_types::Category {
            id: Some(c.id),
            name: c.name,
            slug: Some(c.slug),
            description: c.description,
            parent_id: c.parent_id,
        })
        .collect())
}

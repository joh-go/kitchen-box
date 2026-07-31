use crate::auth::AuthenticatedUser;
use crate::db::DbConn;
use crate::models::{NewRecipe, Recipe};
use crate::schema::recipes::dsl::*;
use crate::schema::recipe_categories::dsl::*;
use diesel::prelude::*;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put};
use serde_json::Value as JsonValue;
use shared_types::{Category as SharedCategory, Ingredient, RecipeImage};

fn ingredients_from_json(value: &JsonValue) -> Vec<Ingredient> {
    serde_json::from_value(value.clone()).unwrap_or_default()
}

fn categories_for_recipe(db: &mut diesel::PgConnection, recipe_ids: &[i32]) -> Result<std::collections::HashMap<i32, Vec<SharedCategory>>, Custom<String>> {
    if recipe_ids.is_empty() {
        return Ok(std::collections::HashMap::new());
    }

    use crate::schema::categories::dsl::*;

    let rows = recipe_categories
        .inner_join(categories)
        .filter(recipe_id.eq_any(recipe_ids))
        .select((
            crate::schema::recipe_categories::recipe_id,
            crate::schema::categories::dsl::id,
            crate::schema::categories::dsl::name,
            crate::schema::categories::dsl::slug,
            crate::schema::categories::dsl::description,
            crate::schema::categories::dsl::parent_id,
        ))
        .load::<(i32, i32, String, String, Option<String>, Option<i32>)>(db)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    let mut map: std::collections::HashMap<i32, Vec<SharedCategory>> =
        std::collections::HashMap::new();
    for (rid, cid, cname, cslug, cdesc, cparent) in rows {
        let cslug = Some(cslug);
        map.entry(rid).or_default().push(SharedCategory {
            id: Some(cid),
            name: cname,
            slug: cslug,
            description: cdesc,
            parent_id: cparent,
        });
    }
    Ok(map)
}

fn images_for_recipe_by_recipe_ids(db: &mut diesel::PgConnection, recipe_ids: &[i32]) -> Result<std::collections::HashMap<i32, Vec<RecipeImage>>, Custom<String>> {
    use crate::schema::images::dsl::*;

    if recipe_ids.is_empty() {
        return Ok(std::collections::HashMap::new());
    }

    let rows = images
        .filter(crate::schema::images::dsl::recipe_id.eq_any(recipe_ids))
        .order((crate::schema::images::dsl::position.asc(), crate::schema::images::dsl::uploaded_at.asc()))
        .load::<crate::models::Image>(db)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    let mut map: std::collections::HashMap<i32, Vec<RecipeImage>> =
        std::collections::HashMap::new();
    for img in rows {
        if let Some(rid) = img.recipe_id {
            map.entry(rid).or_default().push(RecipeImage {
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
            });
        }
    }
    Ok(map)
}

fn recipe_to_list_item(
    recipe: Recipe,
    cat_map: &std::collections::HashMap<i32, Vec<SharedCategory>>,
    img_map: &std::collections::HashMap<i32, Vec<RecipeImage>>,
) -> shared_types::Recipe {
    shared_types::Recipe {
        id: Some(recipe.id),
        title: recipe.title,
        slug: Some(recipe.slug),
        short_description: recipe.short_description,
        ingredients: ingredients_from_json(&recipe.ingredients),
        steps: recipe.steps,
        prep_minutes: recipe.prep_minutes,
        cook_minutes: recipe.cook_minutes,
        servings: recipe.servings,
        notes: recipe.notes,
        author_id: recipe.author_id,
        is_public: recipe.is_public,
        categories: cat_map.get(&recipe.id).cloned().unwrap_or_default(),
        images: img_map.get(&recipe.id).cloned().unwrap_or_default(),
    }
}

#[post("/recipes", format = "json", data = "<recipe>")]
pub fn add_recipe(
    mut db: DbConn,
    auth_user: AuthenticatedUser,
    recipe: Json<shared_types::Recipe>,
) -> Result<Json<shared_types::Recipe>, Custom<String>> {
    let ingredients_json =
        serde_json::to_value(&recipe.ingredients).unwrap_or(JsonValue::Null);
    let slug_val = recipe.slug.as_deref().unwrap_or("");

    let new_recipe = NewRecipe {
        title: recipe.title.clone(),
        slug: slug_val.to_string(),
        short_description: recipe.short_description.clone(),
        ingredients: ingredients_json,
        steps: recipe.steps.clone(),
        prep_minutes: recipe.prep_minutes,
        cook_minutes: recipe.cook_minutes,
        servings: recipe.servings,
        notes: recipe.notes.clone(),
        author_id: auth_user.user_id,
        is_public: recipe.is_public,
    };

    diesel::insert_into(crate::schema::recipes::table)
        .values(&new_recipe)
        .returning((
            crate::schema::recipes::id,
            crate::schema::recipes::title,
            crate::schema::recipes::slug,
            crate::schema::recipes::short_description,
            crate::schema::recipes::ingredients,
            crate::schema::recipes::steps,
            crate::schema::recipes::prep_minutes,
            crate::schema::recipes::cook_minutes,
            crate::schema::recipes::servings,
            crate::schema::recipes::notes,
            crate::schema::recipes::author_id,
            crate::schema::recipes::is_public,
            crate::schema::recipes::created_at,
            crate::schema::recipes::updated_at,
        ))
        .get_result::<Recipe>(&mut *db)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))
        .map(|r| {
            Json(shared_types::Recipe {
                id: Some(r.id),
                title: r.title,
                slug: Some(r.slug),
                short_description: r.short_description,
                ingredients: ingredients_from_json(&r.ingredients),
                steps: r.steps,
                prep_minutes: r.prep_minutes,
                cook_minutes: r.cook_minutes,
                servings: r.servings,
                notes: r.notes,
                author_id: r.author_id,
                is_public: r.is_public,
                categories: Vec::new(),
                images: Vec::new(),
            })
        })
}

#[get("/recipes")]
pub fn get_recipes(mut db: DbConn) -> Result<Json<Vec<shared_types::Recipe>>, Custom<String>> {
    let all_recipes = crate::schema::recipes::table
        .order(crate::schema::recipes::created_at.desc())
        .load::<Recipe>(&mut *db)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    let ids: Vec<i32> = all_recipes.iter().map(|r| r.id).collect();
    let cat_map = categories_for_recipe(&mut *db, &ids)?;
    let img_map = images_for_recipe_by_recipe_ids(&mut *db, &ids)?;

    let result: Vec<shared_types::Recipe> = all_recipes
        .into_iter()
        .map(|r| recipe_to_list_item(r, &cat_map, &img_map))
        .collect();

    Ok(Json(result))
}

#[get("/my-recipes")]
pub fn get_my_recipes(
    mut db: DbConn,
    auth_user: AuthenticatedUser,
) -> Result<Json<Vec<shared_types::Recipe>>, Custom<String>> {
    let all_recipes = crate::schema::recipes::table
        .filter(author_id.eq(&auth_user.user_id))
        .order(crate::schema::recipes::created_at.desc())
        .load::<Recipe>(&mut *db)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    let ids: Vec<i32> = all_recipes.iter().map(|r| r.id).collect();
    let cat_map = categories_for_recipe(&mut *db, &ids)?;
    let img_map = images_for_recipe_by_recipe_ids(&mut *db, &ids)?;

    let result: Vec<shared_types::Recipe> = all_recipes
        .into_iter()
        .map(|r| recipe_to_list_item(r, &cat_map, &img_map))
        .collect();

    Ok(Json(result))
}

#[get("/recipes/<rid>")]
pub fn get_recipe(
    mut db: DbConn,
    rid: i32,
) -> Result<Json<shared_types::Recipe>, Custom<String>> {
    let recipe = crate::schema::recipes::table
        .filter(crate::schema::recipes::id.eq(rid))
        .first::<Recipe>(&mut *db)
        .map_err(|e| Custom(Status::NotFound, format!("Recipe not found: {}", e)))?;

    let cat_map = categories_for_recipe(&mut *db, &[rid])?;
    let img_map = images_for_recipe_by_recipe_ids(&mut *db, &[rid])?;

    Ok(Json(recipe_to_list_item(recipe, &cat_map, &img_map)))
}

#[put("/recipes/<rid>", format = "json", data = "<recipe>")]
pub fn update_recipe(
    mut db: DbConn,
    auth_user: AuthenticatedUser,
    rid: i32,
    recipe: Json<shared_types::Recipe>,
) -> Result<Json<shared_types::Recipe>, Custom<String>> {
    let existing = crate::schema::recipes::table
        .select(author_id)
        .filter(crate::schema::recipes::id.eq(rid))
        .first::<Option<i32>>(&mut *db)
        .map_err(|e| Custom(Status::NotFound, format!("Recipe not found: {}", e)))?;

    if existing != Some(auth_user.user_id) {
        return Err(Custom(
            Status::Forbidden,
            "You can only edit your own recipes".to_string(),
        ));
    }

    let ingredients_json =
        serde_json::to_value(&recipe.ingredients).unwrap_or(JsonValue::Null);
    let slug_val = recipe.slug.as_deref().unwrap_or("");
    let short_desc_val = recipe.short_description.as_deref().unwrap_or("");
    let notes_val = recipe.notes.as_deref().unwrap_or("");
    let is_public_val = recipe.is_public.unwrap_or(true);

    diesel::update(crate::schema::recipes::table.filter(crate::schema::recipes::id.eq(rid)))
        .set((
            crate::schema::recipes::title.eq(&recipe.title),
            crate::schema::recipes::slug.eq(slug_val),
            crate::schema::recipes::short_description.eq(short_desc_val),
            crate::schema::recipes::ingredients.eq(&ingredients_json),
            crate::schema::recipes::steps.eq(&recipe.steps),
            crate::schema::recipes::prep_minutes.eq(recipe.prep_minutes),
            crate::schema::recipes::cook_minutes.eq(recipe.cook_minutes),
            crate::schema::recipes::servings.eq(recipe.servings),
            crate::schema::recipes::notes.eq(notes_val),
            crate::schema::recipes::is_public.eq(is_public_val),
        ))
        .execute(&mut *db)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    get_recipe(db, rid)
}

#[delete("/recipes/<rid>")]
pub fn delete_recipe(
    mut db: DbConn,
    auth_user: AuthenticatedUser,
    rid: i32,
) -> Result<Status, Custom<String>> {
    let existing = crate::schema::recipes::table
        .select(author_id)
        .filter(crate::schema::recipes::id.eq(rid))
        .first::<Option<i32>>(&mut *db)
        .map_err(|e| Custom(Status::NotFound, format!("Recipe not found: {}", e)))?;

    if existing != Some(auth_user.user_id) {
        return Err(Custom(
            Status::Forbidden,
            "You can only delete your own recipes".to_string(),
        ));
    }

    let image_rows: Vec<String> = crate::schema::images::table
        .select(crate::schema::images::file_path)
        .filter(crate::schema::images::recipe_id.eq(rid))
        .load::<String>(&mut *db)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    diesel::delete(crate::schema::recipes::table.filter(crate::schema::recipes::id.eq(rid)))
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

    Ok(Status::NoContent)
}

#[post("/recipes/<rid>/categories/<cid>")]
pub fn assign_category(
    mut db: DbConn,
    auth_user: AuthenticatedUser,
    rid: i32,
    cid: i32,
) -> Result<Status, Custom<String>> {
    let existing = crate::schema::recipes::table
        .select(author_id)
        .filter(crate::schema::recipes::id.eq(rid))
        .first::<Option<i32>>(&mut *db)
        .map_err(|e| Custom(Status::NotFound, format!("Recipe not found: {}", e)))?;

    if existing != Some(auth_user.user_id) {
        return Err(Custom(
            Status::Forbidden,
            "You can only edit your own recipes".to_string(),
        ));
    }

    diesel::delete(recipe_categories.filter(crate::schema::recipe_categories::recipe_id.eq(rid)))
        .execute(&mut *db)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    diesel::insert_into(recipe_categories)
        .values((
            crate::schema::recipe_categories::recipe_id.eq(rid),
            crate::schema::recipe_categories::category_id.eq(cid),
        ))
        .execute(&mut *db)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    Ok(Status::Created)
}

#[delete("/recipes/<rid>/categories")]
pub fn clear_categories(
    mut db: DbConn,
    auth_user: AuthenticatedUser,
    rid: i32,
) -> Result<Status, Custom<String>> {
    let existing = crate::schema::recipes::table
        .select(author_id)
        .filter(crate::schema::recipes::id.eq(rid))
        .first::<Option<i32>>(&mut *db)
        .map_err(|e| Custom(Status::NotFound, format!("Recipe not found: {}", e)))?;

    if existing != Some(auth_user.user_id) {
        return Err(Custom(
            Status::Forbidden,
            "You can only edit your own recipes".to_string(),
        ));
    }

    diesel::delete(recipe_categories.filter(crate::schema::recipe_categories::recipe_id.eq(rid)))
        .execute(&mut *db)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    Ok(Status::NoContent)
}

#[get("/recipes/export")]
pub fn export_recipes(mut db: DbConn) -> Result<Json<shared_types::RecipesExport>, Custom<String>> {
    let all_recipes = crate::schema::recipes::table
        .order(crate::schema::recipes::created_at.desc())
        .load::<Recipe>(&mut *db)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    let ids: Vec<i32> = all_recipes.iter().map(|r| r.id).collect();
    let cat_map = categories_for_recipe(&mut *db, &ids)?;

    let recipe_items: Vec<shared_types::RecipeExportItem> = all_recipes
        .iter()
        .map(|r| {
            let cats = cat_map
                .get(&r.id)
                .cloned()
                .unwrap_or_default()
                .iter()
                .map(|c| c.name.clone())
                .collect();

            shared_types::RecipeExportItem {
                title: r.title.clone(),
                short_description: r.short_description.clone(),
                ingredients: ingredients_from_json(&r.ingredients),
                steps: r.steps.clone(),
                prep_minutes: r.prep_minutes,
                cook_minutes: r.cook_minutes,
                servings: r.servings,
                notes: r.notes.clone(),
                is_public: r.is_public,
                categories: cats,
                images: Vec::new(),
            }
        })
        .collect();

    let exported_at = chrono::Utc::now().to_rfc3339();

    Ok(Json(shared_types::RecipesExport {
        version: "1.0".to_string(),
        exported_at,
        source: "kitchen-box".to_string(),
        recipes: recipe_items,
    }))
}

fn generate_slug(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| match c {
            'a'..='z' | '0'..='9' => c,
            ' ' => '-',
            '-' => '-',
            _ => '-',
        })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join("-")
}

fn ensure_unique_slug(
    db: &mut diesel::PgConnection,
    base_slug: &str,
) -> Result<String, Custom<String>> {
    use crate::schema::recipes::dsl;
    let slug_base = if base_slug.is_empty() { "recipe" } else { base_slug };

    let existing_count: i64 = dsl::recipes
        .filter(dsl::slug.eq(slug_base))
        .count()
        .get_result(db)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    if existing_count == 0 {
        return Ok(slug_base.to_string());
    }

    for i in 2..1000 {
        let candidate = format!("{}-{}", slug_base, i);
        let count: i64 = dsl::recipes
            .filter(dsl::slug.eq(&candidate))
            .count()
            .get_result(db)
            .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;
        if count == 0 {
            return Ok(candidate);
        }
    }

    Err(Custom(
        Status::InternalServerError,
        "Could not generate unique slug".to_string(),
    ))
}

fn find_or_create_category(
    db: &mut diesel::PgConnection,
    cat_name: &str,
) -> Result<i32, Custom<String>> {
    use crate::schema::categories::dsl::*;

    let existing = categories
        .filter(name.eq(cat_name))
        .select(crate::schema::categories::id)
        .first::<i32>(db)
        .ok();

    if let Some(cid) = existing {
        return Ok(cid);
    }

    let slug_val = generate_slug(cat_name);
    let new_cat = crate::models::NewCategory {
        name: cat_name.to_string(),
        slug: slug_val.clone(),
        description: None,
        parent_id: None,
        position: 0,
    };

    diesel::insert_into(categories)
        .values(&new_cat)
        .returning(crate::schema::categories::id)
        .get_result::<i32>(db)
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))
}

#[post("/recipes/import", format = "json", data = "<payload>")]
pub fn import_recipes(
    mut db: DbConn,
    auth_user: AuthenticatedUser,
    payload: Json<shared_types::RecipesExport>,
) -> Result<Json<shared_types::ImportResult>, Custom<String>> {
    let mut created = 0;
    let mut skipped = 0;
    let mut errors: Vec<String> = Vec::new();

    for item in &payload.recipes {
        let result = {
            let slug_base = generate_slug(&item.title);
            let recipe_slug = ensure_unique_slug(&mut *db, &slug_base)?;

            let ingredients_json =
                serde_json::to_value(&item.ingredients).unwrap_or(JsonValue::Null);

            let new_recipe = NewRecipe {
                title: item.title.clone(),
                slug: recipe_slug.clone(),
                short_description: item.short_description.clone(),
                ingredients: ingredients_json,
                steps: item.steps.clone(),
                prep_minutes: item.prep_minutes,
                cook_minutes: item.cook_minutes,
                servings: item.servings,
                notes: item.notes.clone(),
                author_id: auth_user.user_id,
                is_public: item.is_public,
            };

            let inserted: Recipe = diesel::insert_into(crate::schema::recipes::table)
                .values(&new_recipe)
                .get_result::<Recipe>(&mut *db)
                .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

            Ok::<(i32, Recipe), Custom<String>>((inserted.id, inserted))
        };

        match result {
            Ok((new_id, _recipe)) => {
                for cat_name in &item.categories {
                    match find_or_create_category(&mut *db, cat_name) {
                        Ok(cat_id) => {
                            use crate::schema::recipe_categories::dsl as rc_dsl;
                            let _ = diesel::insert_into(rc_dsl::recipe_categories)
                                .values((
                                    rc_dsl::recipe_id.eq(new_id),
                                    rc_dsl::category_id.eq(cat_id),
                                ))
                                .execute(&mut *db);
                        }
                        Err(e) => {
                            errors
                                .push(format!("{}: category '{}': {}", item.title, cat_name, e.0));
                        }
                    }
                }
                created += 1;
            }
            Err(e) => {
                skipped += 1;
                errors.push(format!("{}: {}", item.title, e.0));
            }
        }
    }

    Ok(Json(shared_types::ImportResult {
        created,
        skipped,
        errors,
    }))
}

use rocket::{response::status::Custom, http::Status};
use tokio_postgres::Client;

pub async fn execute_query(
    client: &Client,
    query: &str,
    params: &[&(dyn tokio_postgres::types::ToSql + Sync)],
) -> Result<u64, Custom<String>> {
    client
        .execute(query, params).await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))
}

pub async fn init_tables(client: &Client) -> Result<(), Custom<String>> {
    client.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL,
            password TEXT NOT NULL
        )",
        &[]
    ).await.map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    client.execute(
        "CREATE TABLE IF NOT EXISTS categories (
            id SERIAL PRIMARY KEY,
            name TEXT NOT NULL,
            slug TEXT NOT NULL,
            description TEXT,
            parent_id INTEGER REFERENCES categories(id) ON DELETE SET NULL,
            position INTEGER DEFAULT 0,
            created_at TIMESTAMPTZ DEFAULT now()
        )",
        &[]
    ).await.map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    client.execute(
        "CREATE TABLE IF NOT EXISTS recipes (
            id SERIAL PRIMARY KEY,
            title TEXT NOT NULL,
            slug TEXT NOT NULL,
            short_description TEXT,
            ingredients JSONB NOT NULL,
            steps JSONB NOT NULL,
            prep_minutes INTEGER,
            cook_minutes INTEGER,
            servings INTEGER,
            notes TEXT,
            author_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
            is_public BOOLEAN DEFAULT true,
            created_at TIMESTAMPTZ DEFAULT now(),
            updated_at TIMESTAMPTZ DEFAULT now()
        )",
        &[]
    ).await.map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    client.execute(
        "CREATE TABLE IF NOT EXISTS recipe_categories (
            recipe_id INTEGER REFERENCES recipes(id) ON DELETE CASCADE,
            category_id INTEGER REFERENCES categories(id) ON DELETE CASCADE,
            PRIMARY KEY (recipe_id, category_id)
        )",
        &[]
    ).await.map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    client.execute(
        "CREATE TABLE IF NOT EXISTS images (
            id SERIAL PRIMARY KEY,
            recipe_id INTEGER REFERENCES recipes(id) ON DELETE CASCADE,
            url TEXT,
            alt TEXT,
            is_primary BOOLEAN DEFAULT false,
            uploaded_at TIMESTAMPTZ DEFAULT now()
        )",
        &[]
    ).await.map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    client.execute(
        "CREATE TABLE IF NOT EXISTS recipe_versions (
            id SERIAL PRIMARY KEY,
            recipe_id INTEGER REFERENCES recipes(id) ON DELETE CASCADE,
            payload JSONB,
            created_at TIMESTAMPTZ DEFAULT now(),
            author_id INTEGER REFERENCES users(id) ON DELETE SET NULL
        )",
        &[]
    ).await.map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;

    Ok(())
}

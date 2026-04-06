use rocket::{response::status::Custom, http::Status};
use tokio_postgres::Client;
use bcrypt::{hash, DEFAULT_COST};

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
            email TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL,
            is_admin BOOLEAN DEFAULT false,
            created_at TIMESTAMPTZ DEFAULT now()
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
            filename TEXT NOT NULL,
            original_filename TEXT,
            file_path TEXT NOT NULL,
            file_size INTEGER,
            mime_type TEXT,
            alt TEXT,
            is_primary BOOLEAN DEFAULT false,
            position INTEGER DEFAULT 0,
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

pub async fn create_default_admin(client: &Client) -> Result<(), Custom<String>> {
    // Check if any admin user exists
    let admin_count = client
        .query_one("SELECT COUNT(*) as count FROM users WHERE is_admin = true", &[])
        .await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;
    
    let count: i64 = admin_count.get("count");
    
    if count == 0 {
        // Create default admin user
        let admin_email = "admin@recipes.local";
        let admin_password = "admin123"; // Default password, should be changed
        let hashed_password = hash(admin_password, DEFAULT_COST)
            .map_err(|e| Custom(Status::InternalServerError, format!("Failed to hash password: {}", e)))?;
        
        client.execute(
            "INSERT INTO users (name, email, password, is_admin) VALUES ($1, $2, $3, $4)",
            &[&"Admin User", &admin_email, &hashed_password, &true]
        ).await.map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;
        
        println!("🔑 Default admin user created:");
        println!("   Email: {}", admin_email);
        println!("   Password: {}", admin_password);
        println!("   ⚠️  Please change this password after first login!");
    }
    
    Ok(())
}

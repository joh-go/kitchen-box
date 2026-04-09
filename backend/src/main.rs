#[macro_use]
extern crate rocket;

mod auth;
mod db;
mod handlers;
mod models;

use handlers::{auth::login, auth::logout, auth::get_current_user, auth::update_current_user, categories, recipes, users, images, admin};
use rocket::http::Method;
use rocket::fs::{FileServer, NamedFile};
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
use std::collections::HashSet;
use tokio_postgres::NoTls;
use std::env;
use std::path::{Path, PathBuf};

#[get("/<path..>")]
async fn frontend_index(path: PathBuf) -> Option<NamedFile> {
    let mut path = path.to_path_buf();
    
    // If the path exists as a file in the frontend/dist directory, serve it
    // Otherwise, serve index.html for SPA routing
    if !path.extension().is_some() {
        path = PathBuf::from("index.html");
    }
    
    NamedFile::open(Path::new("frontend/dist").join(path)).await.ok()
}

#[launch]
async fn rocket() -> _ {
    dotenv::dotenv().ok();
    
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let (client, connection) = tokio_postgres::connect(
        &database_url,
        NoTls,
    )
    .await
    .expect("Failed to connect to Postgres");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Failed to connect to Postgres: {}", e);
        }
    });

    db::init_tables(&client)
        .await
        .expect("Failed to initialize database tables");

    //db::create_default_admin(&client)
    //    .await
    //    .expect("Failed to create default admin user");

    let mut methods = HashSet::new();
    methods.insert(Method::Get.into());
    methods.insert(Method::Post.into());
    methods.insert(Method::Put.into());
    methods.insert(Method::Delete.into());
    methods.insert(Method::Options.into());

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(methods)
        .allowed_headers(AllowedHeaders::all()) // Allow all headers for debugging
        .allow_credentials(true)
        .to_cors()
        .expect("Error while building CORS");

    rocket::build()
        .manage(client)
        .mount("/", routes![
            login,
            logout,
            get_current_user,
            update_current_user,
            users::add_user,
            users::get_users,
            users::update_user,
            users::delete_user,
            categories::add_category,
            categories::get_categories,
            recipes::add_recipe,
            recipes::get_recipes,
            recipes::get_my_recipes,
            recipes::get_recipe,
            recipes::update_recipe,
            recipes::delete_recipe,
            recipes::assign_category,
            recipes::clear_categories,
            images::upload_image,
            images::get_recipe_images,
            images::set_primary_image,
            images::delete_image,
            admin::get_all_users,
            admin::create_user,
            admin::update_user,
            admin::delete_user,
            admin::get_all_recipes,
            admin::delete_any_recipe,
            admin::get_all_categories,
            admin::create_category,
            admin::delete_category,
            admin::check_admin_exists,
            admin::setup_initial_admin,
            frontend_index,
        ])
        .mount("/uploads", FileServer::from("uploads"))
        .mount("/app", FileServer::from("frontend/dist"))
        .attach(cors)
}

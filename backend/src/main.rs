mod auth;
mod db;
mod handlers;
mod models;
mod schema;

use db::init_db_pool;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use rocket::fairing::AdHoc;
use rocket::fs::FileServer;
use rocket::http::Header;
use rocket::{get, launch, options, routes};
use std::path::PathBuf;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[launch]
fn rocket() -> _ {
    dotenvy::dotenv().ok();
    env_logger::init();

    let db_pool = init_db_pool();

    let mut conn = db_pool.get().expect("Failed to get DB connection for migrations");
    conn.run_pending_migrations(MIGRATIONS).expect("Failed to run migrations");
    drop(conn);

    let mut rocket = rocket::build().manage(db_pool);

    let cors_origin: Option<&'static str> = match std::env::var("CORS_ORIGINS").or_else(|_| std::env::var("CORS_ORIGIN")) {
        Ok(v) if !v.is_empty() => Some(Box::leak(v.into_boxed_str())),
        _ => {
            if std::env::var("FRONTEND_DIST").is_ok() {
                None
            } else {
                Some("http://127.0.0.1:8080")
            }
        }
    };

    if let Some(origin) = cors_origin {
        rocket = rocket.attach(AdHoc::on_response(
            "CORS & Security Headers",
            move |_, response| {
                Box::pin(async move {
                    response.set_header(Header::new("Access-Control-Allow-Origin", origin));
                    response.set_header(Header::new(
                        "Access-Control-Allow-Methods",
                        "GET, POST, PUT, DELETE, OPTIONS",
                    ));
                    response.set_header(Header::new(
                        "Access-Control-Allow-Headers",
                        "Content-Type, Authorization",
                    ));
                    response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
                    response.set_header(Header::new("X-Content-Type-Options", "nosniff"));
                    response.set_header(Header::new("X-Frame-Options", "DENY"));
                })
            },
        ));
    }

    rocket = rocket.mount("/", routes![api_cors]);

    if let Ok(frontend_dir) = std::env::var("FRONTEND_DIST") {
        let path = PathBuf::from(&frontend_dir);
        if path.exists() {
            rocket = rocket.mount("/", FileServer::from(path));
            log::info!("Serving frontend from {}", frontend_dir);
        } else {
            log::warn!("FRONTEND_DIST directory does not exist: {}", frontend_dir);
            rocket = rocket.mount("/", routes![index]);
        }
    } else {
        rocket = rocket.mount("/", routes![index]);
    }

    rocket = rocket.mount("/uploads", FileServer::from("uploads").rank(0));

    rocket.mount(
        "/api",
        routes![
            handlers::auth::login,
            handlers::auth::logout,
            handlers::auth::get_current_user,
            handlers::auth::update_current_user,
            handlers::users::add_user,
            handlers::users::get_users,
            handlers::users::update_user,
            handlers::users::delete_user,
            handlers::categories::add_category,
            handlers::categories::get_categories,
            handlers::recipes::add_recipe,
            handlers::recipes::get_recipes,
            handlers::recipes::get_my_recipes,
            handlers::recipes::get_recipe,
            handlers::recipes::update_recipe,
            handlers::recipes::delete_recipe,
            handlers::recipes::assign_category,
            handlers::recipes::clear_categories,
            handlers::images::upload_image,
            handlers::images::get_recipe_images,
            handlers::images::set_primary_image,
            handlers::images::delete_image,
            handlers::admin::get_all_users,
            handlers::admin::create_user,
            handlers::admin::update_user,
            handlers::admin::delete_user,
            handlers::admin::get_all_recipes,
            handlers::admin::delete_any_recipe,
            handlers::admin::get_all_categories,
            handlers::admin::create_category,
            handlers::admin::delete_category,
            handlers::admin::check_admin_exists,
            handlers::admin::setup_initial_admin,
        ],
    )
}

#[get("/")]
fn index() -> &'static str {
    "Recipes API - Running"
}

#[options("/api/<_path..>")]
fn api_cors(_path: std::path::PathBuf) -> rocket::http::Status {
    rocket::http::Status::Ok
}

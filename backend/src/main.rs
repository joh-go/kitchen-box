#[macro_use]
extern crate rocket;

mod db;
mod handlers;
mod models;

use handlers::{categories, recipes, users};
use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
use std::collections::HashSet;
use tokio_postgres::NoTls;

#[launch]
async fn rocket() -> _ {
    let (client, connection) = tokio_postgres::connect(
        "host=localhost user=postgres password=postgres dbname=postgres",
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

    let mut methods = HashSet::new();
    methods.insert(Method::Get.into());
    methods.insert(Method::Post.into());
    methods.insert(Method::Put.into());
    methods.insert(Method::Delete.into());
    methods.insert(Method::Options.into());

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(methods)
        .allowed_headers(AllowedHeaders::some(&["Content-Type", "Authorization"]))
        .allow_credentials(true)
        .to_cors()
        .expect("Error while building CORS");

    rocket::build()
        .manage(client)
        .mount(
            "/",
            routes![
                users::add_user,
                users::get_users,
                users::update_user,
                users::delete_user,
                categories::add_category,
                categories::get_categories,
                recipes::add_recipe,
                recipes::get_recipes,
                recipes::get_recipe,
                recipes::update_recipe,
                recipes::delete_recipe,
                recipes::assign_category
            ],
        )
        .attach(cors)
}

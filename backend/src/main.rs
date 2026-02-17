#[macro_use]
extern crate rocket;

mod db;
mod handlers;
mod models;

use handlers::{categories, recipes, users};
use rocket_cors::{AllowedOrigins, CorsOptions};
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

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
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
                recipes::delete_recipe,
                recipes::assign_category
            ],
        )
        .attach(cors)
}

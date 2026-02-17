use rocket::serde::json::Json;
use rocket::{State, response::status::Custom, http::Status};
use tokio_postgres::Client;
use crate::models::User;
use crate::db::execute_query;

#[post("/api/users", data = "<user>")]
pub async fn add_user(
    conn: &State<Client>,
    user: Json<User>
) -> Result<Json<Vec<User>>, Custom<String>> {
    execute_query(
        conn,
        "INSERT INTO users (name, email) VALUES ($1, $2)",
        &[&user.name, &user.email]
    ).await?;
    get_users(conn).await
}

#[get("/api/users")]
pub async fn get_users(conn: &State<Client>) -> Result<Json<Vec<User>>, Custom<String>> {
    get_users_from_db(conn).await.map(Json)
}

pub async fn get_users_from_db(client: &Client) -> Result<Vec<User>, Custom<String>> {
    let users = client
        .query("SELECT id, name, email FROM users", &[]).await
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?
        .iter()
        .map(|row| User { id: Some(row.get(0)), name: row.get(1), email: row.get(2) })
        .collect::<Vec<User>>();

    Ok(users)
}

#[put("/api/users/<id>", data = "<user>")]
pub async fn update_user(
    conn: &State<Client>,
    id: i32,
    user: Json<User>
) -> Result<Json<Vec<User>>, Custom<String>> {
    execute_query(
        conn,
        "UPDATE users SET name = $1, email = $2 WHERE id = $3",
        &[&user.name, &user.email, &id]
    ).await?;
    get_users(conn).await
}

#[delete("/api/users/<id>")]
pub async fn delete_user(conn: &State<Client>, id: i32) -> Result<Status, Custom<String>> {
    execute_query(conn, "DELETE FROM users WHERE id = $1", &[&id]).await?;
    Ok(Status::NoContent)
}

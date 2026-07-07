use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use rocket::request::FromRequest;
use rocket::request::Outcome;
use rocket::{Request, State};
use std::ops::{Deref, DerefMut};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_db_pool() -> DbPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}

pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DbConn {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for DbConn {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let pool = request.guard::<&State<DbPool>>().await;

        match pool {
            Outcome::Success(pool) => match pool.get() {
                Ok(conn) => Outcome::Success(DbConn(conn)),
                Err(_) => Outcome::Error((rocket::http::Status::ServiceUnavailable, ())),
            },
            Outcome::Error(e) => Outcome::Error(e),
            Outcome::Forward(f) => Outcome::Forward(f),
        }
    }
}

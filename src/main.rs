use crate::repositories::RustaceanRepository;
use rocket::get;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;
use rocket_db_pools::{Connection, Database};

mod models;
mod repositories;
mod schema;

#[derive(Database)]
#[database("postgres")]
struct DbConn(rocket_db_pools::diesel::PgPool);

#[get("/rustaceans")]
async fn get_rustaceans(mut db: Connection<DbConn>) -> Result<Value, Custom<Value>> {
    RustaceanRepository::find_multiple(&mut db, 100)
        .await
        .map(|rustaceans| json!(rustaceans))
        .map_err(|e| {
            eprintln!("Failed to fetch rustaceans: {:?}", e);
            Custom(
                Status::InternalServerError,
                json!("Error fetching rustaceans. See server logs for details."),
            )
        })
}
#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", rocket::routes![get_rustaceans])
        .attach(DbConn::init())
        .launch()
        .await;
}

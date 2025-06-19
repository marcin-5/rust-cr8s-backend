use crate::repositories::RustaceanRepository;
use crate::DbConn;
use rocket::get;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;
use rocket_db_pools::Connection;

#[get("/rustaceans")]
pub async fn get_rustaceans(mut db: Connection<DbConn>) -> Result<Value, Custom<Value>> {
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

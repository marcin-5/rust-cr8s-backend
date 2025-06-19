use crate::models::{NewRustacean, UpdateRustacean};
use crate::repositories::RustaceanRepository;
use crate::responses::handle_db_error;
use crate::DbConn;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::response::status::NoContent;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::{Json, Value};
use rocket::{delete, get, post, put};
use rocket_db_pools::Connection;

#[get("/rustaceans")]
pub async fn get_rustaceans(mut db: Connection<DbConn>) -> Result<Value, Custom<Value>> {
    RustaceanRepository::find_multiple(&mut db, 100)
        .await
        .map(|rustaceans| json!(rustaceans))
        .map_err(|e| {
            handle_db_error(
                e,
                "Failed to fetch rustaceans".to_string(),
                "fetching rustaceans",
            )
        })
}

#[get("/rustaceans/<id>")]
pub async fn view_rustacean(mut db: Connection<DbConn>, id: i32) -> Result<Value, Custom<Value>> {
    RustaceanRepository::find(&mut db, id)
        .await
        .map(|rustacean| json!(rustacean))
        .map_err(|e| {
            handle_db_error(
                e,
                format!("Failed to fetch rustacean with id {}", id),
                "fetching rustacean",
            )
        })
}

#[post("/rustaceans", format = "json", data = "<new_rustacean>")]
pub async fn create_rustacean(
    mut db: Connection<DbConn>,
    new_rustacean: Json<NewRustacean>,
) -> Result<Custom<Value>, Custom<Value>> {
    RustaceanRepository::create(&mut db, new_rustacean.into_inner())
        .await
        .map(|rustacean| Custom(Status::Created, json!(rustacean)))
        .map_err(|e| {
            handle_db_error(
                e,
                "Failed to create rustacean".to_string(),
                "creating rustacean",
            )
        })
}

#[put("/rustaceans/<id>", format = "json", data = "<rustacean>")]
pub async fn update_rustacean(
    mut db: Connection<DbConn>,
    id: i32,
    rustacean: Json<UpdateRustacean>,
) -> Result<Value, Custom<Value>> {
    RustaceanRepository::update(&mut db, id, rustacean.into_inner())
        .await
        .map(|rustacean| json!(rustacean))
        .map_err(|e| {
            handle_db_error(
                e,
                format!("Failed to update rustacean with id {}", id),
                "updating rustacean",
            )
        })
}

#[delete("/rustaceans/<id>")]
pub async fn delete_rustacean(
    mut db: Connection<DbConn>,
    id: i32,
) -> Result<NoContent, Custom<Value>> {
    RustaceanRepository::delete(&mut db, id)
        .await
        .map(|_| NoContent)
        .map_err(|e| {
            handle_db_error(
                e,
                format!("Failed to delete rustacean with id {}", id),
                "deleting rustacean",
            )
        })
}

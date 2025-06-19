use crate::models::{NewCrate, UpdateCrate};
use crate::repositories::CrateRepository;
use crate::responses::handle_db_error;
use crate::DbConn;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::response::status::NoContent;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::{Json, Value};
use rocket::{delete, get, post, put};
use rocket_db_pools::Connection;

#[get("/crates")]
pub async fn get_crates(mut db: Connection<DbConn>) -> Result<Value, Custom<Value>> {
    CrateRepository::find_multiple(&mut db, 100)
        .await
        .map(|crates| json!(crates))
        .map_err(|e| handle_db_error(e, "Failed to fetch crates".to_string(), "fetching crates"))
}

#[get("/crates/<id>")]
pub async fn view_crate(mut db: Connection<DbConn>, id: i32) -> Result<Value, Custom<Value>> {
    CrateRepository::find(&mut db, id)
        .await
        .map(|a_crate| json!(a_crate))
        .map_err(|e| {
            handle_db_error(
                e,
                format!("Failed to fetch crate with id {}", id),
                "fetching crate",
            )
        })
}

#[post("/crates", format = "json", data = "<new_crate>")]
pub async fn create_crate(
    mut db: Connection<DbConn>,
    new_crate: Json<NewCrate>,
) -> Result<Custom<Value>, Custom<Value>> {
    CrateRepository::create(&mut db, new_crate.into_inner())
        .await
        .map(|a_crate| Custom(Status::Created, json!(a_crate)))
        .map_err(|e| handle_db_error(e, "Failed to create crate".to_string(), "creating crate"))
}

#[put("/crates/<id>", format = "json", data = "<a_crate>")]
pub async fn update_crate(
    mut db: Connection<DbConn>,
    id: i32,
    a_crate: Json<UpdateCrate>,
) -> Result<Value, Custom<Value>> {
    CrateRepository::update(&mut db, id, a_crate.into_inner())
        .await
        .map(|a_crate| json!(a_crate))
        .map_err(|e| {
            handle_db_error(
                e,
                format!("Failed to update crate with id {}", id),
                "updating crate",
            )
        })
}

#[delete("/crates/<id>")]
pub async fn delete_crate(mut db: Connection<DbConn>, id: i32) -> Result<NoContent, Custom<Value>> {
    CrateRepository::delete(&mut db, id)
        .await
        .map(|_| NoContent)
        .map_err(|e| {
            handle_db_error(
                e,
                format!("Failed to delete crate with id {}", id),
                "deleting crate",
            )
        })
}

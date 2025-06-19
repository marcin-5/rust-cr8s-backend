use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::{serde_json::json, Value};
use std::fmt::Debug;

pub fn handle_db_error<E: Debug>(
    e: E,
    log_context: String,
    response_context: String,
) -> Custom<Value> {
    eprintln!("{}: {:?}", log_context, e);
    Custom(
        Status::InternalServerError,
        json!({ "error": format!(
            "Error {}. See server logs for details.",
            response_context
        )}),
    )
}

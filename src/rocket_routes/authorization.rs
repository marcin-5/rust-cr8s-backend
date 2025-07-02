use crate::models::User;
use crate::repositories::UserRepository;
use crate::rocket_routes::DbConn;
use argon2::{
    password_hash::{PasswordHash, PasswordVerifier},
    Argon2,
};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::{serde_json::json, Json, Value};
use rocket_db_pools::Connection;

#[derive(serde::Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

type HandlerResult<T> = Result<T, Custom<Value>>;

#[rocket::post("/login", format = "json", data = "<credentials>")]
pub async fn login(
    mut db: Connection<DbConn>,
    credentials: Json<Credentials>,
) -> HandlerResult<Value> {
    // Attempt to find the user by username
    let user = match UserRepository::find_by_username(&mut db, &credentials.username).await {
        Ok(user) => user,
        // If user is not found, return 401 Unauthorized to prevent username enumeration
        Err(diesel::result::Error::NotFound) => {
            return Err(Custom(
                Status::Unauthorized,
                json!({ "error": "Invalid credentials" }),
            ));
        }
        // Handle other potential database errors
        Err(e) => {
            eprintln!("Database error during login: {:?}", e);
            return Err(Custom(
                Status::InternalServerError,
                json!({ "error": "An internal server error occurred." }),
            ));
        }
    };

    // The `password` field should store the Argon2 password hash.
    let parsed_hash = match PasswordHash::new(&user.password) {
        Ok(hash) => hash,
        Err(e) => {
            eprintln!(
                "Could not parse password hash for user {}: {:?}",
                user.username, e
            );
            return Err(Custom(
                Status::InternalServerError,
                json!({ "error": "An internal server error occurred during login." }),
            ));
        }
    };

    // Verify the provided password against the stored hash
    if Argon2::default()
        .verify_password(credentials.password.as_bytes(), &parsed_hash)
        .is_ok()
    {
        Ok(json!({ "status": "success", "message": "Login successful" }))
    } else {
        // Passwords do not match.
        Err(Custom(
            Status::Unauthorized,
            json!({ "status": "unauthorized", "error": "Invalid credentials" }),
        ))
    }
}

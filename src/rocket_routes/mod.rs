use crate::models::{RoleCode, User};
use crate::repositories::{RoleRepository, UserRepository};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::response::status::Custom;
use rocket::serde::json::serde_json::json;
use rocket::serde::json::Value;
use rocket::Request;
use rocket_db_pools::deadpool_redis::redis::AsyncCommands;
use rocket_db_pools::Connection;
use std::error::Error;

pub mod authorization;
pub mod crates;
pub mod rustaceans;

#[derive(rocket_db_pools::Database)]
#[database("postgres")]
pub struct DbConn(rocket_db_pools::diesel::PgPool);

#[derive(rocket_db_pools::Database)]
#[database("redis")]
pub struct CacheConn(rocket_db_pools::deadpool_redis::Pool);

pub fn server_error(e: Box<dyn Error>) -> Custom<Value> {
    rocket::error!("{}", e);
    Custom(Status::InternalServerError, json!("Error"))
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Authorization: Bearer SESSION_ID_128_CHARACTERS_LONG
        let session_header = req
            .headers()
            .get_one("Authorization")
            .map(|v| v.split_whitespace().collect::<Vec<_>>())
            .filter(|v| v.len() == 2 && v[0] == "Bearer");

        if let Some(header_value) = session_header {
            let Outcome::Success(mut cache) = req.guard::<Connection<CacheConn>>().await else {
                rocket::error!("Failed to get Redis connection from pool");
                return Outcome::Error((Status::InternalServerError, ()));
            };
            let Outcome::Success(mut db) = req.guard::<Connection<DbConn>>().await else {
                rocket::error!("Failed to get Postgres connection from pool");
                return Outcome::Error((Status::InternalServerError, ()));
            };

            let key = format!("sessions/{}", header_value[1]);
            let result: Result<i32, _> = cache.get(key).await;

            if let Ok(user_id) = result {
                if let Ok(user) = UserRepository::find(&mut db, user_id).await {
                    return Outcome::Success(user);
                }
            }
        }

        Outcome::Error((Status::Unauthorized, ()))
    }
}

pub struct EditorUser(User);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for EditorUser {
    type Error = ();
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let user = match req.guard::<User>().await {
            Outcome::Success(user) => user,
            Outcome::Error(e) => return Outcome::Error(e),
            Outcome::Forward(s) => return Outcome::Forward(s),
        };

        let mut db = match req.guard::<Connection<DbConn>>().await {
            Outcome::Success(db) => db,
            _ => {
                // Catches Error and Forward
                rocket::error!("Failed to retrieve database connection from pool.");
                return Outcome::Error((Status::InternalServerError, ()));
            }
        };

        match RoleRepository::find_by_user(&mut db, &user).await {
            Ok(roles) => {
                let has_permission = roles
                    .iter()
                    .any(|r| matches!(r.code, RoleCode::Admin | RoleCode::Editor));

                if has_permission {
                    Outcome::Success(EditorUser(user))
                } else {
                    // User is authenticated but doesn't have the required role.
                    Outcome::Error((Status::Forbidden, ()))
                }
            }
            Err(e) => {
                rocket::error!(
                    "Role repository lookup failed for user {}: {:?}",
                    user.id,
                    e
                );
                Outcome::Error((Status::InternalServerError, ()))
            }
        }
    }
}

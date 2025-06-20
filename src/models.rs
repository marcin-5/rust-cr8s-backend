use crate::schema::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize)]
#[diesel(table_name = rustaceans)]
pub struct Rustacean {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = rustaceans)]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}

#[derive(Queryable, Serialize)]
#[diesel(table_name = crates)]
pub struct Crate {
    pub id: i32,
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crates)]
pub struct NewCrate {
    pub rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
}

#[derive(AsChangeset, Debug, Deserialize)]
#[diesel(table_name = rustaceans)]
pub struct UpdateRustacean {
    pub name: Option<String>,
    pub email: Option<String>,
}

#[derive(AsChangeset, Debug, Default, Deserialize)]
#[diesel(table_name = crates)]
pub struct UpdateCrate {
    pub rustacean_id: Option<i32>,
    pub name: Option<String>,
    pub code: Option<String>,
    pub version: Option<String>,
    pub description: Option<Option<String>>,
}

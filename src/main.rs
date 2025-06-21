use rocket_db_pools::Database;

pub mod macros;
pub mod models;
pub mod repositories;
pub mod responses;
pub mod rocket_routes;
pub mod schema;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/rustaceans", rocket_routes::rustaceans::routes())
        .mount("/crates", rocket_routes::crates::routes())
        .attach(rocket_routes::DbConn::init())
        .launch()
        .await;
}

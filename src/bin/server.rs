extern crate backend;
use rocket_db_pools::Database;

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/rustaceans", backend::rocket_routes::rustaceans::routes())
        .mount("/crates", backend::rocket_routes::crates::routes())
        .attach(backend::rocket_routes::DbConn::init())
        .launch()
        .await;
}

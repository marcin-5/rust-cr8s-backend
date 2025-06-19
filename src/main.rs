use rocket_db_pools::Database;

mod models;
mod repositories;
mod responses;
mod rocket_routes;
mod schema;

#[derive(Database)]
#[database("postgres")]
struct DbConn(rocket_db_pools::diesel::PgPool);

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            rocket::routes![rocket_routes::rustaceans::get_rustaceans],
        )
        .attach(DbConn::init())
        .launch()
        .await;
}

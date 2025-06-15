use rocket::get;

mod models;
mod repositories;
mod schema;

#[get("/rustaceans")]
fn get_rustaceans() {
    //
}
#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount("/", rocket::routes![get_rustaceans])
        .launch()
        .await;
}

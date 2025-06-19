use crate::crud_handlers;
use crate::models::{NewRustacean, UpdateRustacean};
use crate::repositories::RustaceanRepository;

crud_handlers!(
    "rustacean",
    "rustaceans",
    RustaceanRepository,
    NewRustacean,
    UpdateRustacean,
    get_rustaceans,
    view_rustacean,
    create_rustacean,
    update_rustacean,
    delete_rustacean
);

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![
        get_rustaceans,
        view_rustacean,
        create_rustacean,
        update_rustacean,
        delete_rustacean
    ]
}

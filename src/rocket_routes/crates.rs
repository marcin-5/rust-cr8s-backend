use crate::crud_handlers;
use crate::models::{NewCrate, UpdateCrate};
use crate::repositories::CrateRepository;

crud_handlers!(
    "crate",
    "crates",
    CrateRepository,
    NewCrate,
    UpdateCrate,
    get_crates,
    view_crate,
    create_crate,
    update_crate,
    delete_crate
);

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![
        get_crates,
        view_crate,
        create_crate,
        update_crate,
        delete_crate
    ]
}

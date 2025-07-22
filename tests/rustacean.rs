use rocket::serde::json::serde_json::json;

mod common;
use common::{create_test_rustacean, create_test_rustacean_with_data, RUSTACEANS_URL};

#[test]
fn test_get_rustaceans() {
    let client = common::get_client_with_logged_in_admin();
    let rustacean1 = create_test_rustacean(&client);
    let rustacean2 = create_test_rustacean(&client);

    let response = client.get(RUSTACEANS_URL).send().unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let json: rocket::serde::json::Value = response.json().unwrap();
    assert!(json.as_array().unwrap().contains(&*rustacean1));
    assert!(json.as_array().unwrap().contains(&*rustacean2));
}

#[test]
fn test_create_rustacean() {
    let client = common::get_client_with_logged_in_admin();
    let name = "John Smith";
    let email = "john@smith.com";

    let response = create_test_rustacean_with_data(&client, name, email);
    assert_eq!(response.status(), reqwest::StatusCode::CREATED);

    let rustacean_value: rocket::serde::json::Value = response.json().unwrap();
    let _rustacean_guard = common::RustaceanGuard {
        client: &client,
        value: rustacean_value.clone(),
    };

    assert_eq!(
        rustacean_value,
        json!({
            "id": rustacean_value["id"],
            "name": name,
            "email": email,
            "created_at": rustacean_value["created_at"],
        })
    );
}

#[test]
fn test_create_rustacean_unprivileged() {
    let client = common::get_client_with_logged_in_viewer();
    let name = "John Smith";
    let email = "john@smith.com";

    let response = create_test_rustacean_with_data(&client, name, email);
    assert_eq!(response.status(), reqwest::StatusCode::FORBIDDEN);
}

#[test]
fn test_view_rustacean() {
    let client = common::get_client_with_logged_in_admin();
    let rustacean = create_test_rustacean(&client);
    let rustacean_id = rustacean["id"].clone();

    let response = client
        .get(format!("{}/{}", RUSTACEANS_URL, rustacean_id))
        .send()
        .unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let fetched_rustacean: rocket::serde::json::Value = response.json().unwrap();
    assert_eq!(*rustacean, fetched_rustacean);

    let response = client
        .get(format!("{}/{}", RUSTACEANS_URL, 9999))
        .send()
        .unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::NOT_FOUND);
}

#[test]
fn test_update_rustacean() {
    let client = common::get_client_with_logged_in_admin();
    let rustacean = create_test_rustacean(&client);
    let rustacean_id = rustacean["id"].clone();

    let response = client
        .put(format!("{}/{}", RUSTACEANS_URL, rustacean_id))
        .json(&json!({
            "name": "Jane Doe",
            "email": "jane@doe.com"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let updated_rustacean: rocket::serde::json::Value = response.json().unwrap();
    assert_eq!(
        updated_rustacean,
        json!({
            "id": rustacean["id"],
            "name": "Jane Doe",
            "email": "jane@doe.com",
            "created_at": rustacean["created_at"],
        })
    );
}

#[test]
fn test_delete_rustacean() {
    let client = common::get_client_with_logged_in_admin();
    let rustacean = create_test_rustacean(&client);
    let rustacean_id = rustacean["id"].clone();

    let response = client
        .delete(format!("{}/{}", RUSTACEANS_URL, rustacean_id))
        .send()
        .unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::NO_CONTENT);
}

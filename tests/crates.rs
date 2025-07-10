use rocket::serde::json::{serde_json::json, Value};

mod common;
use common::{create_test_crate, create_test_crate_with_data, create_test_rustacean, CRATES_URL};

#[test]
fn test_get_crates() {
    let client = common::get_client_with_logged_in_admin();
    let rustacean = create_test_rustacean(&client);
    let rustacean_id = rustacean["id"].as_i64().unwrap() as i32;
    let crate1 = create_test_crate(&client, rustacean_id);
    let crate2 = create_test_crate(&client, rustacean_id);

    let response = client.get(CRATES_URL).send().unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let json: Value = response.json().unwrap();
    assert!(json.as_array().unwrap().contains(&*crate1));
    assert!(json.as_array().unwrap().contains(&*crate2));
}

#[test]
fn test_create_crate() {
    let client = common::get_client_with_logged_in_admin();
    let rustacean = create_test_rustacean(&client);
    let rustacean_id = rustacean["id"].as_i64().unwrap() as i32;
    let (name, code, version) = ("diesel", "DIESEL", "2.0");

    let a_crate = create_test_crate_with_data(&client, rustacean_id, name, code, version);

    assert_eq!(
        *a_crate,
        json!({
            "id": a_crate["id"],
            "rustacean_id": rustacean_id,
            "name": name,
            "code": code,
            "version": version,
            "description": null,
            "created_at": a_crate["created_at"],
        })
    );

    // Test creating crate with non-existing rustacean
    let (name, code, version) = ("another-crate", "ANO", "0.1");
    let response = client
        .post(CRATES_URL)
        .json(&json!({
            "rustacean_id": 9999,
            "name": name,
            "code": code,
            "version": version,
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::NOT_FOUND);
}

#[test]
fn test_view_crate() {
    let client = common::get_client_with_logged_in_admin();
    let rustacean = create_test_rustacean(&client);
    let rustacean_id = rustacean["id"].as_i64().unwrap() as i32;
    let a_crate = create_test_crate(&client, rustacean_id);
    let crate_id = a_crate["id"].clone();

    let response = client
        .get(format!("{}/{}", CRATES_URL, crate_id))
        .send()
        .unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let fetched_crate: Value = response.json().unwrap();
    assert_eq!(*a_crate, fetched_crate);

    let response = client
        .get(format!("{}/{}", CRATES_URL, 9999))
        .send()
        .unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::NOT_FOUND);
}

#[test]
fn test_update_crate() {
    let client = common::get_client_with_logged_in_admin();
    let rustacean = create_test_rustacean(&client);
    let rustacean_id = rustacean["id"].as_i64().unwrap() as i32;
    let a_crate = create_test_crate(&client, rustacean_id);
    let crate_id = a_crate["id"].clone();

    let response = client
        .put(format!("{}/{}", CRATES_URL, crate_id))
        .json(&json!({ "description": "An ORM for Rust" }))
        .send()
        .unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let updated_crate: Value = response.json().unwrap();
    assert_eq!(updated_crate["description"], json!("An ORM for Rust"));

    // Test changing crate owner
    let another_rustacean = create_test_rustacean(&client);
    let another_rustacean_id = another_rustacean["id"].as_i64().unwrap() as i32;
    let response = client
        .put(format!("{}/{}", CRATES_URL, crate_id))
        .json(&json!({
            "rustacean_id": another_rustacean_id,
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let updated_crate: Value = response.json().unwrap();
    assert_eq!(updated_crate["rustacean_id"], json!(another_rustacean_id));

    // Test changing crate owner to non-existing rustacean
    let response = client
        .put(format!("{}/{}", CRATES_URL, crate_id))
        .json(&json!({
            "rustacean_id": 9999,
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::NOT_FOUND);
}

#[test]
fn test_delete_crate() {
    let client = common::get_client_with_logged_in_admin();
    let rustacean = create_test_rustacean(&client);
    let rustacean_id = rustacean["id"].as_i64().unwrap() as i32;
    let a_crate = create_test_crate(&client, rustacean_id);
    let crate_id = a_crate["id"].clone();

    let response = client
        .delete(format!("{}/{}", CRATES_URL, crate_id))
        .send()
        .unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::NO_CONTENT);
}

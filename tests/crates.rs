use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::{serde_json::json, Value};
use std::ops::Deref;

const CRATES_URL: &str = "http://127.0.0.1:8000/crates";
const RUSTACEANS_URL: &str = "http://127.0.0.1:8000/rustaceans";

/// A guard that deletes the test crate when it goes out of scope.
struct CrateGuard<'a> {
    client: &'a Client,
    value: Value,
}

impl<'a> Deref for CrateGuard<'a> {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<'a> Drop for CrateGuard<'a> {
    fn drop(&mut self) {
        // Fire-and-forget deletion for cleanup.
        let _ = self
            .client
            .delete(format!("{}/{}", CRATES_URL, self.value["id"]))
            .send();
    }
}

/// Helper to create a rustacean and return its ID. Necessary for creating crates.
fn create_test_rustacean(client: &Client) -> i32 {
    let response = client
        .post(RUSTACEANS_URL)
        .json(&json!({
            "name": "Test Rustacean",
            "email": "test@rustacean.com"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let rustacean: Value = response.json().unwrap();
    rustacean["id"].as_i64().unwrap() as i32
}

/// Helper to create a crate with specific data. Returns a cleanup guard.
fn create_test_crate_with_data<'a>(
    client: &'a Client,
    rustacean_id: i32,
    name: &str,
    code: &str,
    version: &str,
) -> CrateGuard<'a> {
    let response = client
        .post(CRATES_URL)
        .json(&json!({
            "rustacean_id": rustacean_id,
            "name": name,
            "code": code,
            "version": version
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let value = response.json().unwrap();
    CrateGuard { client, value }
}

/// Helper to create a crate with default data.
fn create_test_crate<'a>(client: &'a Client, rustacean_id: i32) -> CrateGuard<'a> {
    create_test_crate_with_data(client, rustacean_id, "serde", "SERDE", "1.0")
}

#[test]
fn test_get_crates() {
    let client = Client::new();
    let rustacean_id = create_test_rustacean(&client);
    let crate1 = create_test_crate(&client, rustacean_id);
    let crate2 = create_test_crate(&client, rustacean_id);

    let response = client.get(CRATES_URL).send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let json: Value = response.json().unwrap();
    assert!(json.as_array().unwrap().contains(&*crate1));
    assert!(json.as_array().unwrap().contains(&*crate2));
}

#[test]
fn test_create_crate() {
    let client = Client::new();
    let rustacean_id = create_test_rustacean(&client);
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
}

#[test]
fn test_view_crate() {
    let client = Client::new();
    let rustacean_id = create_test_rustacean(&client);
    let a_crate = create_test_crate(&client, rustacean_id);
    let crate_id = a_crate["id"].clone();

    let response = client
        .get(format!("{}/{}", CRATES_URL, crate_id))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let fetched_crate: Value = response.json().unwrap();
    assert_eq!(*a_crate, fetched_crate);
}

#[test]
fn test_update_crate() {
    let client = Client::new();
    let rustacean_id = create_test_rustacean(&client);
    let a_crate = create_test_crate(&client, rustacean_id);
    let crate_id = a_crate["id"].clone();

    let response = client
        .put(format!("{}/{}", CRATES_URL, crate_id))
        .json(&json!({ "description": "An ORM for Rust" }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let updated_crate: Value = response.json().unwrap();
    assert_eq!(updated_crate["description"], json!("An ORM for Rust"));
}

#[test]
fn test_delete_crate() {
    let client = Client::new();
    let rustacean_id = create_test_rustacean(&client);
    let a_crate = create_test_crate(&client, rustacean_id);
    let crate_id = a_crate["id"].clone();

    let response = client
        .delete(format!("{}/{}", CRATES_URL, crate_id))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

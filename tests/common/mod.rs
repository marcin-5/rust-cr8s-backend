#![allow(dead_code)]

use reqwest::{blocking::Client, blocking::ClientBuilder, header, StatusCode};
use rocket::serde::json::{serde_json::json, Value};
use std::ops::Deref;
use std::process::Command;

// --- Constants ---
pub const SERVER_URL: &str = "http://127.0.0.1:8000";
pub const RUSTACEANS_URL: &str = "http://127.0.0.1:8000/rustaceans";
pub const CRATES_URL: &str = "http://127.0.0.1:8000/crates";

// --- RAII Guards for automatic cleanup ---

/// Guard for cleaning up test rustaceans.
pub struct RustaceanGuard<'a> {
    pub client: &'a Client,
    pub value: Value,
}

impl<'a> Deref for RustaceanGuard<'a> {
    type Target = Value;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<'a> Drop for RustaceanGuard<'a> {
    fn drop(&mut self) {
        let _ = self
            .client
            .delete(format!("{}/{}", RUSTACEANS_URL, self.value["id"]))
            .send();
    }
}

/// Guard for cleaning up test crates.
pub struct CrateGuard<'a> {
    pub client: &'a Client,
    pub value: Value,
}

impl<'a> Deref for CrateGuard<'a> {
    type Target = Value;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<'a> Drop for CrateGuard<'a> {
    fn drop(&mut self) {
        let _ = self
            .client
            .delete(format!("{}/{}", CRATES_URL, self.value["id"]))
            .send();
    }
}

// --- Helper Functions ---

/// Creates a rustacean with specific data.
pub fn create_test_rustacean_with_data<'a>(
    client: &'a Client,
    name: &str,
    email: &str,
) -> RustaceanGuard<'a> {
    let response = client
        .post(RUSTACEANS_URL)
        .json(&json!({ "name": name, "email": email }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let value = response.json().unwrap();
    RustaceanGuard { client, value }
}

/// Creates a rustacean with default data.
pub fn create_test_rustacean(client: &Client) -> RustaceanGuard {
    create_test_rustacean_with_data(client, "John Doe", "john@doe.com")
}

/// Creates a crate with specific data.
pub fn create_test_crate_with_data<'a>(
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

/// Creates a crate with default data.
pub fn create_test_crate(client: &Client, rustacean_id: i32) -> CrateGuard<'_> {
    create_test_crate_with_data(client, rustacean_id, "serde", "SERDE", "1.0")
}

/// Creates and returns a new `reqwest::Client` instance with the default headers
/// configured for an authenticated admin user.
pub fn get_client_with_logged_in_admin() -> Client {
    let create_user_args = [
        "run",
        "--bin",
        "cli",
        "users",
        "create",
        "test_admin",
        "1234",
        "admin",
    ];
    let _ = Command::new("cargo")
        .args(create_user_args)
        .output()
        .unwrap();

    let client = Client::new();
    let response = client
        .post(format!("{}/login", SERVER_URL))
        .json(&json!({
            "username": "test_admin",
            "password": "1234",
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.get("token").is_some());
    let header_value = format!("Bearer {}", json["token"].as_str().unwrap());

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&header_value).unwrap(),
    );

    ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap()
}

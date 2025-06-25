use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::{serde_json::json, Value};
use std::ops::Deref;

// --- Constants ---
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
pub fn create_test_crate<'a>(client: &'a Client, rustacean_id: i32) -> CrateGuard<'a> {
    create_test_crate_with_data(client, rustacean_id, "serde", "SERDE", "1.0")
}

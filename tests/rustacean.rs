use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::{serde_json::json, Value};
use std::ops::Deref;

const RUSTACEANS_URL: &str = "http://127.0.0.1:8000/rustaceans";

/// A guard that deletes the test rustacean when it goes out of scope.
/// This uses the RAII (Resource Acquisition Is Initialization) pattern
/// to ensure cleanup happens automatically.
struct RustaceanGuard<'a> {
    client: &'a Client,
    value: Value,
}

impl<'a> Deref for RustaceanGuard<'a> {
    type Target = Value;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<'a> Drop for RustaceanGuard<'a> {
    fn drop(&mut self) {
        // Fire-and-forget deletion for cleanup. We don't care about the result,
        // as the test that focuses on deletion will have its own assertions.
        let _ = self
            .client
            .delete(format!("{}/{}", RUSTACEANS_URL, self.value["id"]))
            .send();
    }
}

/// Helper function to create a rustacean with specific data for testing.
/// It returns a guard that will automatically delete the rustacean upon being dropped.
fn create_test_rustacean_with_data<'a>(
    client: &'a Client,
    name: &str,
    email: &str,
) -> RustaceanGuard<'a> {
    let response = client
        .post(RUSTACEANS_URL)
        .json(&json!({
            "name": name,
            "email": email
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let value = response.json().unwrap();
    RustaceanGuard { client, value }
}

/// Helper function to create a rustacean with default data for testing purposes.
fn create_test_rustacean(client: &Client) -> RustaceanGuard {
    create_test_rustacean_with_data(client, "John Doe", "john@doe.com")
}

#[test]
fn test_get_rustaceans() {
    let client = Client::new();
    let rustacean1 = create_test_rustacean(&client);
    let rustacean2 = create_test_rustacean(&client);

    let response = client.get(RUSTACEANS_URL).send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let json: Value = response.json().unwrap();
    assert!(json.as_array().unwrap().contains(&*rustacean1));
    assert!(json.as_array().unwrap().contains(&*rustacean2));
}

#[test]
fn test_create_rustacean() {
    let client = Client::new();
    let name = "John Smith";
    let email = "john@smith.com";

    // Use the flexible helper to create a specific rustacean.
    let rustacean = create_test_rustacean_with_data(&client, name, email);

    // Assert that the response contains the specific data we sent.
    assert_eq!(
        *rustacean,
        json!({
            "id": rustacean["id"],
            "name": name,
            "email": email,
            "created_at": rustacean["created_at"],
        })
    );
}

#[test]
fn test_view_rustacean() {
    let client = Client::new();
    let rustacean = create_test_rustacean(&client);
    let rustacean_id = rustacean["id"].clone();

    let response = client
        .get(format!("{}/{}", RUSTACEANS_URL, rustacean_id))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let fetched_rustacean: Value = response.json().unwrap();
    assert_eq!(*rustacean, fetched_rustacean);
}

#[test]
fn test_update_rustacean() {
    let client = Client::new();
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
    assert_eq!(response.status(), StatusCode::OK);

    let updated_rustacean: Value = response.json().unwrap();
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
    let client = Client::new();
    let rustacean = create_test_rustacean(&client);
    let rustacean_id = rustacean["id"].clone();

    let response = client
        .delete(format!("{}/{}", RUSTACEANS_URL, rustacean_id))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

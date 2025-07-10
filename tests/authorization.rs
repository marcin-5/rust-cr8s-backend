use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::serde_json::json;

pub mod common;

#[test]
fn test_login() {
    // Setup
    common::create_test_admin_user();
    let client = Client::new();

    // Test for successful login
    let token = common::get_admin_token(&client);
    assert_eq!(token.len(), 128);

    // Test for failed login
    let response = client
        .post(format!("{}/login", common::SERVER_URL))
        .json(&json!({
            "username": "test_admin",
            "password": "12345", // Wrong password
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

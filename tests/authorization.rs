use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::serde_json::json;

pub mod common;

#[test]
fn test_login() {
    // Setup
    common::create_test_admin_user();
    let client = Client::new();

    // Test for successful login
    let token = common::get_user_token(&client, common::TEST_ADMIN_USERNAME);
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

#[test]
fn test_unauthorized_access_to_private_routes() {
    let client = Client::new();

    // A list of private routes to test. We check different methods (GET, POST, PUT, DELETE).
    let private_routes = vec![
        ("GET", common::RUSTACEANS_URL.to_string()),
        ("POST", common::RUSTACEANS_URL.to_string()),
        ("GET", format!("{}/1", common::RUSTACEANS_URL)),
        ("PUT", format!("{}/1", common::RUSTACEANS_URL)),
        ("DELETE", format!("{}/1", common::RUSTACEANS_URL)),
        ("GET", common::CRATES_URL.to_string()),
        ("POST", common::CRATES_URL.to_string()),
        ("GET", format!("{}/1", common::CRATES_URL)),
        ("PUT", format!("{}/1", common::CRATES_URL)),
        ("DELETE", format!("{}/1", common::CRATES_URL)),
    ];

    for (method, url) in private_routes {
        let mut request = match method {
            "GET" => client.get(&url),
            "POST" => client.post(&url),
            "PUT" => client.put(&url),
            "DELETE" => client.delete(&url),
            _ => panic!("Unsupported HTTP method in test"),
        };

        // Add a dummy body for methods that require it.
        if method == "POST" || method == "PUT" {
            request = request.json(&json!({}));
        }

        let response = request.send().unwrap();
        assert_eq!(
            response.status(),
            StatusCode::UNAUTHORIZED,
            "Expected UNAUTHORIZED for {} {}",
            method,
            url
        );
    }
}

use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::{serde_json::json, Value};
use std::process::Command;

pub mod common;

#[test]
fn test_login() {
    // Setup
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
    let output = Command::new("cargo")
        .args(create_user_args)
        .output()
        .unwrap();

    println!("{:?}", output);

    let client = Client::new();

    // Test
    let response = client
        .post(format!("{}/login", common::SERVER_URL))
        .json(&json!({
            "username": "test_admin",
            "password": "1234",
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.get("token").is_some());
    assert_eq!(json["token"].as_str().unwrap().len(), 128);

    let response = client
        .post(format!("{}/login", common::SERVER_URL))
        .json(&json!({
            "username": "test_admin",
            "password": "12345",
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

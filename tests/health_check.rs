use std::{assert_eq};

fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address");

    let _ = tokio::spawn(server);
}
#[tokio::test]
async fn health_check_works() {
    //Start web server
    spawn_app();

    //Build a client and a health check request
    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    //Assert response
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

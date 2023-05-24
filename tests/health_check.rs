use std::{assert_eq, format, net::TcpListener};

fn spawn_app() -> String {
    //Get a random port from the OS
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();

    let server = zero2prod::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_works() {
    //Start web server
    let address = spawn_app();

    //Build a client and a health check request
    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request.");

    //Assert response
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let url = spawn_app();
    let client = reqwest::Client::new();

    let body = "name=jean%20jacques&email=jean.jacques%40gmail.com";
    let response = client
        .post(format!("{}/subscriptions", &url))
        .header("Content-type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_return_a_400_when_data_is_missing() {
    let url = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=jean%20jacques", "missing the email"),
        ("email=jean.jacques%40gmail.com", "missing the name"),
        ("", "missing both email and name"),
    ];

    for (invalid_body, invalid_reason) in test_cases {
        let response = client
            .post(format!("{}/subscriptions", &url))
            .header("Content-type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            invalid_reason
        );
    }
}

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

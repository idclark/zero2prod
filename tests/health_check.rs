use std::{fmt::format, net::TcpListener};

use zero2prod::run;

#[tokio::test]
async fn health_check_works() {
    // Get started
    let address = spawn_app();
    let client = reqwest::Client::new();

    // Act on it
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request. ");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to address");
    // get the port assigned by the OS
    let port = listener.local_addr().unwrap().port();
    let server = run(listener).expect("Failed to bind to address");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

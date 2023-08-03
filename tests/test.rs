use template::prelude::f;

fn spawn_app() -> String {
    // Need test server to run on free port to run tests in parallel
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let server = template::run(listener).expect("Failed to bind address");
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future
    // As the server's future does not resolve we throw the handler away
    let _ = tokio::spawn(server);
    f!("127.0.0.1:{}", listener.local_addr().unwrap().port())
}

#[tokio::test]
async fn health_check_works() {
    // --- Run server as a background task
    let addr = spawn_app().await.expect("Failed to spawn server.");

    // --- Perform HTTP request
    let client = reqwest::Client::new();
    let response = client
        .get(f!("http://{addr}/health"))        // Builder pattern for request
        .send()                                     // Construct request
        .await
        .expect("Failed to execute request.");

    // --- Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

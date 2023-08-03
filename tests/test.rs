fn spawn_app() {
    let server = template::run().expect("Failed to bind address");
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future
    let _ = tokio::spawn(server);
}

#[tokio::test]
async fn health_check_works() {
    // Runs server as a background task
    spawn_app().await.expect("Failed to spawn our app.");

    // Perform HTTP requests against our application.
    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

use reqwest::StatusCode;
use zero2prod::ADDRESS;

#[tokio::test]
async fn test_health_check() {
    // setup the server
    spawn_app();

    let url = format!("http://{ADDRESS}/health");
    let response = reqwest::get(url).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

// Spawns the application in the background
fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}

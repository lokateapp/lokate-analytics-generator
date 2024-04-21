mod spawn_app;
use spawn_app::spawn_app;

#[tokio::test]
async fn app_health_check() {
    let address = spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to get response");

    assert!(response.status().is_success());

    let response_text = response.text().await.expect("Failed to get payload");

    assert_eq!(response_text, "It works!");
}

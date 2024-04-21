mod spawn_app;
use spawn_app::spawn_app;
use sqlx::{PgConnection, Connection};
use lokate_visit_analytics::get_configuration;

#[tokio::test]
async fn app_health_check() {
    let address = spawn_app();

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

#[tokio::test]
async fn db_health_check() {
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    let _connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect postgres");
}

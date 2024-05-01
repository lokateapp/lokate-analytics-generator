use lokate_visit_analytics::{get_configuration, run};
use sqlx::PgPool;
use std::net::TcpListener;

pub async fn spawn_app() -> String {
    let configuration = get_configuration().expect("Failed to read configuration");
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // Retrieve the port assigned to app by the OS
    let port = listener.local_addr().unwrap().port();
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");
    let server = run(listener, connection_pool).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

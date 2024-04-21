use lokate_visit_analytics::{get_configuration, run};
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let configuration = get_configuration().expect("Failed to read configuration");
    let server_address = format!("127.0.0.1:{}", configuration.server_port);
    let listener = TcpListener::bind(server_address).expect("Failed to bind");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");
    run(listener, connection_pool)?.await
}

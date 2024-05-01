use lokate_analytics::{get_configuration, run};
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let configuration = get_configuration().expect("Failed to read configuration");
    println!("{configuration:?}");
    let server_address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(server_address).expect("Failed to bind");
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy(&configuration.database.connection_string())
        .expect("Failed to connect to Postgres");
    run(listener, connection_pool)?.await
}

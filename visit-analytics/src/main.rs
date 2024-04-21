use lokate_visit_analytics::{get_configuration, run};
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let address = format!("127.0.0.1:{}", configuration.server_port);
    let listener = TcpListener::bind(address).expect("Failed to bind");
    run(listener)?.await
}

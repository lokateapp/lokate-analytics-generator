use lokate_visit_analytics::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4444").expect("Failed to bind");
    run(listener)?.await
}

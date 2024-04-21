use crate::routes::{generate_analytics, generate_visit_sequences, health_check};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route(
                "/generate_visit_sequences",
                web::post().to(generate_visit_sequences),
            )
            .route(
                "/generate_visit_analytics",
                web::get().to(generate_analytics),
            )
    })
    .listen(listener)?
    .run();
    Ok(server)
}

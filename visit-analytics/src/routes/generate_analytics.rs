use actix_web::{web, Error, Responder};

#[derive(serde::Serialize)]
struct Analytic {
    name: &'static str,
}

pub async fn generate_analytics() -> Result<impl Responder, Error> {
    let analytics = vec![Analytic { name: "asdf" }];
    Ok(web::Json(analytics))
}

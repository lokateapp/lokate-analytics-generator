use actix_web::{web, Error, Responder};

#[derive(serde::Serialize)]
struct Analytic {
    name: &'static str,
}

pub async fn generate_visit_analytics() -> Result<impl Responder, Error> {
    log::info!("generate visit analytics endpoint is called");
    let analytics = vec![Analytic { name: "asdf" }];
    Ok(web::Json(analytics))
}

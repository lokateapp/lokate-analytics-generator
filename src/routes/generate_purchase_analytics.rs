use actix_web::{web, Error, Responder};

pub async fn generate_visit_analytics() -> Result<impl Responder, Error> {
    log::info!("generate purchase analytics endpoint is called");
    Ok(web::Json(""))
}

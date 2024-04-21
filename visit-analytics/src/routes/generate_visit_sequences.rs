use actix_web::HttpResponse;

pub async fn generate_visit_sequences() -> HttpResponse {
    HttpResponse::Ok().finish()
}

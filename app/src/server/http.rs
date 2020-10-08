use actix_web::HttpResponse;

pub fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

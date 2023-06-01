use actix_web::{http::header::LOCATION, HttpResponse};

pub fn redirect(to: &str) -> HttpResponse {
    HttpResponse::SeeOther().header(LOCATION, to).finish()
}

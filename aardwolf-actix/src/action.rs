use actix_web::{http::header::LOCATION, HttpResponse};

pub fn redirect(to: &str) -> HttpResponse {
    HttpResponse::SeeOther()
        .append_header((LOCATION, to))
        .finish()
}

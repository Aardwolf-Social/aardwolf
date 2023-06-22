use actix_web::{http::header::LOCATION, HttpResponse, ResponseError};
use failure::Fail;

#[derive(Clone, Debug, Fail)]
#[fail(display = "Could not render template")]
pub struct RenderError;

impl ResponseError for RenderError {
    fn error_response(&self) -> HttpResponse {
        redirect_error("/", None)
    }
}

pub fn redirect_error(to: &str, msg: Option<String>) -> HttpResponse {
    let msg = msg.map(|m| format!("?msg={}", m)).unwrap_or("".to_owned());
    let location = format!("{}{}", to, msg);

    HttpResponse::SeeOther()
        .append_header((LOCATION, location.as_str()))
        .finish()
}

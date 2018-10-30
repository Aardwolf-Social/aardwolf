use actix_web::{http::header::LOCATION, HttpResponse, ResponseError};

pub type RenderResult = Result<HttpResponse, RenderError>;

#[derive(Clone, Debug, Fail)]
#[fail(display = "Could not render template")]
pub struct RenderError;

impl ResponseError for RenderError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::SeeOther().header(LOCATION, "/").finish()
    }
}

#[derive(Debug, Fail)]
#[fail(display = "Redirect to {}", _0)]
pub struct RedirectError(String);

impl RedirectError {
    pub fn new(s: &str) -> Self {
        RedirectError(s.to_owned())
    }
}

impl ResponseError for RedirectError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::SeeOther()
            .header(LOCATION, self.0.as_str())
            .finish()
    }
}

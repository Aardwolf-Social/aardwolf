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
pub struct RedirectError(String, Option<String>);

impl RedirectError {
    pub fn new(s: &str, msg: Option<&str>) -> Self {
        RedirectError(s.to_owned(), msg.map(|m| m.to_owned()))
    }
}

impl ResponseError for RedirectError {
    fn error_response(&self) -> HttpResponse {
        let msg = self
            .1
            .as_ref()
            .map(|m| format!("?msg={}", m))
            .unwrap_or("".to_owned());
        let location = format!("{}{}", self.0, msg);

        println!("Redirecting to {}", location);

        HttpResponse::SeeOther()
            .header(LOCATION, location.as_str())
            .finish()
    }
}

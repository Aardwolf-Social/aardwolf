use aardwolf_types::error::{AardwolfFail, RedirectFail};
use actix_web::{http::header::LOCATION, HttpResponse, ResponseError};
use failure::Fail;
use serde::ser::{Serialize, Serializer};

use crate::AppConfig;

pub type RenderResult = Result<HttpResponse, RenderError>;

#[derive(Clone, Debug, Fail)]
#[fail(display = "Could not render template")]
pub struct RenderError;

impl ResponseError for RenderError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::SeeOther().header(LOCATION, "/").finish()
    }
}

pub fn redirect_error(to: &str, msg: Option<String>) -> HttpResponse {
    let msg = msg.map(|m| format!("?msg={}", m)).unwrap_or("".to_owned());
    let location = format!("{}{}", to, msg);

    HttpResponse::SeeOther()
        .header(LOCATION, location.as_str())
        .finish()
}

#[derive(Clone, Debug, Fail)]
#[fail(display = "{}", _1)]
pub struct ErrorWrapper<E>(AppConfig, pub E)
where
    E: AardwolfFail;

impl<E> Serialize for ErrorWrapper<E>
where
    E: AardwolfFail,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        E::serialize(&self.1, serializer)
    }
}

impl<E> ErrorWrapper<E>
where
    E: AardwolfFail,
{
    pub fn new(state: AppConfig, error: E) -> Self {
        ErrorWrapper(state, error)
    }
}

impl<E> AardwolfFail for ErrorWrapper<E> where E: AardwolfFail {}

impl<E> ResponseError for ErrorWrapper<E>
where
    Self: RedirectFail,
    E: AardwolfFail,
{
    fn error_response(&self) -> HttpResponse {
        HttpResponse::SeeOther()
            .header(LOCATION, self.redirect().path())
            .finish()
    }
}

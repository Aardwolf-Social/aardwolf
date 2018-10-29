use actix_web::{HttpResponse, ResponseError};

pub type RenderResult = Result<HttpResponse, RenderError>;

#[derive(Clone, Debug, Fail)]
#[fail(display = "Could not render template")]
pub struct RenderError;

impl ResponseError for RenderError {}

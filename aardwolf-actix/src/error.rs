use actix_web::ResponseError;

pub type RenderResult = Result<String, RenderError>;

#[derive(Clone, Debug, Fail)]
#[fail(display = "Could not render template")]
pub struct RenderError;

impl ResponseError for RenderError {}

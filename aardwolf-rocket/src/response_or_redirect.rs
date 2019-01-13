use rocket::{Response, response::Redirect};

#[derive(Responder)]
#[cfg_attr(feature = "cargo-clippy", allow(clippy::large_enum_variant))]
pub enum ResponseOrRedirect {
    Response(Response<'static>),
    Redirect(Redirect),
}

impl From<Response<'static>> for ResponseOrRedirect {
    fn from(r: Response<'static>) -> Self {
        ResponseOrRedirect::Response(r)
    }
}

impl From<Redirect> for ResponseOrRedirect {
    fn from(r: Redirect) -> Self {
        ResponseOrRedirect::Redirect(r)
    }
}


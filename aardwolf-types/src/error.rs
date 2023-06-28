use serde::ser::Serialize;
use std::error::Error;

pub enum ResponseKind {
    BadRequest,
    RequiresAuthentication,
    RequiresPermission,
    NotFound,
    InternalServerError,
}

pub trait AardwolfFail: Serialize + Error {}

#[derive(Clone, Debug)]
pub struct RedirectTo(String);

impl RedirectTo {
    pub fn new(s: String) -> Self {
        RedirectTo(s)
    }

    pub fn path(&self) -> &str {
        &self.0
    }
}

pub trait RedirectFail: AardwolfFail {
    fn redirect(&self) -> RedirectTo;
}

#[derive(Clone, Debug)]
pub struct TemplateName(String);

impl TemplateName {
    pub fn new(s: &str) -> Self {
        TemplateName(s.to_owned())
    }

    pub fn name(&self) -> &str {
        &self.0
    }
}

pub trait TemplateFail: AardwolfFail {
    fn template(&self) -> TemplateName;

    fn response_kind(&self) -> ResponseKind;
}

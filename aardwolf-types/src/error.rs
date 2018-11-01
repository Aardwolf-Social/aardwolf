use aardwolf_models::user::PermissionError;
use failure::Fail;

pub enum AardwolfErrorKind {
    Redirect(String),
    BadRequest,
    RequiresAuthentication,
    RequiresPermission,
    NotFound,
    InternalServerError,
}

pub trait AardwolfError: Fail {
    fn name(&self) -> &str;
    fn kind(&self) -> AardwolfErrorKind;
    fn description(&self) -> String;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ErrorJson {
    name: String,
    description: String,
}

impl<E> From<E> for ErrorJson
where
    E: AardwolfError,
{
    fn from(e: E) -> Self {
        ErrorJson {
            name: e.name().to_owned(),
            description: e.description(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RedirectTo(String);

impl RedirectTo {
    pub fn new(s: String) -> Self {
        RedirectTo(s)
    }
}

pub trait RedirectError: Fail {
    fn redirect() -> RedirectTo;
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

pub trait TemplateError: AardwolfError {
    fn template(&self) -> TemplateName;
}

impl AardwolfError for PermissionError {
    fn name(&self) -> &str {
        "permission error"
    }

    fn kind(&self) -> AardwolfErrorKind {
        match *self {
            PermissionError::Diesel => AardwolfErrorKind::InternalServerError,
            PermissionError::Permission => AardwolfErrorKind::RequiresPermission,
        }
    }

    fn description(&self) -> String {
        format!("{}", self)
    }
}

impl<T> AardwolfError for T
where
    T: RedirectError,
{
    fn name(&self) -> &str {
        "redirect"
    }

    fn kind(&self) -> AardwolfErrorKind {
        AardwolfErrorKind::Redirect(Self::redirect().0)
    }

    fn description(&self) -> String {
        format!("Redirecting to {}", Self::redirect().0)
    }
}

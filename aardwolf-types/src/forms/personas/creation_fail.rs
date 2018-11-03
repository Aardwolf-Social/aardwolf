use aardwolf_models::user::PermissionError;
use diesel::result::Error as DieselError;
use url::ParseError as UrlParseError;

use crate::error::{AardwolfError, AardwolfErrorKind};

#[derive(Clone, Debug, Fail)]
pub enum PersonaCreationFail {
    #[fail(display = "Failed to validate persona")]
    Validation,
    #[fail(display = "User doesn't have permission to create persona")]
    Permission,
    #[fail(display = "Error in database")]
    Database,
}

impl AardwolfError for PersonaCreationFail {
    fn name(&self) -> &str {
        "Persona Creation Fail"
    }

    fn kind(&self) -> AardwolfErrorKind {
        match *self {
            PersonaCreationFail::Validation => AardwolfErrorKind::BadRequest,
            PersonaCreationFail::Permission => AardwolfErrorKind::RequiresPermission,
            PersonaCreationFail::Database => AardwolfErrorKind::InternalServerError,
        }
    }

    fn description(&self) -> String {
        format!("{}", self)
    }
}

impl From<UrlParseError> for PersonaCreationFail {
    fn from(_: UrlParseError) -> Self {
        PersonaCreationFail::Validation
    }
}

impl From<DieselError> for PersonaCreationFail {
    fn from(_: DieselError) -> Self {
        PersonaCreationFail::Database
    }
}

impl From<PermissionError> for PersonaCreationFail {
    fn from(e: PermissionError) -> Self {
        match e {
            PermissionError::Diesel => PersonaCreationFail::Database,
            PermissionError::Permission => PersonaCreationFail::Permission,
        }
    }
}

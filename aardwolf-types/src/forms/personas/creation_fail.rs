use aardwolf_models::user::PermissionError;
use diesel::result::Error as DieselError;
use openssl::error::ErrorStack;
use url::ParseError as UrlParseError;

use crate::error::AardwolfFail;

#[derive(Clone, Debug, Fail, Serialize)]
pub enum PersonaCreationFail {
    #[fail(display = "Failed to validate persona")]
    Validation,
    #[fail(display = "User doesn't have permission to create persona")]
    Permission,
    #[fail(display = "Error in database")]
    Database,
    #[fail(display = "Error generating keys")]
    Keygen,
}

impl AardwolfFail for PersonaCreationFail {}

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

impl From<ErrorStack> for PersonaCreationFail {
    fn from(_: ErrorStack) -> Self {
        PersonaCreationFail::Keygen
    }
}

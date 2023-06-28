use aardwolf_models::user::PermissionError;
use diesel::result::Error as DieselError;
use openssl::error::ErrorStack;
use thiserror::Error;

use crate::{error::AardwolfFail, forms::personas::ValidatePersonaCreationFail};

#[derive(Clone, Debug, Error, Serialize)]
pub enum PersonaCreationFail {
    #[error("Failed to validate persona")]
    Validation(#[source] ValidatePersonaCreationFail),
    #[error("User doesn't have permission to create persona")]
    Permission,
    #[error("Error in database")]
    Database,
    #[error("Error generating keys")]
    Keygen,
}

impl AardwolfFail for PersonaCreationFail {}

impl From<ValidatePersonaCreationFail> for PersonaCreationFail {
    fn from(e: ValidatePersonaCreationFail) -> Self {
        PersonaCreationFail::Validation(e)
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

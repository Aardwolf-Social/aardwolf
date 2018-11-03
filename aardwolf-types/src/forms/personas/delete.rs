use aardwolf_models::{
    base_actor::persona::Persona,
    user::{AuthenticatedUser, PermissionError, PermissionedUser, PersonaDeleter},
};
use diesel::{pg::PgConnection, result::Error as DieselError};

use crate::{
    error::{AardwolfError, AardwolfErrorKind},
    forms::{personas::FetchPersonaFail, traits::DbAction},
};

pub struct CheckDeletePersonaPermission(AuthenticatedUser);

impl CheckDeletePersonaPermission {
    pub fn new(user: AuthenticatedUser) -> Self {
        CheckDeletePersonaPermission(user)
    }

    pub fn with(self, persona: Persona) -> CheckDeletePersonaPermissionOperation {
        CheckDeletePersonaPermissionOperation(self.0, persona)
    }
}

pub struct CheckDeletePersonaPermissionOperation(AuthenticatedUser, Persona);

impl DbAction<PersonaDeleter, PermissionError> for CheckDeletePersonaPermissionOperation {
    fn db_action(self, conn: &PgConnection) -> Result<PersonaDeleter, PermissionError> {
        self.0.can_delete_persona(self.1, conn)
    }
}

pub struct DeletePersona;

impl DeletePersona {
    pub fn with(self, persona_deleter: PersonaDeleter) -> Delete {
        Delete(persona_deleter)
    }
}

pub struct Delete(PersonaDeleter);

impl DbAction<(), PersonaDeletionFail> for Delete {
    fn db_action(self, conn: &PgConnection) -> Result<(), PersonaDeletionFail> {
        self.0.delete_persona(conn).map_err(From::from)
    }
}

#[derive(Clone, Debug, Fail)]
pub enum PersonaDeletionFail {
    #[fail(display = "Insufficient permissions")]
    Permission,
    #[fail(display = "Error in database")]
    Database,
    #[fail(display = "Persona not found")]
    NotFound,
}

impl From<DieselError> for PersonaDeletionFail {
    fn from(e: DieselError) -> Self {
        match e {
            DieselError::NotFound => PersonaDeletionFail::NotFound,
            _ => PersonaDeletionFail::Database,
        }
    }
}

impl From<PermissionError> for PersonaDeletionFail {
    fn from(e: PermissionError) -> Self {
        match e {
            PermissionError::Permission => PersonaDeletionFail::Permission,
            PermissionError::Diesel => PersonaDeletionFail::Database,
        }
    }
}

impl From<FetchPersonaFail> for PersonaDeletionFail {
    fn from(e: FetchPersonaFail) -> Self {
        match e {
            FetchPersonaFail::Database => PersonaDeletionFail::Database,
            FetchPersonaFail::NotFound => PersonaDeletionFail::NotFound,
        }
    }
}

impl AardwolfError for PersonaDeletionFail {
    fn name(&self) -> &str {
        "Persona Deletion Error"
    }

    fn kind(&self) -> AardwolfErrorKind {
        match *self {
            PersonaDeletionFail::Permission => AardwolfErrorKind::RequiresPermission,
            PersonaDeletionFail::Database => AardwolfErrorKind::InternalServerError,
            PersonaDeletionFail::NotFound => AardwolfErrorKind::NotFound,
        }
    }

    fn description(&self) -> String {
        format!("{}", self)
    }
}

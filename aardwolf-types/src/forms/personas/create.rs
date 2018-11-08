use aardwolf_models::{
    base_actor::{persona::Persona, BaseActor},
    user::{LocalPersonaCreator, PermissionError, PermissionedUser},
};
use diesel::pg::PgConnection;

use crate::forms::{
    personas::{PersonaCreationFail, ValidatedPersonaCreationForm},
    traits::DbAction,
};

pub struct CheckCreatePersonaPermission<U>(U)
where
    U: PermissionedUser;

impl<U> CheckCreatePersonaPermission<U>
where
    U: PermissionedUser,
{
    pub fn new(user: U) -> Self {
        CheckCreatePersonaPermission(user)
    }

    pub fn with(
        self,
        form: ValidatedPersonaCreationForm,
    ) -> CheckCreatePersonaPermissionOperation<U> {
        CheckCreatePersonaPermissionOperation(self.0, form)
    }
}

pub struct CheckCreatePersonaPermissionOperation<U>(U, ValidatedPersonaCreationForm);

impl<U>
    DbAction<
        (LocalPersonaCreator<U>, ValidatedPersonaCreationForm),
        CheckCreatePersonaPermissionFail,
    > for CheckCreatePersonaPermissionOperation<U>
where
    U: PermissionedUser + Clone,
{
    fn db_action(
        self,
        conn: &PgConnection,
    ) -> Result<
        (LocalPersonaCreator<U>, ValidatedPersonaCreationForm),
        CheckCreatePersonaPermissionFail,
    > {
        Ok((self.0.can_make_persona(conn)?, self.1))
    }
}

#[derive(Clone, Debug, Fail, Serialize)]
pub enum CheckCreatePersonaPermissionFail {
    #[fail(display = "Could not check user permissions")]
    Database,
    #[fail(display = "User does not haver permission to create persona")]
    Permission,
}

impl From<PermissionError> for CheckCreatePersonaPermissionFail {
    fn from(e: PermissionError) -> Self {
        match e {
            PermissionError::Diesel => CheckCreatePersonaPermissionFail::Database,
            PermissionError::Permission => CheckCreatePersonaPermissionFail::Permission,
        }
    }
}

pub struct CreatePersona;

impl CreatePersona {
    pub fn with<U>(
        self,
        (persona_creator, form): (LocalPersonaCreator<U>, ValidatedPersonaCreationForm),
    ) -> CreatePersonaOperation<U>
    where
        U: PermissionedUser,
    {
        CreatePersonaOperation(persona_creator, form)
    }
}

pub struct CreatePersonaOperation<U>(LocalPersonaCreator<U>, ValidatedPersonaCreationForm)
where
    U: PermissionedUser;

impl<U> DbAction<(BaseActor, Persona), PersonaCreationFail> for CreatePersonaOperation<U>
where
    U: PermissionedUser,
{
    fn db_action(self, conn: &PgConnection) -> Result<(BaseActor, Persona), PersonaCreationFail> {
        Ok(self.0.create_persona(
            self.1.display_name,
            self.1.profile_url,
            self.1.inbox_url,
            self.1.outbox_url,
            self.1.follow_policy,
            self.1.default_visibility,
            self.1.is_searchable,
            None,
            self.1.shortname,
            conn,
        )?)
    }
}

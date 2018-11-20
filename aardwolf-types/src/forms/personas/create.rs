use aardwolf_models::{
    base_actor::{persona::Persona, BaseActor},
    user::{LocalPersonaCreator, PermissionError, PermissionedUser},
};
use diesel::pg::PgConnection;

use crate::{
    error::AardwolfFail,
    forms::personas::{PersonaCreationFail, ValidatedPersonaCreationForm},
    traits::DbAction,
    wrapper::{DbActionWrapper, Wrapped},
};

pub struct CheckCreatePersonaPermission<U>(pub U)
where
    U: PermissionedUser + Clone;

impl<U> Wrapped for CheckCreatePersonaPermission<U>
where
    U: PermissionedUser + Clone,
{
    type Wrapper = DbActionWrapper<Self, <Self as DbAction>::Item, <Self as DbAction>::Error>;
}

impl<U> DbAction for CheckCreatePersonaPermission<U>
where
    U: PermissionedUser + Clone,
{
    type Item = LocalPersonaCreator<U>;
    type Error = CheckCreatePersonaPermissionFail;

    fn db_action(
        self,
        conn: &PgConnection,
    ) -> Result<LocalPersonaCreator<U>, CheckCreatePersonaPermissionFail> {
        Ok(self.0.can_make_persona(conn)?)
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

impl AardwolfFail for CheckCreatePersonaPermissionFail {}

pub struct CreatePersona<U>(pub LocalPersonaCreator<U>, pub ValidatedPersonaCreationForm)
where
    U: PermissionedUser;

impl<U> Wrapped for CreatePersona<U>
where
    U: PermissionedUser,
{
    type Wrapper = DbActionWrapper<Self, <Self as DbAction>::Item, <Self as DbAction>::Error>;
}

impl<U> DbAction for CreatePersona<U>
where
    U: PermissionedUser,
{
    type Item = (BaseActor, Persona);
    type Error = PersonaCreationFail;

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

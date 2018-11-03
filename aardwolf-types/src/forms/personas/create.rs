use aardwolf_models::{
    base_actor::{persona::Persona, BaseActor},
    user::PermissionedUser,
};
use diesel::pg::PgConnection;

use crate::forms::{
    personas::{PersonaCreationFail, ValidatedPersonaCreationForm},
    traits::DbAction,
};

pub struct CreatePersona<U>(U)
where
    U: PermissionedUser;

impl<U> CreatePersona<U>
where
    U: PermissionedUser,
{
    pub fn new(user: U) -> Self {
        CreatePersona(user)
    }

    pub fn with(self, form: ValidatedPersonaCreationForm) -> CreatePersonaOperation<U> {
        CreatePersonaOperation(self.0, form)
    }
}

pub struct CreatePersonaOperation<U>(U, ValidatedPersonaCreationForm)
where
    U: PermissionedUser;

impl<U> DbAction<(BaseActor, Persona), PersonaCreationFail> for CreatePersonaOperation<U>
where
    U: PermissionedUser,
{
    fn db_action(self, conn: &PgConnection) -> Result<(BaseActor, Persona), PersonaCreationFail> {
        let persona_maker = self.0.can_make_persona(conn)?;

        Ok(persona_maker.create_persona(
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

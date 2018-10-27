use aardwolf_models::{
    base_actor::persona::Persona,
    user::{AuthenticatedUser, PermissionError, PermissionedUser},
};
use diesel::{pg::PgConnection, result::Error as DieselError};

use aardwolf_types::forms::{
    personas::{PersonaCreationFail, PersonaCreationForm},
    traits::Validate,
};

pub(crate) fn create(
    user: AuthenticatedUser,
    form: PersonaCreationForm,
    db: &PgConnection,
) -> Result<String, PersonaCreationFail> {
    form.validate()?.create(&user, db)?;
    Ok(format!("Created!"))
}

#[derive(Debug, Fail)]
#[fail(display = "Failed to delete persona")]
pub struct PersonaDeletionFail;

impl From<DieselError> for PersonaDeletionFail {
    fn from(_: DieselError) -> Self {
        PersonaDeletionFail
    }
}

impl From<PermissionError> for PersonaDeletionFail {
    fn from(_: PermissionError) -> Self {
        PersonaDeletionFail
    }
}

pub(crate) fn delete(
    user: AuthenticatedUser,
    persona_id: i32,
    db: &PgConnection,
) -> Result<String, PersonaDeletionFail> {
    let persona = Persona::by_id(persona_id, db)?;

    user.can_delete_persona(persona, db)?.delete_persona(db)?;

    Ok(format!("Deleted!"))
}

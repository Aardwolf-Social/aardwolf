use aardwolf_models::user::AuthenticatedUser;
use diesel::pg::PgConnection;

use forms::personas::{PersonaCreationFail, PersonaCreationForm};
use forms::traits::Validate;

pub(crate) fn create(
    user: AuthenticatedUser,
    form: PersonaCreationForm,
    db: &PgConnection,
) -> Result<String, PersonaCreationFail> {
    form.validate()?.create(&user, db)?;
    Ok(format!("Created!"))
}

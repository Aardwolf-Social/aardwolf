use rocket::request::Form;

use aardwolf_types::forms::personas::{
    CheckCreatePersonaPermission, CheckCreatePersonaPermissionFail, CheckDeletePersonaPermission,
    CheckDeletePersonaPermissionFail, CreatePersona, DeletePersona, FetchPersona, FetchPersonaFail,
    PersonaCreationFail, PersonaCreationForm, PersonaDeletionFail, ValidatePersonaCreationForm,
};
use types::user::SignedInUser;
use DbConn;

#[get("/new")]
pub fn new(_user: SignedInUser) -> String {
    format!("placeholder")
}

#[derive(Clone, Debug, Fail)]
pub enum PersonaCreateError {
    #[fail(display = "Error talking db")]
    Database,
    #[fail(display = "User does not have permission to create a persona")]
    Permission,
    #[fail(display = "Submitted form is invalid")]
    Form,
}

impl From<PersonaCreationFail> for PersonaCreateError {
    fn from(e: PersonaCreationFail) -> Self {
        match e {
            PersonaCreationFail::Validation => PersonaCreateError::Form,
            PersonaCreationFail::Permission => PersonaCreateError::Permission,
            PersonaCreationFail::Database => PersonaCreateError::Database,
        }
    }
}

impl From<CheckCreatePersonaPermissionFail> for PersonaCreateError {
    fn from(e: CheckCreatePersonaPermissionFail) -> Self {
        match e {
            CheckCreatePersonaPermissionFail::Permission => PersonaCreateError::Permission,
            CheckCreatePersonaPermissionFail::Database => PersonaCreateError::Database,
        }
    }
}

#[post("/create", data = "<form>")]
pub fn create(
    user: SignedInUser,
    form: Form<PersonaCreationForm>,
    db: DbConn,
) -> Result<String, PersonaCreateError> {
    let _ = perform!(&db, PersonaCreateError, [
        (form = ValidatePersonaCreationForm(form.into_inner())),
        (creator = CheckCreatePersonaPermission(user.0)),
        (_ = CreatePersona(creator, form)),
    ])?;

    Ok(format!("Created!"))
}

#[derive(Clone, Debug, Fail)]
pub enum PersonaDeleteError {
    #[fail(display = "Error talking to db actor")]
    Mailbox,
    #[fail(display = "Error talking db")]
    Database,
    #[fail(display = "Error confirming account: {}", _0)]
    Delete(#[cause] PersonaDeletionFail),
}

impl From<PersonaDeletionFail> for PersonaDeleteError {
    fn from(e: PersonaDeletionFail) -> Self {
        PersonaDeleteError::Delete(e)
    }
}

impl From<FetchPersonaFail> for PersonaDeleteError {
    fn from(e: FetchPersonaFail) -> Self {
        PersonaDeleteError::Delete(e.into())
    }
}

impl From<CheckDeletePersonaPermissionFail> for PersonaDeleteError {
    fn from(e: CheckDeletePersonaPermissionFail) -> Self {
        PersonaDeleteError::Delete(e.into())
    }
}

#[get("/delete/<id>")]
pub fn delete(user: SignedInUser, id: i32, db: DbConn) -> Result<String, PersonaDeleteError> {
    let _ = perform!(&db, PersonaDeleteError, [
        (persona = FetchPersona(id)),
        (deleter = CheckDeletePersonaPermission(user.0, persona)),
        (_ = DeletePersona(deleter)),
    ])?;

    Ok(format!("Deleted!"))
}

#[get("/switch/<switch_persona>")]
pub fn switch(_user: SignedInUser, switch_persona: i32) -> String {
    format!("placeholder, {}", switch_persona)
}

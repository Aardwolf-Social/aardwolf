use rocket::request::Form;

use aardwolf_types::{
    forms::personas::{PersonaCreationFail, PersonaCreationForm, ValidatePersonaCreationForm},
    operations::{
        check_create_persona_permission::{
            CheckCreatePersonaPermission, CheckCreatePersonaPermissionFail,
        },
        check_delete_persona_permission::{
            CheckDeletePersonaPermission, CheckDeletePersonaPermissionFail,
        },
        create_persona::CreatePersona,
        delete_persona::{DeletePersona, DeletePersonaFail},
        fetch_persona::{FetchPersona, FetchPersonaFail},
    },
};
use types::user::SignedInUser;
use DbConn;

#[get("/new")]
pub fn new(_user: SignedInUser) -> String {
    drop(_user);
    "placeholder".to_string()
}

#[derive(Clone, Debug, Fail)]
pub enum PersonaCreateError {
    #[fail(display = "Error talking db")]
    Database,
    #[fail(display = "User does not have permission to create a persona")]
    Permission,
    #[fail(display = "Submitted form is invalid")]
    Form,
    #[fail(display = "Could not generate keys")]
    Keygen,
}

impl From<PersonaCreationFail> for PersonaCreateError {
    fn from(e: PersonaCreationFail) -> Self {
        match e {
            PersonaCreationFail::Validation => PersonaCreateError::Form,
            PersonaCreationFail::Permission => PersonaCreateError::Permission,
            PersonaCreationFail::Database => PersonaCreateError::Database,
            PersonaCreationFail::Keygen => PersonaCreateError::Keygen,
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

    Ok("Created!".to_string())
}

#[derive(Clone, Debug, Fail)]
pub enum PersonaDeleteError {
    #[fail(display = "Error talking to db actor")]
    Mailbox,
    #[fail(display = "Error talking db")]
    Database,
    #[fail(display = "Error confirming account: {}", _0)]
    Delete(#[cause] DeletePersonaFail),
}

impl From<DeletePersonaFail> for PersonaDeleteError {
    fn from(e: DeletePersonaFail) -> Self {
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
    perform!(&db, PersonaDeleteError, [
        (persona = FetchPersona(id)),
        (deleter = CheckDeletePersonaPermission(user.0, persona)),
        (_ = DeletePersona(deleter)),
    ])?;

    Ok("Deleted!".to_string())
}

#[get("/switch/<switch_persona>")]
pub fn switch(_user: SignedInUser, switch_persona: i32) -> String {
    drop(_user);
    format!("placeholder, {}", switch_persona)
}

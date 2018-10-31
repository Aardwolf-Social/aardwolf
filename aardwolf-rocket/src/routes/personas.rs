use rocket::request::Form;

use aardwolf_types::{
    error::AardwolfError,
    forms::{
        personas::{
            DeletePersona, GetPersonaById, PersonaCreationFail, PersonaCreationForm,
            UserCanDeletePersona,
        },
        traits::{DbAction, Validate},
    },
};
use types::user::SignedInUser;
use DbConn;

#[get("/new")]
fn new(_user: SignedInUser) -> String {
    format!("placeholder")
}

#[post("/create", data = "<persona_creation_form>")]
fn create(
    user: SignedInUser,
    persona_creation_form: Form<PersonaCreationForm>,
    db: DbConn,
) -> Result<String, PersonaCreationFail> {
    persona_creation_form
        .into_inner()
        .validate()?
        .to_operation(user.0)
        .db_action(&db)?;

    Ok(format!("Created!"))
}

#[get("/delete/<id>")]
fn delete(user: SignedInUser, id: i32, db: DbConn) -> Result<String, Box<dyn AardwolfError>> {
    let persona = GetPersonaById::new(id)
        .db_action(&db)
        .map_err(|e| Box::new(e) as Box<dyn AardwolfError>)?;

    let persona_deleter = UserCanDeletePersona::new(user.0, persona)
        .db_action(&db)
        .map_err(|e| Box::new(e) as Box<dyn AardwolfError>)?;

    DeletePersona::new(persona_deleter)
        .db_action(&db)
        .map_err(|e| Box::new(e) as Box<dyn AardwolfError>)?;

    Ok(format!("Deleted!"))
}

#[get("/switch/<switch_persona>")]
fn switch(_user: SignedInUser, switch_persona: i32) -> String {
    format!("placeholder, {}", switch_persona)
}

use rocket::request::Form;

use DbConn;
use forms::personas::{PersonaCreationFail, PersonaCreationForm};
use types::SignedInUser;

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
    use controllers::personas;
    personas::create(user.0, persona_creation_form.into_inner(), &db)
}

#[get("/delete/<delete_persona>")]
fn delete(_user: SignedInUser, delete_persona: i32) -> String {
    format!("placeholder, {}", delete_persona)
}

#[get("/switch/<switch_persona>")]
fn switch(_user: SignedInUser, switch_persona: i32) -> String {
    format!("placeholder, {}", switch_persona)
}

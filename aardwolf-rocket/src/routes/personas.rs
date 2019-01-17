use aardwolf_models::sql_types::{FollowPolicy, PostVisibility};
use aardwolf_types::{
    forms::personas::{
        PersonaCreationFail, PersonaCreationForm, ValidatePersonaCreationFail,
        ValidatePersonaCreationForm,
    },
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
use rocket::{
    http::{Cookie, Cookies, Status},
    request::Form,
    response::Redirect,
    Response,
};
use rocket_i18n::I18n;

use crate::{render_template, types::user::SignedInUser, DbConn, ResponseOrRedirect};

#[get("/create")]
pub fn new(_user: SignedInUser, i18n: I18n) -> Response<'static> {
    let res = render_template(&aardwolf_templates::FirstLogin::new(
        &i18n.catalog,
        "csrf",
        "",
        "",
        FollowPolicy::AutoAccept,
        PostVisibility::Public,
        false,
        None,
        false,
    ));

    drop(_user);
    drop(i18n);

    res
}

#[derive(Clone, Debug, Fail)]
pub enum PersonaCreateError {
    #[fail(display = "Error talking db")]
    Database,
    #[fail(display = "User does not have permission to create a persona")]
    Permission,
    #[fail(display = "Submitted form is invalid")]
    Form(#[cause] ValidatePersonaCreationFail),
    #[fail(display = "Could not generate keys")]
    Keygen,
}

impl From<ValidatePersonaCreationFail> for PersonaCreateError {
    fn from(e: ValidatePersonaCreationFail) -> Self {
        PersonaCreateError::Form(e)
    }
}

impl From<PersonaCreationFail> for PersonaCreateError {
    fn from(e: PersonaCreationFail) -> Self {
        match e {
            PersonaCreationFail::Validation(e) => PersonaCreateError::Form(e),
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
    i18n: I18n,
    mut cookies: Cookies,
    db: DbConn,
) -> ResponseOrRedirect {
    let form = form.into_inner();
    let form_state = form.as_state();

    let res = perform!(&db, PersonaCreateError, [
        (form = ValidatePersonaCreationForm(form)),
        (creator = CheckCreatePersonaPermission(user.0)),
        (_ = CreatePersona(creator, form)),
    ]);

    let res = match res {
        Ok((_actor, persona)) => {
            let mut cookie = Cookie::new("persona_id", format!("{}", persona.id()));
            cookie.set_http_only(true);
            cookies.add_private(cookie);
            Redirect::to("/").into()
        }
        Err(e) => {
            let (status, validation, system) = match e {
                PersonaCreateError::Form(ref e) => (Status::BadRequest, Some(e), false),
                _ => (Status::InternalServerError, None, true),
            };

            let mut response = render_template(&aardwolf_templates::FirstLogin::new(
                &i18n.catalog,
                "csrf",
                &form_state.display_name,
                &form_state.shortname,
                form_state.follow_policy,
                form_state.default_visibility,
                form_state.is_searchable,
                validation,
                system,
            ));
            response.set_status(status);
            response.into()
        }
    };

    drop(i18n);

    res
}

#[derive(Clone, Debug, Fail)]
pub enum PersonaDeleteError {
    #[fail(display = "Error talking to db actor")]
    Mailbox,
    #[fail(display = "Error talking db")]
    Database,
    #[fail(display = "Error deleting persona: {}", _0)]
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

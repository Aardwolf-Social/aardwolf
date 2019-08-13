use aardwolf_models::{base_actor::persona::Persona, user::AuthenticatedUser};
use aardwolf_templates::FirstLogin;
use aardwolf_types::{
    error::AardwolfFail,
    forms::personas::{
        PersonaCreationFail, PersonaCreationForm, PersonaCreationFormState,
        ValidatePersonaCreationFail, ValidatePersonaCreationForm,
    },
    operations::{
        check_create_persona_permission::{
            CheckCreatePersonaPermission, CheckCreatePersonaPermissionFail,
        },
        check_delete_persona_permission::CheckDeletePersonaPermission,
        create_persona::CreatePersona,
        delete_persona::{DeletePersona, DeletePersonaFail},
        fetch_persona::FetchPersona,
    },
    traits::{DbAction, DbActionError, Validate},
};
use actix_i18n::I18n;
use actix_session::Session;
use actix_web::{
    web::{Data, Form, Path},
    HttpResponse
};
use failure::Fail;
use serde_derive::Serialize;

use crate::{
    action::{RenderableExt, redirect},
    error::redirect_error,
    types::user::SignedInUser,
    AppConfig, WithRucte,
};

pub(crate) fn new((_user, i18n): (SignedInUser, I18n)) -> HttpResponse {
    let res = FirstLogin::new(
        &i18n.catalog,
        "csrf",
        &PersonaCreationFormState::default(),
        None,
        false,
    ).ok();

    drop(i18n);

    res
}

async fn create_inner(
    state: AppConfig,
    form: PersonaCreationForm,
    user: AuthenticatedUser,
    session: Session,
) -> Result<HttpResponse, PersonaCreateError> {
    let form = ValidatePersonaCreationForm(form).validate()?;
    let creator = CheckCreatePersonaPermission(user).run(state.pool.clone()).await?;
    let (_, persona) = CreatePersona(creator, form, state.generator.clone()).run(state.pool.clone()).await?;
    SetPersonaCookie(session, persona).run()?;
    Ok(redirect("/"))
}

pub(crate) async fn create(
    (session, state, user, form, i18n): (
        Session,
        Data<AppConfig>,
        SignedInUser,
        Form<PersonaCreationForm>,
        I18n,
    ),
) -> Result<HttpResponse, actix_web::Error> {
    let form = form.into_inner();
    let form_state = form.as_state();

    let error = match create_inner((*state).clone(), form, user.0, session).await {
        Ok(res) => return Ok(res),
        Err(e) => e,
    };

    let (mut res, validation, system) = match error {
        PersonaCreateError::Form(ref e) => (HttpResponse::BadRequest(), Some(e), false),
        _ => (HttpResponse::InternalServerError(), None, true),
    };

    Ok(res.ructe(FirstLogin::new(
        &i18n.catalog,
        "csrf",
        &form_state,
        validation,
        system,
    )))
}

async fn delete_inner(
    state: AppConfig,
    user: AuthenticatedUser,
    id: i32,
) -> Result<HttpResponse, PersonaDeleteError> {
    let persona = FetchPersona(id).run(state.pool.clone()).await?;
    let deleter = CheckDeletePersonaPermission(user, persona).run(state.pool.clone()).await?;
    DeletePersona(deleter).run(state.pool).await?;
    Ok(redirect("/"))
}

pub(crate) async fn delete(
    (state, user, id): (Data<AppConfig>, SignedInUser, Path<i32>),
) -> Result<HttpResponse, actix_web::Error> {
    let _ = match delete_inner((*state).clone(), user.0, id.into_inner()).await {
        Ok(res) => return Ok(res),
        Err(e) => e,
    };

    Ok(redirect_error("/personas", None))
}

#[derive(Clone, Debug, Fail)]
pub enum PersonaCreateError {
    #[fail(display = "Error talking to db actor")]
    Canceled,
    #[fail(display = "Error talking db")]
    Database,
    #[fail(display = "User does not have permission to create a persona")]
    Permission,
    #[fail(display = "Could not set cookie")]
    Cookie,
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
            CheckCreatePersonaPermissionFail::Database => PersonaCreateError::Database,
            CheckCreatePersonaPermissionFail::Permission => PersonaCreateError::Permission,
        }
    }
}

impl From<DbActionError<CheckCreatePersonaPermissionFail>> for PersonaCreateError {
    fn from(e: DbActionError<CheckCreatePersonaPermissionFail>) -> Self {
        match e {
            DbActionError::Pool(_) => PersonaCreateError::Database,
            DbActionError::Canceled => PersonaCreateError::Canceled,
            DbActionError::Error(e) => e.into(),
        }
    }
}

impl From<DbActionError<PersonaCreationFail>> for PersonaCreateError {
    fn from(e: DbActionError<PersonaCreationFail>) -> Self {
        match e {
            DbActionError::Pool(_) => PersonaCreateError::Database,
            DbActionError::Canceled => PersonaCreateError::Canceled,
            DbActionError::Error(e) => e.into(),
        }
    }
}

struct SetPersonaCookie(Session, Persona);

impl SetPersonaCookie {
    fn run(self) -> Result<(), PersonaCreateError> {
        self
            .0
            .set("persona_id", self.1.id())
            .map_err(|_| PersonaCreateError::Cookie)
    }
}

#[derive(Clone, Debug, Fail, Serialize)]
pub enum PersonaDeleteError {
    #[fail(display = "Error talking to db actor")]
    Canceled,
    #[fail(display = "Error talking db")]
    Database,
    #[fail(display = "Error deleting persona: {}", _0)]
    Delete(#[cause] DeletePersonaFail),
}

impl<E> From<DbActionError<E>> for PersonaDeleteError
where
    E: Into<DeletePersonaFail> + AardwolfFail,
{
    fn from(e: DbActionError<E>) -> Self {
        match e {
            DbActionError::Pool(_) => PersonaDeleteError::Database,
            DbActionError::Canceled => PersonaDeleteError::Canceled,
            DbActionError::Error(e) => PersonaDeleteError::Delete(e.into()),
        }
    }
}

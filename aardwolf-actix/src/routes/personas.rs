use aardwolf_models::sql_types::{FollowPolicy, PostVisibility};
use aardwolf_types::{
    error::AardwolfFail,
    forms::personas::{
        PersonaCreationFail, PersonaCreationForm, ValidatePersonaCreationFail,
        ValidatePersonaCreationForm,
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
};
use actix_web::{
    http::header::LOCATION, middleware::session::Session, Form, HttpResponse, Path, State,
};
use failure::Fail;
use futures::Future;
use rocket_i18n::I18n;
use serde_derive::Serialize;

use crate::{
    db::DbActionError, error::RedirectError, types::user::SignedInUser, AppConfig, WithRucte,
};

pub(crate) fn new((_user, i18n): (SignedInUser, I18n)) -> HttpResponse {
    let res = HttpResponse::Ok().with_ructe(aardwolf_templates::FirstLogin::new(
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

    drop(i18n);

    res
}

#[derive(Clone, Debug, Fail)]
pub enum PersonaCreateError {
    #[fail(display = "Error talking to db actor")]
    Mailbox,
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
            DbActionError::Connection => PersonaCreateError::Database,
            DbActionError::Mailbox => PersonaCreateError::Mailbox,
            DbActionError::Action(e) => e.into(),
        }
    }
}

impl From<DbActionError<PersonaCreationFail>> for PersonaCreateError {
    fn from(e: DbActionError<PersonaCreationFail>) -> Self {
        match e {
            DbActionError::Connection => PersonaCreateError::Database,
            DbActionError::Mailbox => PersonaCreateError::Mailbox,
            DbActionError::Action(e) => e.into(),
        }
    }
}

pub(crate) fn create(
    (session, state, user, form, i18n): (
        Session,
        State<AppConfig>,
        SignedInUser,
        Form<PersonaCreationForm>,
        I18n,
    ),
) -> Box<dyn Future<Item = HttpResponse, Error = actix_web::error::Error>> {
    let form = form.into_inner();
    let form_state = form.as_state();

    let res = perform!(state, PersonaCreateError, [
        (form = ValidatePersonaCreationForm(form)),
        (creater = CheckCreatePersonaPermission(user.0)),
        (_ = CreatePersona(creater, form, state.generator.clone())),
    ]);

    Box::new(
        res.and_then(move |(_actor, persona)| {
            session
                .set("persona_id", persona.id())
                .map_err(|_| PersonaCreateError::Cookie)
        })
        .map(|_| HttpResponse::SeeOther().header(LOCATION, "/").finish())
        .or_else(move |e| {
            let (mut res, validation, system) = match e {
                PersonaCreateError::Form(ref e) => (HttpResponse::BadRequest(), Some(e), false),
                _ => (HttpResponse::InternalServerError(), None, true),
            };

            Ok(res.with_ructe(aardwolf_templates::FirstLogin::new(
                &i18n.catalog,
                "csrf",
                &form_state.display_name,
                &form_state.shortname,
                form_state.follow_policy,
                form_state.default_visibility,
                form_state.is_searchable,
                validation,
                system,
            )))
        }),
    )
}

#[derive(Clone, Debug, Fail, Serialize)]
pub enum PersonaDeleteError {
    #[fail(display = "Error talking to db actor")]
    Mailbox,
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
            DbActionError::Connection => PersonaDeleteError::Database,
            DbActionError::Mailbox => PersonaDeleteError::Mailbox,
            DbActionError::Action(e) => PersonaDeleteError::Delete(e.into()),
        }
    }
}

pub(crate) fn delete(
    (state, user, id): (State<AppConfig>, SignedInUser, Path<i32>),
) -> Box<dyn Future<Item = String, Error = actix_web::error::Error>> {
    let res = perform!(state, PersonaDeleteError, [
        (persona = FetchPersona(id.into_inner())),
        (deleter = CheckDeletePersonaPermission(user.0, persona)),
        (_ = DeletePersona(deleter)),
    ]);

    Box::new(
        res.map(|_| "Deleted!".to_string())
            .map_err(|_| RedirectError::new("/personas", &None).into()),
    )
}

use aardwolf_models::{
    base_actor::persona::Persona,
};
use aardwolf_types::{
    error::AardwolfFail,
    forms::personas::{
        PersonaCreationFail, PersonaCreationForm, ValidatePersonaCreationFail,
        ValidatePersonaCreationForm, PersonaCreationFormState,
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
use actix_i18n::I18n;
use actix_session::Session;
use actix_web::{
    web::{Data, Form, Path},
    HttpResponse,
};
use failure::Fail;
use futures::{future::IntoFuture, Future};
use serde_derive::Serialize;

use crate::{
    action::{Action, Impossible, Redirect, Wrapped},
    db::DbActionError,
    error::RedirectError,
    types::user::SignedInUser,
    AppConfig, WithRucte,
};

pub(crate) fn new((_user, i18n): (SignedInUser, I18n)) -> HttpResponse {
    let res = HttpResponse::Ok().with_ructe(aardwolf_templates::FirstLogin::new(
        &i18n.catalog,
        "csrf",
        &PersonaCreationFormState::default(),
        None,
        false,
    ));

    drop(i18n);

    res
}

pub(crate) fn create(
    (session, state, user, form, i18n): (
        Session,
        Data<AppConfig>,
        SignedInUser,
        Form<PersonaCreationForm>,
        I18n,
    ),
) -> Box<dyn Future<Item = HttpResponse, Error = actix_web::error::Error>> {
    let form = form.into_inner();
    let form_state = form.as_state();

    let res = perform!((*state).clone(), PersonaCreateError, [
        (form = ValidatePersonaCreationForm(form)),
        (creater = CheckCreatePersonaPermission(user.0)),
        ((_, persona) = CreatePersona(creater, form, state.generator.clone())),
        (_ = SetPersonaCookie(session, persona)),
        (_ = Redirect("/".to_owned())),
    ]);

    Box::new(res.or_else(move |e| {
        let (mut res, validation, system) = match e {
            PersonaCreateError::Form(ref e) => (HttpResponse::BadRequest(), Some(e), false),
            _ => (HttpResponse::InternalServerError(), None, true),
        };

        Ok(res.with_ructe(aardwolf_templates::FirstLogin::new(
            &i18n.catalog,
            "csrf",
            &form_state,
            validation,
            system,
        )))
    }))
}

pub(crate) fn delete(
    (state, user, id): (Data<AppConfig>, SignedInUser, Path<i32>),
) -> Box<dyn Future<Item = String, Error = actix_web::error::Error>> {
    let res = perform!((*state).clone(), PersonaDeleteError, [
        (persona = FetchPersona(id.into_inner())),
        (deleter = CheckDeletePersonaPermission(user.0, persona)),
        (_ = DeletePersona(deleter)),
    ]);

    Box::new(
        res.map(|_| "Deleted!".to_string())
            .map_err(|_| RedirectError::new("/personas", &None).into()),
    )
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

impl From<Impossible> for PersonaCreateError {
    fn from(_: Impossible) -> Self {
        PersonaCreateError::Mailbox
    }
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

struct SetPersonaCookie(Session, Persona);

impl Wrapped for SetPersonaCookie {
    type Wrapper = SetPersonaCookie;
}

impl Action<(), PersonaCreateError> for SetPersonaCookie {
    fn action(self, _: AppConfig) -> Box<dyn Future<Item = (), Error = PersonaCreateError>> {
        Box::new(
            self.0
                .set("persona_id", self.1.id())
                .map_err(|_| PersonaCreateError::Cookie)
                .into_future(),
        )
    }
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

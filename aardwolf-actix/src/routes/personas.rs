use aardwolf_types::{
    error::AardwolfFail,
    forms::personas::{
        CheckDeletePersonaPermission, DeletePersona, FetchPersona, PersonaCreationFail,
        PersonaCreationForm, PersonaDeletionFail, ValidatePersonaCreationForm,
    },
    operations::{
        check_create_persona_permission::{
            CheckCreatePersonaPermission, CheckCreatePersonaPermissionFail,
        },
        create_persona::CreatePersona,
    },
};
use actix_web::{Form, Path, State};
use failure::Fail;
use futures::Future;
use serde_derive::Serialize;

use crate::{db::DbActionError, error::RedirectError, types::user::SignedInUser, AppConfig};

pub(crate) fn new((_state, _user): (State<AppConfig>, SignedInUser)) -> String {
    "placeholder".to_string()
}

#[derive(Clone, Debug, Fail)]
pub enum PersonaCreateError {
    #[fail(display = "Error talking to db actor")]
    Mailbox,
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
    (state, user, form): (State<AppConfig>, SignedInUser, Form<PersonaCreationForm>),
) -> Box<dyn Future<Item = String, Error = actix_web::error::Error>> {
    let res = perform!(state, PersonaCreateError, [
        (form = ValidatePersonaCreationForm(form.into_inner())),
        (creater = CheckCreatePersonaPermission(user.0)),
        (_ = CreatePersona(creater, form)),
    ]);

    Box::new(
        res.map(|(_base_actor, _persona)| "Created!".to_string())
            .map_err(|_| RedirectError::new("/personas/new", &None).into()),
    )
}

#[derive(Clone, Debug, Fail, Serialize)]
pub enum PersonaDeleteError {
    #[fail(display = "Error talking to db actor")]
    Mailbox,
    #[fail(display = "Error talking db")]
    Database,
    #[fail(display = "Error confirming account: {}", _0)]
    Delete(#[cause] PersonaDeletionFail),
}

impl<E> From<DbActionError<E>> for PersonaDeleteError
where
    E: Into<PersonaDeletionFail> + AardwolfFail,
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

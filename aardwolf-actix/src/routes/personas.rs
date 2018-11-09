use aardwolf_types::{
    error::AardwolfFail,
    forms::personas::{
        CheckCreatePersonaPermission, CheckCreatePersonaPermissionFail,
        CheckDeletePersonaPermission, CreatePersona, DeletePersona, FetchPersona,
        PersonaCreationFail, PersonaCreationForm, PersonaDeletionFail, ValidatePersonaCreationForm,
    },
};
use actix_web::{Form, Path, State};
use futures::Future;

use crate::{
    action::{DbActionWrapper, ValidateWrapper},
    db::DbActionError,
    error::RedirectError,
    types::user::SignedInUser,
    AppConfig,
};

pub(crate) fn new((_state, _user): (State<AppConfig>, SignedInUser)) -> String {
    format!("placeholder")
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
    let res = perform!(
        state,
        form.into_inner(),
        PersonaCreateError,
        [
            (ValidateWrapper<_, _, _> => ValidatePersonaCreationForm),
            (DbActionWrapper<_, _, _> => CheckCreatePersonaPermission::new(user.0)),
            (DbActionWrapper<_, _, _> => CreatePersona),
        ]
    );

    Box::new(
        res.map(|(_base_actor, _persona)| format!("Created!"))
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
    let res = perform!(
        state,
        id.into_inner(),
        PersonaDeleteError,
        [
            (DbActionWrapper<_, _, _> => FetchPersona),
            (DbActionWrapper<_, _, _> => CheckDeletePersonaPermission::new(user.0)),
            (DbActionWrapper<_, _, _> => DeletePersona),
        ]
    );

    Box::new(
        res.map(|_| format!("Deleted!"))
            .map_err(|_| RedirectError::new("/personas", &None).into()),
    )
}

use aardwolf_types::forms::personas::{
    DeletePersona, GetPersonaById, PersonaCreationFail, PersonaDeletionFail, UserCanDeletePersona,
};
use actix_web::{Path, State};
use failure::Fail;
use futures::Future;

use crate::{
    action::DbActionWrapper,
    db::DbActionError,
    error::RedirectError,
    types::{personas::ValidPersonaCreationForm, user::SignedInUser},
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
    #[fail(display = "Error confirming account: {}", _0)]
    Create(#[cause] PersonaCreationFail),
}

impl From<DbActionError<PersonaCreationFail>> for PersonaCreateError {
    fn from(e: DbActionError<PersonaCreationFail>) -> Self {
        match e {
            DbActionError::Connection => PersonaCreateError::Database,
            DbActionError::Mailbox => PersonaCreateError::Mailbox,
            DbActionError::Action(e) => PersonaCreateError::Create(e),
        }
    }
}

pub(crate) fn create(
    (state, user, persona_creation_form): (
        State<AppConfig>,
        SignedInUser,
        ValidPersonaCreationForm,
    ),
) -> Box<dyn Future<Item = String, Error = actix_web::error::Error>> {
    let res = perform!(
        state,
        user.0,
        PersonaCreateError,
        [(DbActionWrapper<_, _, _> => persona_creation_form.0),]
    );

    Box::new(
        res.map(|(_base_actor, _persona)| format!("Created!"))
            .map_err(|_| RedirectError::new("/personas/new", None).into()),
    )
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

impl<E> From<DbActionError<E>> for PersonaDeleteError
where
    E: Into<PersonaDeletionFail> + Fail,
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
            (DbActionWrapper<_, _, _> => GetPersonaById::new()),
            (DbActionWrapper<_, _, _> => UserCanDeletePersona::new(user.0)),
            (DbActionWrapper<_, _, _> => DeletePersona::new()),
        ]
    );

    Box::new(
        res.map(|_| format!("Deleted!"))
            .map_err(|_| RedirectError::new("/personas", None).into()),
    )
}

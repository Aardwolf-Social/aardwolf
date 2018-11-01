use aardwolf_types::forms::personas::{
    DeletePersona, GetPersonaById, PersonaDeletionFail, UserCanDeletePersona,
};
use actix_web::{Path, State};
use futures::Future;

use crate::{
    db::execute_db_query,
    error::RedirectError,
    types::{personas::ValidPersonaCreationForm, user::SignedInUser},
    AppConfig,
};

pub(crate) fn new((_state, _user): (State<AppConfig>, SignedInUser)) -> String {
    format!("placeholder")
}

pub(crate) fn create(
    (state, user, persona_creation_form): (
        State<AppConfig>,
        SignedInUser,
        ValidPersonaCreationForm,
    ),
) -> Box<dyn Future<Item = String, Error = actix_web::error::Error>> {
    Box::new(
        execute_db_query(state.clone(), persona_creation_form.0.to_operation(user.0))
            .map(|(_base_actor, _persona)| format!("Created!"))
            .map_err(|_| RedirectError::new("/personas/new", None).into()),
    )
}

pub(crate) fn delete(
    (state, user, id): (State<AppConfig>, SignedInUser, Path<i32>),
) -> Box<dyn Future<Item = String, Error = actix_web::error::Error>> {
    let state1 = state.clone();
    let state2 = state.clone();

    Box::new(
        execute_db_query(state.clone(), GetPersonaById::new(id.into_inner()))
            .map_err(|e| e.map_err::<PersonaDeletionFail>())
            .and_then(move |persona| {
                execute_db_query(state1, UserCanDeletePersona::new(user.0, persona))
                    .map_err(|e| e.map_err::<PersonaDeletionFail>())
            })
            .and_then(move |persona_deleter| {
                execute_db_query(state2, DeletePersona::new(persona_deleter))
            })
            .map(|_| format!("Deleted!"))
            .map_err(|_| RedirectError::new("/personas", None).into()),
    )
}

use aardwolf_types::forms::personas::{DeletePersona, GetPersonaById, UserCanDeletePersona};
use actix_web::{Path, State};
use futures::Future;

use crate::{
    db::PerformDbAction,
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
        state
            .db
            .send(PerformDbAction::new(
                persona_creation_form.0.to_operation(user.0),
            ))
            .then(|res| match res {
                Ok(item_res) => match item_res {
                    Ok(item) => Ok(item),
                    Err(e) => Err(e.into()),
                },
                Err(e) => Err(e.into()),
            })
            .map(|(_base_actor, _persona)| format!("Created!"))
            .map_err(|_: actix_web::Error| RedirectError::new("/personas/new", None).into()),
    )
}

pub(crate) fn delete(
    (state, user, id): (State<AppConfig>, SignedInUser, Path<i32>),
) -> Box<dyn Future<Item = String, Error = actix_web::error::Error>> {
    let db1 = state.db.clone();
    let db2 = state.db.clone();

    Box::new(
        state
            .db
            .send(PerformDbAction::new(GetPersonaById::new(id.into_inner())))
            .then(|res| match res {
                Ok(item_res) => match item_res {
                    Ok(item) => Ok(item),
                    Err(e) => Err(e.into()),
                },
                Err(e) => Err(e.into()),
            })
            .and_then(move |persona| {
                db1.send(PerformDbAction::new(UserCanDeletePersona::new(
                    user.0, persona,
                )))
                .then(|res| match res {
                    Ok(item_res) => match item_res {
                        Ok(item) => Ok(item),
                        Err(e) => Err(e.into()),
                    },
                    Err(e) => Err(e.into()),
                })
            })
            .and_then(move |persona_deleter| {
                db2.send(PerformDbAction::new(DeletePersona::new(persona_deleter)))
                    .then(|res| match res {
                        Ok(item_res) => match item_res {
                            Ok(item) => Ok(item),
                            Err(e) => Err(e.into()),
                        },
                        Err(e) => Err(e.into()),
                    })
            })
            .map(|_| format!("Deleted!"))
            .map_err(|_: actix_web::Error| RedirectError::new("/personas", None).into()),
    )
}

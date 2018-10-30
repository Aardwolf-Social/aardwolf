use aardwolf_models::user::UserLike;
use aardwolf_types::forms::auth::{
    ConfirmToken, SignInError, SignUpError, ValidatedSignInForm, ValidatedSignUpForm,
};
use actix_web::{http::header::LOCATION, middleware::session::Session, HttpResponse, Query, State};
use futures::Future;

use crate::{
    db::PerformDbAction,
    error::{RedirectError, RenderResult},
    types::user::SignedInUser,
    AppConfig,
};

pub(crate) fn sign_up_form_with_error(
    (state, error): (State<AppConfig>, Option<Query<SignUpError>>),
) -> RenderResult {
    let token = "some csrf token";

    error
        .map(|error| {
            let msg = error.into_inner().msg;

            state.render(
                "sign_up",
                &hashmap!{
                    "token" => token,
                    "error_msg" => msg.as_str(),
                },
            )
        })
        .unwrap_or_else(|| sign_up_form(state))
}

fn sign_up_form(state: State<AppConfig>) -> RenderResult {
    let token = "some csrf token";

    state.render(
        "sign_up",
        &hashmap!{
            "token" => token,
        },
    )
}

pub(crate) fn sign_in_form_with_error(
    (state, error): (State<AppConfig>, Option<Query<SignInError>>),
) -> RenderResult {
    let token = "some csrf token";

    error
        .map(|error| {
            let msg = error.into_inner().msg;

            state.render(
                "sign_in",
                &hashmap!{
                    "token" => token,
                    "error_msg" => msg.as_str(),
                },
            )
        })
        .unwrap_or_else(|| sign_in_form(state))
}

fn sign_in_form(state: State<AppConfig>) -> RenderResult {
    let token = "some csrf token";

    state.render(
        "sign_in",
        &hashmap!{
            "token" => token,
        },
    )
}

pub(crate) fn sign_up(
    (state, signup_form): (State<AppConfig>, ValidatedSignUpForm),
) -> Box<dyn Future<Item = HttpResponse, Error = actix_web::Error>> {
    Box::new(
        state
            .db
            .send(PerformDbAction::new(signup_form))
            .then(|res| match res {
                Ok(item_res) => match item_res {
                    Ok(item) => Ok(item),
                    Err(e) => Err(e.into()),
                },
                Err(e) => Err(e.into()),
            })
            .map(|(email, token)| {
                println!(
                    "confirmation token url: /auth/confirmation?id={}&token={}",
                    email.id(),
                    token
                );

                HttpResponse::SeeOther()
                    .header(LOCATION, "/auth/sign_in")
                    .finish()
            })
            .map_err(|e: actix_web::Error| {
                RedirectError::new("/auth/sign_up", Some(e.to_string().as_str())).into()
            }),
    )
}

pub(crate) fn sign_in(
    (state, session, signin_form): (State<AppConfig>, Session, ValidatedSignInForm),
) -> Box<dyn Future<Item = HttpResponse, Error = actix_web::Error>> {
    Box::new(
        state
            .db
            .send(PerformDbAction::new(signin_form))
            .then(|res| match res {
                Ok(item_res) => match item_res {
                    Ok(item) => Ok(item),
                    Err(e) => Err(e.into()),
                },
                Err(e) => Err(e.into()),
            })
            .and_then(move |user| session.set("user_id", user.id()))
            .map(|_| HttpResponse::SeeOther().header(LOCATION, "/").finish())
            .map_err(|e| RedirectError::new("/auth/sign_in", Some(e.to_string().as_str())).into()),
    )
}

pub(crate) fn confirm(
    (state, token): (State<AppConfig>, Query<ConfirmToken>),
) -> Box<dyn Future<Item = HttpResponse, Error = actix_web::Error>> {
    Box::new(
        state
            .db
            .send(PerformDbAction::new(token.into_inner()))
            .then(|res| match res {
                Ok(item_res) => match item_res {
                    Ok(item) => Ok(item),
                    Err(e) => Err(e.into()),
                },
                Err(e) => Err(e.into()),
            })
            .map(|_user| {
                HttpResponse::SeeOther()
                    .header(LOCATION, "/auth/sign_in")
                    .finish()
            }),
    )
}

pub(crate) fn sign_out((session, _): (Session, SignedInUser)) -> HttpResponse {
    session.remove("user_id");

    HttpResponse::SeeOther()
        .header(LOCATION, "/auth/sign_in")
        .finish()
}

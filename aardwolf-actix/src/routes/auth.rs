use aardwolf_models::user::UserLike;
use aardwolf_types::forms::auth::{
    ConfirmAccountFail, ConfirmToken, SignInErrorMessage, SignInFail, SignUpErrorMessage,
    SignUpFail,
};
use actix_web::{http::header::LOCATION, middleware::session::Session, HttpResponse, Query, State};
use failure::Fail;
use futures::Future;

use crate::{
    action::DbActionWrapper,
    db::DbActionError,
    error::{RedirectError, RenderResult},
    types::{
        auth::{ValidSignInForm, ValidSignUpForm},
        user::SignedInUser,
    },
    AppConfig,
};

pub(crate) fn sign_up_form_with_error(
    (state, error): (State<AppConfig>, Option<Query<SignUpErrorMessage>>),
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
    (state, error): (State<AppConfig>, Option<Query<SignInErrorMessage>>),
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

#[derive(Clone, Debug, Fail)]
pub enum SignUpError {
    #[fail(display = "Error talking to db actor")]
    Mailbox,
    #[fail(display = "Error talking db")]
    Database,
    #[fail(display = "Error signing up: {}", _0)]
    SignUp(#[cause] SignUpFail),
}

impl<E> From<DbActionError<E>> for SignUpError
where
    E: Into<SignUpFail> + Fail,
{
    fn from(e: DbActionError<E>) -> Self {
        match e {
            DbActionError::Connection => SignUpError::Database,
            DbActionError::Mailbox => SignUpError::Mailbox,
            DbActionError::Action(e) => SignUpError::SignUp(e.into()),
        }
    }
}

pub(crate) fn sign_up(
    (state, signup_form): (State<AppConfig>, ValidSignUpForm),
) -> Box<dyn Future<Item = HttpResponse, Error = actix_web::Error>> {
    let res = perform!(state, (), SignUpError, [(DbActionWrapper<_, _, _> => signup_form.0),]);

    Box::new(
        res.map(|(email, token)| {
            println!(
                "confirmation token url: /auth/confirmation?id={}&token={}",
                email.id(),
                token
            );

            HttpResponse::SeeOther()
                .header(LOCATION, "/auth/sign_in")
                .finish()
        })
        .map_err(|e| RedirectError::new("/auth/sign_up", Some(e.to_string().as_str())).into()),
    )
}

#[derive(Clone, Debug, Fail)]
pub enum SignInError {
    #[fail(display = "Error talking to db actor")]
    Mailbox,
    #[fail(display = "Error talking db")]
    Database,
    #[fail(display = "Error signing in: {}", _0)]
    SignIn(#[cause] SignInFail),
}

impl<E> From<DbActionError<E>> for SignInError
where
    E: Into<SignInFail> + Fail,
{
    fn from(e: DbActionError<E>) -> Self {
        match e {
            DbActionError::Connection => SignInError::Database,
            DbActionError::Mailbox => SignInError::Mailbox,
            DbActionError::Action(e) => SignInError::SignIn(e.into()),
        }
    }
}

pub(crate) fn sign_in(
    (state, session, signin_form): (State<AppConfig>, Session, ValidSignInForm),
) -> Box<dyn Future<Item = HttpResponse, Error = actix_web::Error>> {
    let res = perform!(state, (), SignInError, [(DbActionWrapper<_, _, _> => signin_form.0),]);

    Box::new(
        res.map_err(|e| RedirectError::new("/auth/sign_in", Some(e.to_string().as_str())).into())
            .and_then(move |user| {
                session.set("user_id", user.id()).map_err(|e| {
                    RedirectError::new("/auth/sign_in", Some(e.to_string().as_str())).into()
                })
            })
            .map(|_| HttpResponse::SeeOther().header(LOCATION, "/").finish()),
    )
}

#[derive(Clone, Debug, Fail)]
pub enum ConfirmError {
    #[fail(display = "Error talking to db actor")]
    Mailbox,
    #[fail(display = "Error talking db")]
    Database,
    #[fail(display = "Error confirming account: {}", _0)]
    Confirm(#[cause] ConfirmAccountFail),
}

impl From<DbActionError<ConfirmAccountFail>> for ConfirmError {
    fn from(e: DbActionError<ConfirmAccountFail>) -> Self {
        match e {
            DbActionError::Connection => ConfirmError::Database,
            DbActionError::Mailbox => ConfirmError::Mailbox,
            DbActionError::Action(e) => ConfirmError::Confirm(e),
        }
    }
}

pub(crate) fn confirm(
    (state, token): (State<AppConfig>, Query<ConfirmToken>),
) -> Box<dyn Future<Item = HttpResponse, Error = actix_web::Error>> {
    let res =
        perform!(state, (), ConfirmError, [(DbActionWrapper<_, _, _> => token.into_inner()),]);

    Box::new(
        res.map(|_user| {
            HttpResponse::SeeOther()
                .header(LOCATION, "/auth/sign_in")
                .finish()
        })
        .map_err(|e| RedirectError::new("/auth/sign_up", Some(e.to_string().as_str())).into()),
    )
}

pub(crate) fn sign_out((session, _): (Session, SignedInUser)) -> HttpResponse {
    session.remove("user_id");

    HttpResponse::SeeOther()
        .header(LOCATION, "/auth/sign_in")
        .finish()
}

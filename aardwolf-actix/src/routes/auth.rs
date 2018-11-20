use aardwolf_models::user::UserLike;
use aardwolf_types::forms::auth::{
    ConfirmAccountFail, ConfirmToken, ConfirmationToken, SignIn, SignInErrorMessage, SignInFail,
    SignInForm, SignUp, SignUpErrorMessage, SignUpFail, SignUpForm, ValidateSignInForm,
    ValidateSignInFormFail, ValidateSignUpForm, ValidateSignUpFormFail,
};
use actix_web::{
    http::header::LOCATION, middleware::session::Session, Form, HttpResponse, Query, State,
};
use failure::Fail;
use futures::future::Future;

use crate::{
    db::DbActionError,
    error::{RedirectError, RenderResult},
    types::user::SignedInUser,
    AppConfig,
};

pub(crate) fn sign_up_form(
    (state, error): (State<AppConfig>, Option<Query<SignUpErrorMessage>>),
) -> RenderResult {
    match error {
        Some(error) => sign_up_form_with_error(state, error.into_inner()),
        None => sign_up_form_without_error(state),
    }
}

fn sign_up_form_with_error(state: State<AppConfig>, msg: SignUpErrorMessage) -> RenderResult {
    let token = "some csrf token";

    state.render(
        "sign_up",
        &hashmap!{
            "token" => token,
            "error_msg" => msg.msg.as_str(),
        },
    )
}

fn sign_up_form_without_error(state: State<AppConfig>) -> RenderResult {
    let token = "some csrf token";

    state.render(
        "sign_up",
        &hashmap!{
            "token" => token,
        },
    )
}

pub(crate) fn sign_in_form(
    (state, error): (State<AppConfig>, Option<Query<SignInErrorMessage>>),
) -> RenderResult {
    match error {
        Some(error) => sign_in_form_with_error(state, error.into_inner()),
        None => sign_in_form_without_error(state),
    }
}

fn sign_in_form_with_error(state: State<AppConfig>, error: SignInErrorMessage) -> RenderResult {
    let token = "some csrf token";

    state.render(
        "sign_in",
        &hashmap!{
            "token" => token,
            "error_msg" => error.msg.as_str(),
        },
    )
}

fn sign_in_form_without_error(state: State<AppConfig>) -> RenderResult {
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

impl From<DbActionError<SignUpFail>> for SignUpError {
    fn from(e: DbActionError<SignUpFail>) -> Self {
        match e {
            DbActionError::Connection => SignUpError::Database,
            DbActionError::Mailbox => SignUpError::Mailbox,
            DbActionError::Action(e) => SignUpError::SignUp(e),
        }
    }
}

impl From<ValidateSignUpFormFail> for SignUpError {
    fn from(e: ValidateSignUpFormFail) -> Self {
        SignUpError::SignUp(e.into())
    }
}

pub(crate) fn sign_up(
    (state, form): (State<AppConfig>, Form<SignUpForm>),
) -> Box<dyn Future<Item = HttpResponse, Error = actix_web::Error>> {
    let res = perform!( state, SignUpError, [
        (form = ValidateSignUpForm(form.into_inner())),
        (_ = SignUp(form)),
    ]);

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
        .map_err(|e| RedirectError::new("/auth/sign_up", &Some(e.to_string())).into()),
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

impl From<DbActionError<SignInFail>> for SignInError {
    fn from(e: DbActionError<SignInFail>) -> Self {
        match e {
            DbActionError::Connection => SignInError::Database,
            DbActionError::Mailbox => SignInError::Mailbox,
            DbActionError::Action(e) => SignInError::SignIn(e),
        }
    }
}

impl From<ValidateSignInFormFail> for SignInError {
    fn from(e: ValidateSignInFormFail) -> Self {
        SignInError::SignIn(e.into())
    }
}

pub(crate) fn sign_in(
    (state, session, form): (State<AppConfig>, Session, Form<SignInForm>),
) -> Box<dyn Future<Item = HttpResponse, Error = actix_web::Error>> {
    let res = perform!(state, SignInError, [
        (form = ValidateSignInForm(form.into_inner())),
        (_ = SignIn(form)),
    ]);

    Box::new(
        res.map_err(|e| RedirectError::new("/auth/sign_in", &Some(e.to_string())).into())
            .and_then(move |user| {
                session
                    .set("user_id", user.id())
                    .map_err(|e| RedirectError::new("/auth/sign_in", &Some(e.to_string())).into())
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
    (state, query): (State<AppConfig>, Query<ConfirmationToken>),
) -> Box<dyn Future<Item = HttpResponse, Error = actix_web::Error>> {
    let res = perform!(state, ConfirmError, [
        (_ = ConfirmToken(query.into_inner())),
    ]);

    Box::new(
        res.map(|_user| {
            HttpResponse::SeeOther()
                .header(LOCATION, "/auth/sign_in")
                .finish()
        })
        .map_err(|e| RedirectError::new("/auth/sign_up", &Some(e.to_string())).into()),
    )
}

pub(crate) fn sign_out((session, _user): (Session, SignedInUser)) -> HttpResponse {
    session.remove("user_id");

    HttpResponse::SeeOther()
        .header(LOCATION, "/auth/sign_in")
        .finish()
}

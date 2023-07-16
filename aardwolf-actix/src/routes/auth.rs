use aardwolf_models::user::{
    email::{EmailToken, UnverifiedEmail},
    AuthenticatedUser, UserLike,
};
use aardwolf_templates::{SignIn as TSignIn, SignUp as TSignUp};
use aardwolf_types::{
    forms::auth::{
        SignInForm, SignInFormState, SignUpForm, SignUpFormState, ValidateSignInForm,
        ValidateSignInFormFail, ValidateSignUpForm, ValidateSignUpFormFail,
    },
    operations::{
        confirm_account::{ConfirmAccount, ConfirmAccountFail, ConfirmAccountToken},
        sign_in::{SignIn, SignInFail},
        sign_up::{SignUp, SignUpFail},
    },
    traits::{DbAction, DbActionError, Validate},
};
use actix_session::Session;
use actix_web::{
    http::header::LOCATION,
    web::{Data, Form, Query},
    HttpResponse, ResponseError,
};
use rocket_i18n::I18n;
use std::fmt;
use thiserror::Error;

use crate::{
    action::redirect,
    error::redirect_error,
    traits::{RenderableExt, WithRucte},
    types::user::SignedInUser,
    AppConfig,
};

pub(crate) async fn sign_up_form(i18n: Data<I18n>) -> HttpResponse {
    let res = TSignUp::new(
        &i18n.catalog,
        "csrf token",
        &SignUpFormState::default(),
        None,
        false,
    )
    .ok();

    drop(i18n);

    res
}

async fn sign_up_inner(state: &AppConfig, form: SignUpForm) -> Result<HttpResponse, SignUpError> {
    let form = ValidateSignUpForm(form).validate()?;
    let (email, token) = SignUp(form).run(state.pool.clone()).await?;
    print_result(email, token);
    Ok(redirect("/auth/sign_in"))
}

pub(crate) async fn sign_up(
    (state, form, i18n): (Data<AppConfig>, Form<SignUpForm>, Data<I18n>),
) -> Result<HttpResponse, SignUpResponseError> {
    let form = form.into_inner();
    let form_state = form.as_state();

    sign_up_inner(&state, form)
        .await
        .map_err(|error| SignUpResponseError {
            i18n: I18n {
                // I18n can't be cloned but its fields can be
                catalog: i18n.catalog.clone(),
                lang: i18n.lang,
            },
            csrf_token: "csrf token".to_owned(),
            form_state,
            error,
        })
}

pub(crate) async fn sign_in_form(i18n: Data<I18n>) -> HttpResponse {
    let res = TSignIn::new(
        &i18n.catalog,
        "csrf token",
        &SignInFormState::default(),
        None,
        false,
    )
    .ok();

    drop(i18n);

    res
}

async fn sign_in_inner(
    state: &AppConfig,
    form: SignInForm,
    session: Session,
) -> Result<HttpResponse, SignInError> {
    let form = ValidateSignInForm(form).validate()?;
    let user = SignIn(form).run(state.pool.clone()).await?;
    SetUserCookie(session, user).run()?;
    Ok(redirect("/"))
}

pub(crate) async fn sign_in(
    (state, session, form, i18n): (Data<AppConfig>, Session, Form<SignInForm>, Data<I18n>),
) -> Result<HttpResponse, SignInResponseError> {
    let form = form.into_inner();
    let form_state = form.as_state();

    sign_in_inner(&state, form, session)
        .await
        .map_err(|error| SignInResponseError {
            i18n: I18n {
                // I18n can't be cloned but its fields can be
                catalog: i18n.catalog.clone(),
                lang: i18n.lang,
            },
            csrf_token: "csrf token".to_owned(),
            form_state,
            error,
        })
}

pub(crate) async fn confirm(
    (state, query): (Data<AppConfig>, Query<ConfirmAccountToken>),
) -> Result<HttpResponse, ConfirmError> {
    ConfirmAccount(query.into_inner())
        .run(state.pool.clone())
        .await?;
    Ok(redirect("/auth/sign_in"))
}

pub(crate) async fn sign_out((session, _user): (Session, SignedInUser)) -> HttpResponse {
    session.remove("user_id");

    HttpResponse::SeeOther()
        .append_header((LOCATION, "/auth/sign_in"))
        .finish()
}

#[derive(Error)]
#[error("Error")]
pub struct SignUpResponseError {
    i18n: I18n,
    csrf_token: String,
    form_state: SignUpFormState,
    error: SignUpError,
}

impl fmt::Debug for SignUpResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "...")
    }
}

impl ResponseError for SignUpResponseError {
    fn error_response(&self) -> HttpResponse {
        let (mut res, valid, system) = match self.error {
            SignUpError::SignUp(ref e) => match *e {
                SignUpFail::ValidationError(ref e) => (HttpResponse::BadRequest(), Some(e), false),
                _ => (HttpResponse::InternalServerError(), None, true),
            },
            _ => (HttpResponse::InternalServerError(), None, true),
        };

        res.ructe(TSignUp::new(
            &self.i18n.catalog,
            &self.csrf_token,
            &self.form_state,
            valid,
            system,
        ))
    }
}

#[derive(Error)]
#[error("Error")]
pub struct SignInResponseError {
    i18n: I18n,
    csrf_token: String,
    form_state: SignInFormState,
    error: SignInError,
}

impl fmt::Debug for SignInResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "...")
    }
}

impl ResponseError for SignInResponseError {
    fn error_response(&self) -> HttpResponse {
        let (mut res, validation, system) = match self.error {
            SignInError::SignIn(ref e) => match *e {
                SignInFail::ValidationError(ref e) => (HttpResponse::BadRequest(), Some(e), false),
                _ => (HttpResponse::InternalServerError(), None, true),
            },
            _ => (HttpResponse::InternalServerError(), None, true),
        };

        res.ructe(TSignIn::new(
            &self.i18n.catalog,
            &self.csrf_token,
            &self.form_state,
            validation,
            system,
        ))
    }
}

#[derive(Clone, Debug, Error)]
pub enum SignUpError {
    #[error("Error talking to db actor")]
    Canceled,
    #[error("Error talking db")]
    Database,
    #[error("Error signing up: {}", _0)]
    SignUp(#[source] SignUpFail),
}

impl From<DbActionError<SignUpFail>> for SignUpError {
    fn from(e: DbActionError<SignUpFail>) -> Self {
        match e {
            DbActionError::Pool(_) => SignUpError::Database,
            DbActionError::Canceled => SignUpError::Canceled,
            DbActionError::Error(e) => SignUpError::SignUp(e),
        }
    }
}

impl From<ValidateSignUpFormFail> for SignUpError {
    fn from(e: ValidateSignUpFormFail) -> Self {
        SignUpError::SignUp(e.into())
    }
}

fn print_result(email: UnverifiedEmail, token: EmailToken) {
    println!(
        "confirmation token url: /auth/confirmation?id={}&token={}",
        email.id(),
        token,
    );
}

#[derive(Clone, Debug, Error)]
pub enum SignInError {
    #[error("Error talking to db actor")]
    Canceled,
    #[error("Error talking db")]
    Database,
    #[error("Error setting the cookie")]
    Cookie,
    #[error("Error signing in: {}", _0)]
    SignIn(#[source] SignInFail),
}

impl From<DbActionError<SignInFail>> for SignInError {
    fn from(e: DbActionError<SignInFail>) -> Self {
        match e {
            DbActionError::Pool(_) => SignInError::Database,
            DbActionError::Canceled => SignInError::Canceled,
            DbActionError::Error(e) => SignInError::SignIn(e),
        }
    }
}

impl From<ValidateSignInFormFail> for SignInError {
    fn from(e: ValidateSignInFormFail) -> Self {
        SignInError::SignIn(e.into())
    }
}

struct SetUserCookie(Session, AuthenticatedUser);

impl SetUserCookie {
    fn run(self) -> Result<(), SignInError> {
        self.0
            .insert("user_id", self.1.id())
            .map_err(|_| SignInError::Cookie)
    }
}

#[derive(Clone, Debug, Error)]
pub enum ConfirmError {
    #[error("Error talking to db actor")]
    Canceled,
    #[error("Error talking db")]
    Database,
    #[error("Error confirming account: {}", _0)]
    Confirm(#[source] ConfirmAccountFail),
}

impl From<DbActionError<ConfirmAccountFail>> for ConfirmError {
    fn from(e: DbActionError<ConfirmAccountFail>) -> Self {
        match e {
            DbActionError::Pool(_) => ConfirmError::Database,
            DbActionError::Canceled => ConfirmError::Canceled,
            DbActionError::Error(e) => ConfirmError::Confirm(e),
        }
    }
}

impl ResponseError for ConfirmError {
    fn error_response(&self) -> HttpResponse {
        redirect_error("/auth/sign_up", Some(self.to_string()))
    }
}

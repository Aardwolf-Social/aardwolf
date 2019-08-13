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
use actix_i18n::I18n;
use actix_session::Session;
use actix_web::{
    http::header::LOCATION,
    web::{Data, Form, Query},
    HttpResponse
};
use failure::Fail;

use crate::{
    action::{redirect, RenderableExt},
    error::redirect_error,
    types::user::SignedInUser,
    AppConfig, WithRucte,
};

pub(crate) fn sign_up_form(i18n: I18n) -> HttpResponse {
    let res = TSignUp::new(
        &i18n.catalog,
        "csrf token",
        &SignUpFormState::default(),
        None,
        false,
    ).ok();

    drop(i18n);

    res
}

async fn sign_up_inner(state: AppConfig, form: SignUpForm) -> Result<HttpResponse, SignUpError> {
    let form = ValidateSignUpForm(form).validate()?;
    let (email, token) = SignUp(form).run(state.pool.clone()).await?;
    PrintResult(email, token).run();
    Ok(redirect("/auth/sign_in"))
}

pub(crate) async fn sign_up(
    (state, form, i18n): (Data<AppConfig>, Form<SignUpForm>, I18n),
) -> Result<HttpResponse, actix_web::Error> {
    let form = form.into_inner();
    let form_state = form.as_state();

    let error = match sign_up_inner((*state).clone(), form).await {
        Ok(res) => return Ok(res),
        Err(e) => e,
    };

    let (mut res, valid, system) = match error {
        SignUpError::SignUp(ref e) => match *e {
            SignUpFail::ValidationError(ref e) => (HttpResponse::BadRequest(), Some(e), false),
            _ => (HttpResponse::InternalServerError(), None, true),
        },
        _ => (HttpResponse::InternalServerError(), None, true),
    };

    Ok(res.ructe(TSignUp::new(
        &i18n.catalog,
        "csrf token",
        &form_state,
        valid,
        system,
    )))
}

pub(crate) fn sign_in_form(i18n: I18n) -> HttpResponse {
    let res = TSignIn::new(
        &i18n.catalog,
        "csrf token",
        &SignInFormState::default(),
        None,
        false,
    ).ok();

    drop(i18n);

    res
}

async fn sign_in_inner(
    state: AppConfig,
    form: SignInForm,
    session: Session,
) -> Result<HttpResponse, SignInError> {
    let form = ValidateSignInForm(form).validate()?;
    let user = SignIn(form).run(state.pool.clone()).await?;
    SetUserCookie(session, user).run()?;
    Ok(redirect("/"))
}

pub(crate) async fn sign_in(
    (state, session, form, i18n): (Data<AppConfig>, Session, Form<SignInForm>, I18n),
) -> Result<HttpResponse, actix_web::Error> {
    let form = form.into_inner();
    let form_state = form.as_state();

    let error = match sign_in_inner((*state).clone(), form, session).await {
        Ok(res) => return Ok(res),
        Err(e) => e,
    };

    let (mut res, validation, system) = match error {
        SignInError::SignIn(ref e) => match *e {
            SignInFail::ValidationError(ref e) => (HttpResponse::BadRequest(), Some(e), false),
            _ => (HttpResponse::InternalServerError(), None, true),
        },
        _ => (HttpResponse::InternalServerError(), None, true),
    };

    Ok(res.ructe(TSignIn::new(
        &i18n.catalog,
        "csrf token",
        &form_state,
        validation,
        system,
    )))
}

async fn confirm_inner(
    state: AppConfig,
    query: ConfirmAccountToken,
) -> Result<HttpResponse, ConfirmError> {
    ConfirmAccount(query).run(state.pool.clone()).await?;
    Ok(redirect("/auth/sign_in"))
}

pub(crate) async fn confirm(
    (state, query): (Data<AppConfig>, Query<ConfirmAccountToken>),
) -> Result<HttpResponse, actix_web::Error> {
    let error = match confirm_inner((*state).clone(), query.into_inner()).await {
        Ok(res) => return Ok(res),
        Err(e) => e,
    };

    Ok(redirect_error("/auth/sign_up", Some(error.to_string())))
}

pub(crate) fn sign_out((session, _user): (Session, SignedInUser)) -> HttpResponse {
    session.remove("user_id");

    HttpResponse::SeeOther()
        .header(LOCATION, "/auth/sign_in")
        .finish()
}

#[derive(Clone, Debug, Fail)]
pub enum SignUpError {
    #[fail(display = "Error talking to db actor")]
    Canceled,
    #[fail(display = "Error talking db")]
    Database,
    #[fail(display = "Error signing up: {}", _0)]
    SignUp(#[cause] SignUpFail),
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

struct PrintResult(UnverifiedEmail, EmailToken);

impl PrintResult {
    fn run(self) {
        println!(
            "confirmation token url: /auth/confirmation?id={}&token={}",
            self.0.id(),
            self.1,
        );
    }
}

#[derive(Clone, Debug, Fail)]
pub enum SignInError {
    #[fail(display = "Error talking to db actor")]
    Canceled,
    #[fail(display = "Error talking db")]
    Database,
    #[fail(display = "Error setting the cookie")]
    Cookie,
    #[fail(display = "Error signing in: {}", _0)]
    SignIn(#[cause] SignInFail),
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
        self
            .0
            .set("user_id", self.1.id())
            .map_err(|_| SignInError::Cookie)
    }
}

#[derive(Clone, Debug, Fail)]
pub enum ConfirmError {
    #[fail(display = "Error talking to db actor")]
    Canceled,
    #[fail(display = "Error talking db")]
    Database,
    #[fail(display = "Error confirming account: {}", _0)]
    Confirm(#[cause] ConfirmAccountFail),
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

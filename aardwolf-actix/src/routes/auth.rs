use aardwolf_models::user::{
    email::{EmailToken, UnverifiedEmail},
    AuthenticatedUser, UserLike,
};
use aardwolf_types::{
    forms::auth::{
        SignInForm, SignUpForm, ValidateSignInForm, ValidateSignInFormFail, ValidateSignUpForm,
        ValidateSignUpFormFail,
    },
    operations::{
        confirm_account::{ConfirmAccount, ConfirmAccountFail, ConfirmAccountToken},
        sign_in::{SignIn, SignInFail},
        sign_up::{SignUp, SignUpFail},
    },
};
use actix_i18n::I18n;
use actix_session::Session;
use actix_web::{
    http::header::LOCATION,
    web::{Data, Form, Query},
    HttpResponse,
};
use failure::Fail;
use futures::future::{Future, IntoFuture};

use crate::{
    action::{Action, Impossible, Redirect, Wrapped},
    db::DbActionError,
    error::RedirectError,
    types::user::SignedInUser,
    AppConfig, WithRucte,
};

pub(crate) fn sign_up_form(i18n: I18n) -> HttpResponse {
    let res = HttpResponse::Ok().with_ructe(aardwolf_templates::SignUp::new(
        &i18n.catalog,
        "csrf token",
        "",
        None,
        false,
    ));

    drop(i18n);

    res
}

pub(crate) fn sign_up(
    (state, form, i18n): (Data<AppConfig>, Form<SignUpForm>, I18n),
) -> Box<dyn Future<Item = HttpResponse, Error = actix_web::Error>> {
    let form = form.into_inner();
    let form_state = form.as_state();

    let res = perform!((*state).clone(), SignUpError, [
        (form = ValidateSignUpForm(form)),
        ((email, token) = SignUp(form)),
        (_ = PrintResult(email, token)),
        (_ = Redirect("/auth/sign_in".to_owned())),
    ]);

    Box::new(res.or_else(move |e: SignUpError| {
        let (mut res, valid, system) = match e {
            SignUpError::SignUp(ref e) => match *e {
                SignUpFail::ValidationError(ref e) => (HttpResponse::BadRequest(), Some(e), false),
                _ => (HttpResponse::InternalServerError(), None, true),
            },
            _ => (HttpResponse::InternalServerError(), None, true),
        };

        Ok(res.with_ructe(aardwolf_templates::SignUp::new(
            &i18n.catalog,
            "csrf token",
            &form_state.email,
            valid,
            system,
        )))
    }))
}

pub(crate) fn sign_in_form(i18n: I18n) -> HttpResponse {
    let res = HttpResponse::Ok().with_ructe(aardwolf_templates::SignIn::new(
        &i18n.catalog,
        "csrf token",
        "",
        None,
        false,
    ));

    drop(i18n);

    res
}

pub(crate) fn sign_in(
    (state, session, form, i18n): (Data<AppConfig>, Session, Form<SignInForm>, I18n),
) -> Box<dyn Future<Item = HttpResponse, Error = actix_web::Error>> {
    let form = form.into_inner();
    let form_state = form.as_state();

    let res = perform!((*state).clone(), SignInError, [
        (form = ValidateSignInForm(form)),
        (user = SignIn(form)),
        (_ = SetUserCookie(session, user)),
        (_ = Redirect("/".to_owned())),
    ]);

    Box::new(res.or_else(move |e| {
        let (mut res, validation, system) = match e {
            SignInError::SignIn(ref e) => match *e {
                SignInFail::ValidationError(ref e) => (HttpResponse::BadRequest(), Some(e), false),
                _ => (HttpResponse::InternalServerError(), None, true),
            },
            _ => (HttpResponse::InternalServerError(), None, true),
        };

        Ok(res.with_ructe(aardwolf_templates::SignIn::new(
            &i18n.catalog,
            "csrf token",
            &form_state.email,
            validation,
            system,
        )))
    }))
}

pub(crate) fn confirm(
    (state, query): (Data<AppConfig>, Query<ConfirmAccountToken>),
) -> Box<dyn Future<Item = HttpResponse, Error = actix_web::Error>> {
    let res = perform!((*state).clone(), ConfirmError, [
        (_ = ConfirmAccount(query.into_inner())),
        (_ = Redirect("/auth/sign_in".to_owned())),
    ]);

    Box::new(res.map_err(|e| RedirectError::new("/auth/sign_up", &Some(e.to_string())).into()))
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
    Mailbox,
    #[fail(display = "Error talking db")]
    Database,
    #[fail(display = "Error signing up: {}", _0)]
    SignUp(#[cause] SignUpFail),
}

impl From<Impossible> for SignUpError {
    fn from(_: Impossible) -> Self {
        SignUpError::Mailbox
    }
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

struct PrintResult(UnverifiedEmail, EmailToken);

impl Wrapped for PrintResult {
    type Wrapper = PrintResult;
}

impl Action<(), Impossible> for PrintResult {
    fn action(self, _: AppConfig) -> Box<dyn Future<Item = (), Error = Impossible>> {
        println!(
            "confirmation token url: /auth/confirmation?id={}&token={}",
            self.0.id(),
            self.1,
        );

        Box::new(futures::future::ok(()))
    }
}

#[derive(Clone, Debug, Fail)]
pub enum SignInError {
    #[fail(display = "Error talking to db actor")]
    Mailbox,
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

impl From<Impossible> for SignInError {
    fn from(_: Impossible) -> Self {
        SignInError::Mailbox
    }
}

struct SetUserCookie(Session, AuthenticatedUser);

impl Wrapped for SetUserCookie {
    type Wrapper = SetUserCookie;
}

impl Action<(), SignInError> for SetUserCookie {
    fn action(self, _: AppConfig) -> Box<dyn Future<Item = (), Error = SignInError>> {
        Box::new(
            self.0
                .set("user_id", self.1.id())
                .map_err(|_| SignInError::Cookie)
                .into_future(),
        )
    }
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

impl From<Impossible> for ConfirmError {
    fn from(_: Impossible) -> Self {
        ConfirmError::Mailbox
    }
}

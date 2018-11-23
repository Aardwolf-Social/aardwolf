use aardwolf_models::user::UserLike;
use aardwolf_templates::templates;
use aardwolf_types::forms::auth::{
    ConfirmAccountFail, ConfirmToken, ConfirmationToken, SignIn, SignInFail, SignInForm, SignUp,
    SignUpFail, SignUpForm, ValidateSignInForm, ValidateSignInFormFail, ValidateSignUpForm,
    ValidateSignUpFormFail,
};
use actix_web::{
    http::header::LOCATION, middleware::session::Session, Form, HttpResponse, Query, State,
};
use failure::Fail;
use futures::future::Future;
use rocket_i18n::I18n;

use crate::{
    db::DbActionError, error::RedirectError, types::user::SignedInUser, AppConfig, WithRucte,
};

pub(crate) fn sign_up_form(i18n: I18n) -> HttpResponse {
    HttpResponse::Ok().with_ructe(move |buf| {
        templates::sign_up(
            buf,
            aardwolf_templates::SignUp::new(&i18n.catalog, "csrf token", "", None, false),
        )
    })
}

pub(crate) fn sign_in_form(i18n: I18n) -> HttpResponse {
    HttpResponse::Ok().with_ructe(move |buf| {
        templates::sign_in(
            buf,
            aardwolf_templates::SignIn::new(&i18n.catalog, "csrf token", "", None, false),
        )
    })
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
    (state, form, i18n): (State<AppConfig>, Form<SignUpForm>, I18n),
) -> Box<dyn Future<Item = HttpResponse, Error = actix_web::Error>> {
    let form = form.into_inner();
    let form_state = form.as_state();

    let res = perform!(state, SignUpError, [
        (form = ValidateSignUpForm(form)),
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
        .or_else(move |e| {
            Ok(match e {
                SignUpError::SignUp(e) => match e {
                    SignUpFail::ValidationError(e) => {
                        HttpResponse::BadRequest().with_ructe(move |buf| {
                            templates::sign_up(
                                buf,
                                aardwolf_templates::SignUp::new(
                                    &i18n.catalog,
                                    "csrf token",
                                    &form_state.email,
                                    Some(&e),
                                    false,
                                ),
                            )
                        })
                    }
                    _ => HttpResponse::InternalServerError().with_ructe(move |buf| {
                        templates::sign_up(
                            buf,
                            aardwolf_templates::SignUp::new(
                                &i18n.catalog,
                                "csrf token",
                                &form_state.email,
                                None,
                                true,
                            ),
                        )
                    }),
                },
                _ => HttpResponse::InternalServerError().with_ructe(move |buf| {
                    templates::sign_up(
                        buf,
                        aardwolf_templates::SignUp::new(
                            &i18n.catalog,
                            "csrf token",
                            &form_state.email,
                            None,
                            true,
                        ),
                    )
                }),
            })
        }),
    )
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

pub(crate) fn sign_in(
    (state, session, form, i18n): (State<AppConfig>, Session, Form<SignInForm>, I18n),
) -> Box<dyn Future<Item = HttpResponse, Error = actix_web::Error>> {
    let form = form.into_inner();
    let form_state = form.as_state();

    let res = perform!(state, SignInError, [
        (form = ValidateSignInForm(form)),
        (_ = SignIn(form)),
    ]);

    Box::new(
        res.and_then(move |user| {
            session
                .set("user_id", user.id())
                .map_err(|_| SignInError::Cookie)
        })
        .map(|_| HttpResponse::SeeOther().header(LOCATION, "/").finish())
        .or_else(move |e| {
            Ok(match e {
                SignInError::SignIn(e) => match e {
                    SignInFail::ValidationError(e) => {
                        HttpResponse::BadRequest().with_ructe(move |buf| {
                            templates::sign_in(
                                buf,
                                aardwolf_templates::SignIn::new(
                                    &i18n.catalog,
                                    "csrf token",
                                    &form_state.email,
                                    Some(&e),
                                    false,
                                ),
                            )
                        })
                    }
                    _ => HttpResponse::InternalServerError().with_ructe(move |buf| {
                        templates::sign_in(
                            buf,
                            aardwolf_templates::SignIn::new(
                                &i18n.catalog,
                                "csrf token",
                                &form_state.email,
                                None,
                                false,
                            ),
                        )
                    }),
                },
                _ => HttpResponse::InternalServerError().with_ructe(move |buf| {
                    templates::sign_in(
                        buf,
                        aardwolf_templates::SignIn::new(
                            &i18n.catalog,
                            "csrf token",
                            &form_state.email,
                            None,
                            false,
                        ),
                    )
                }),
            })
        }),
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

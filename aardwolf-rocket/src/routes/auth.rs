use aardwolf_models::user::UserLike;
use aardwolf_types::forms::auth::{
    ConfirmAccountFail, ConfirmToken, ConfirmationToken, SignIn, SignInErrorMessage, SignInFail,
    SignInForm, SignUp, SignUpErrorMessage, SignUpFail, SignUpForm, ValidateSignInForm,
    ValidateSignInFormFail, ValidateSignUpForm, ValidateSignUpFormFail,
};
use rocket::{
    http::{Cookie, Cookies},
    request::Form,
    response::Redirect,
    Response,
};
use rocket_i18n::I18n;

use render_template;
use templates;
use types::user::SignedInUser;
use DbConn;

#[get("/sign_up?<error..>")]
pub fn sign_up_form_with_error(i18n: I18n, error: Form<SignUpErrorMessage>) -> Response<'static> {
    render_template(move |buf| {
        templates::sign_up(buf, i18n.catalog.clone(), "csrf token", "aardwolf.social")
    })
}

#[get("/sign_up")]
pub fn sign_up_form(i18n: I18n) -> Response<'static> {
    render_template(move |buf| {
        templates::sign_up(buf, i18n.catalog.clone(), "csrf token", "aardwolf.social")
    })
}

#[get("/sign_in?<error..>")]
pub fn sign_in_form_with_error(i18n: I18n, error: Form<SignInErrorMessage>) -> Response<'static> {
    render_template(move |buf| templates::sign_in(buf, i18n.catalog.clone(), "csrf token"))
}

#[get("/sign_in")]
pub fn sign_in_form(i18n: I18n) -> Response<'static> {
    render_template(move |buf| templates::sign_in(buf, i18n.catalog.clone(), "csrf token"))
}

#[derive(Clone, Debug, Fail)]
pub enum SignUpError {
    #[fail(display = "Error talking db")]
    Database,
    #[fail(display = "Error signing up: {}", _0)]
    SignUp(#[cause] SignUpFail),
}

impl From<SignUpFail> for SignUpError {
    fn from(e: SignUpFail) -> Self {
        SignUpError::SignUp(e)
    }
}

impl From<ValidateSignUpFormFail> for SignUpError {
    fn from(e: ValidateSignUpFormFail) -> Self {
        SignUpError::SignUp(e.into())
    }
}
#[post("/sign_up", data = "<form>")]
pub fn sign_up(form: Form<SignUpForm>, db: DbConn) -> Redirect {
    let res = perform!(&db, SignUpError, [
        (form = ValidateSignUpForm(form.into_inner())),
        (_ = SignUp(form)),
    ]);

    match res {
        Ok((email, token)) => {
            // just printing this out for now so we can copy/paste into the browser to confirm accounts,
            // but obviously this will need to be emailed to the user
            println!(
                "confirmation token url: /auth/confirmation?id={}&token={}",
                email.id(),
                token
            );

            Redirect::to("/auth/sign_in")
        }
        Err(e) => {
            println!("unable to create account: {}, {:?}", e, e);
            Redirect::to(format!("/auth/sign_up?msg={}", e))
        }
    }
}

#[derive(Clone, Debug, Fail)]
pub enum SignInError {
    #[fail(display = "Error talking db")]
    Database,
    #[fail(display = "Error signing in: {}", _0)]
    SignIn(#[cause] SignInFail),
}

impl From<SignInFail> for SignInError {
    fn from(e: SignInFail) -> Self {
        SignInError::SignIn(e)
    }
}

impl From<ValidateSignInFormFail> for SignInError {
    fn from(e: ValidateSignInFormFail) -> Self {
        SignInError::SignIn(e.into())
    }
}

#[post("/sign_in", data = "<form>")]
pub fn sign_in(form: Form<SignInForm>, db: DbConn, mut cookies: Cookies) -> Redirect {
    // TODO: check csrf token (this will probably be a request guard)

    let res = perform!(&db, SignInError, [
        (form = ValidateSignInForm(form.into_inner())),
        (_ = SignIn(form)),
    ]);

    match res {
        Ok(user) => {
            let mut cookie = Cookie::new("user_id", format!("{}", user.id()));
            cookie.set_http_only(true);
            cookies.add_private(cookie);
            Redirect::to("/")
        }
        Err(e) => {
            println!("unable to log in: {}, {:?}", e, e);
            Redirect::to(format!("/auth/sign_in?msg={}", e))
        }
    }
}

#[derive(Clone, Debug, Fail)]
pub enum ConfirmError {
    #[fail(display = "Error talking db")]
    Database,
    #[fail(display = "Error confirming account: {}", _0)]
    Confirm(#[cause] ConfirmAccountFail),
}

impl From<ConfirmAccountFail> for ConfirmError {
    fn from(e: ConfirmAccountFail) -> Self {
        ConfirmError::Confirm(e)
    }
}

#[get("/confirmation?<token..>")]
pub fn confirm(token: Form<ConfirmationToken>, db: DbConn) -> Result<Redirect, ConfirmError> {
    let res = perform!(&db, ConfirmError, [
       (_ = ConfirmToken(token.into_inner())),
    ]);

    Ok(match res {
        Ok(_) => Redirect::to("/auth/sign_in"),
        Err(e) => {
            println!("unable to confirm account: {}, {:?}", e, e);
            return Err(e);
        }
    })
}

#[post("/sign_out")]
pub fn sign_out(_user: SignedInUser, mut cookies: Cookies) -> Redirect {
    cookies.remove_private(Cookie::named("user_id"));
    Redirect::to("/auth/sign_in")
}

use aardwolf_models::user::UserLike;
use aardwolf_types::forms::auth::{
    ConfirmAccountFail, ConfirmToken, ConfirmationToken, SignIn, SignInFail, SignInForm, SignUp,
    SignUpFail, SignUpForm, ValidateSignInForm, ValidateSignUpForm,
};
use rocket::{
    http::{Cookie, Cookies, Status},
    request::Form,
    response::Redirect,
    Response,
};
use rocket_i18n::I18n;

use render_template;
use types::user::SignedInUser;
use DbConn;

#[derive(Responder)]
#[cfg_attr(feature = "cargo-clippy", allow(clippy::large_enum_variant))]
pub enum ResponseOrRedirect {
    Response(Response<'static>),
    Redirect(Redirect),
}

impl From<Response<'static>> for ResponseOrRedirect {
    fn from(r: Response<'static>) -> Self {
        ResponseOrRedirect::Response(r)
    }
}

impl From<Redirect> for ResponseOrRedirect {
    fn from(r: Redirect) -> Self {
        ResponseOrRedirect::Redirect(r)
    }
}

#[get("/sign_up")]
pub fn sign_up_form(i18n: I18n) -> Response<'static> {
    let res = render_template(&aardwolf_templates::SignUp::new(
        &i18n.catalog,
        "csrf token",
        "",
        None,
        false,
    ));

    drop(i18n);

    res
}

#[get("/sign_in")]
pub fn sign_in_form(i18n: I18n) -> Response<'static> {
    let res = render_template(&aardwolf_templates::SignIn::new(
        &i18n.catalog,
        "csrf token",
        "",
        None,
        false,
    ));

    drop(i18n);

    res
}

#[post("/sign_up", data = "<form>")]
pub fn sign_up(form: Form<SignUpForm>, i18n: I18n, db: DbConn) -> ResponseOrRedirect {
    let sign_up_form = form.into_inner();
    let form_state = sign_up_form.as_state();

    let res = perform!(&db, SignUpFail, [
        (form = ValidateSignUpForm(sign_up_form)),
        (_ = SignUp(form)),
    ]);

    let res = match res {
        Ok((email, token)) => {
            // just printing this out for now so we can copy/paste into the browser to confirm accounts,
            // but obviously this will need to be emailed to the user
            println!(
                "confirmation token url: /auth/confirmation?id={}&token={}",
                email.id(),
                token
            );

            Redirect::to("/auth/sign_in").into()
        }
        Err(e) => {
            let (status, valid, system) = match e {
                SignUpFail::ValidationError(ref e) => (Status::BadRequest, Some(e), false),
                _ => (Status::InternalServerError, None, true),
            };

            let mut response = render_template(&aardwolf_templates::SignUp::new(
                &i18n.catalog,
                "csrf token",
                &form_state.email,
                valid,
                system,
            ));
            response.set_status(status);
            response.into()
        }
    };

    drop(i18n);

    res
}

#[post("/sign_in", data = "<form>")]
pub fn sign_in(
    form: Form<SignInForm>,
    db: DbConn,
    mut cookies: Cookies,
    i18n: I18n,
) -> ResponseOrRedirect {
    // TODO: check csrf token (this will probably be a request guard)
    let form = form.into_inner();
    let form_state = form.as_state();

    let res = perform!(&db, SignInFail, [
        (form = ValidateSignInForm(form)),
        (_ = SignIn(form)),
    ]);

    let res = match res {
        Ok(user) => {
            let mut cookie = Cookie::new("user_id", format!("{}", user.id()));
            cookie.set_http_only(true);
            cookies.add_private(cookie);
            Redirect::to("/").into()
        }
        Err(e) => {
            let (status, valid, system) = match e {
                SignInFail::ValidationError(ref e) => (Status::BadRequest, Some(e), false),
                _ => (Status::InternalServerError, None, true),
            };

            let mut response = render_template(&aardwolf_templates::SignIn::new(
                &i18n.catalog,
                "csrf token",
                &form_state.email,
                valid,
                system,
            ));
            response.set_status(status);
            response.into()
        }
    };

    drop(i18n);

    res
}

#[get("/confirmation?<token..>")]
pub fn confirm(token: Form<ConfirmationToken>, db: DbConn) -> Result<Redirect, ConfirmAccountFail> {
    let res = perform!(&db, ConfirmAccountFail, [
       (_ = ConfirmToken(token.into_inner())),
    ]);

    drop(db);

    Ok(match res {
        Ok(_) => Redirect::to("/auth/sign_in"),
        Err(e) => {
            println!("unable to confirm account: {}, {:?}", e, e);
            return Err(e);
        }
    })
}

#[get("/sign_out")]
pub fn sign_out(_user: SignedInUser, mut cookies: Cookies) -> Redirect {
    drop(_user);
    cookies.remove_private(Cookie::named("user_id"));
    Redirect::to("/auth/sign_in")
}

#[get("/sign_out", rank = 2)]
pub fn already_signed_out() -> Redirect {
    Redirect::to("/auth/sign_in")
}

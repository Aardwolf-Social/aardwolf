use aardwolf_models::user::UserLike;
use aardwolf_types::forms::auth::{
    ConfirmAccountFail, ConfirmToken, ConfirmationToken, SignIn, SignInErrorMessage, SignInFail,
    SignInForm, SignUp, SignUpFail, SignUpForm, ValidateSignInForm, ValidateSignUpForm,
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

#[derive(Responder)]
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
    render_template(move |buf| {
        templates::sign_up(
            buf,
            aardwolf_templates::SignUp::new(&i18n.catalog, "csrf token", "", None, false),
        )
    })
}

#[get("/sign_in?<error..>")]
pub fn sign_in_form_with_error(i18n: I18n, error: Form<SignInErrorMessage>) -> Response<'static> {
    let error = error.into_inner();
    render_template(move |buf| {
        templates::sign_in(buf, &i18n.catalog, "csrf token", Some(error.clone()))
    })
}

#[get("/sign_in")]
pub fn sign_in_form(i18n: I18n) -> Response<'static> {
    render_template(move |buf| templates::sign_in(buf, &i18n.catalog, "csrf token", None))
}

#[post("/sign_up", data = "<form>")]
pub fn sign_up(form: Form<SignUpForm>, i18n: I18n, db: DbConn) -> ResponseOrRedirect {
    let sign_up_form = form.into_inner();
    let form_state = sign_up_form.as_state();

    let res = perform!(&db, SignUpFail, [
        (form = ValidateSignUpForm(sign_up_form)),
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

            Redirect::to("/auth/sign_in").into()
        }
        Err(e) => match e {
            SignUpFail::ValidationError(e) => render_template(move |buf| {
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
            .into(),
            _ => render_template(move |buf| {
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
            })
            .into(),
        },
    }
}

#[post("/sign_in", data = "<form>")]
pub fn sign_in(form: Form<SignInForm>, db: DbConn, mut cookies: Cookies) -> Redirect {
    // TODO: check csrf token (this will probably be a request guard)

    let res = perform!(&db, SignInFail, [
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
            // TODO: Percent Encode the error
            Redirect::to(format!("/auth/sign_in?msg=Unable%20to%20log%20in"))
        }
    }
}

#[get("/confirmation?<token..>")]
pub fn confirm(token: Form<ConfirmationToken>, db: DbConn) -> Result<Redirect, ConfirmAccountFail> {
    let res = perform!(&db, ConfirmAccountFail, [
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

#[get("/sign_out")]
pub fn sign_out(_user: SignedInUser, mut cookies: Cookies) -> Redirect {
    cookies.remove_private(Cookie::named("user_id"));
    Redirect::to("/auth/sign_in")
}

#[get("/sign_out", rank = 2)]
pub fn already_signed_out() -> Redirect {
    Redirect::to("/auth/sign_in")
}

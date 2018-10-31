use rocket::{
    http::{Cookie, Cookies},
    request::Form,
    response::Redirect,
};
use rocket_contrib::Template;

use aardwolf_types::forms::{
    auth::{ConfirmToken, SignInError, SignInForm, SignUpError, SignUpForm},
    traits::{DbAction, Validate},
};
use types::user::SignedInUser;
use DbConn;

#[get("/sign_up?<error>")]
fn sign_up_form_with_error(error: SignUpError) -> Template {
    let token = "some csrf token";
    Template::render(
        "sign_up",
        hashmap!{ "token" => token, "error_msg" => error.msg.as_str() },
    )
}

#[get("/sign_up")]
fn sign_up_form() -> Template {
    let token = "some csrf token";
    Template::render("sign_up", hashmap!{ "token" => token })
}

#[get("/sign_in?<error>")]
fn sign_in_form_with_error(error: SignInError) -> Template {
    let token = "some csrf token";
    Template::render(
        "sign_in",
        hashmap!{ "token" => token, "error_msg" => error.msg.as_str() },
    )
}

#[get("/sign_in")]
fn sign_in_form() -> Template {
    let token = "some csrf token";
    Template::render("sign_in", hashmap!{ "token" => token })
}

#[post("/sign_up", data = "<form>")]
fn sign_up(form: Form<SignUpForm>, db: DbConn) -> Redirect {
    // validation goes here

    let res = form
        .into_inner()
        .validate()
        .map_err(From::from)
        .and_then(|form| form.db_action(&db));

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
            Redirect::to(&format!("/auth/sign_up?msg={}", e))
        }
    }
}

#[post("/sign_in", data = "<form>")]
fn sign_in(form: Form<SignInForm>, db: DbConn, mut cookies: Cookies) -> Redirect {
    use aardwolf_models::user::UserLike;

    // TODO: check csrf token (this will probably be a request guard)

    let res = form
        .into_inner()
        .validate()
        .map_err(From::from)
        .and_then(|form| form.sign_in(&db));

    match res {
        Ok(user) => {
            let mut cookie = Cookie::new("user_id", format!("{}", user.id()));
            cookie.set_http_only(true);
            cookies.add_private(cookie);
            Redirect::to("/")
        }
        Err(e) => {
            println!("unable to log in: {}, {:?}", e, e);
            Redirect::to(&format!("/auth/sign_in?msg={}", e))
        }
    }
}

#[derive(Debug, Fail)]
#[fail(display = "Failed to confirm account")]
pub struct ConfirmError;

#[get("/confirmation?<token>")]
fn confirm(token: ConfirmToken, db: DbConn) -> Result<Redirect, ConfirmError> {
    Ok(match token.confirm_account(&db) {
        Ok(_) => Redirect::to("/auth/sign_in"),
        Err(e) => {
            println!("unable to confirm account: {}, {:?}", e, e);
            return Err(ConfirmError);
        }
    })
}

#[post("/sign_out")]
fn sign_out(_user: SignedInUser, mut cookies: Cookies) -> Redirect {
    cookies.remove_private(Cookie::named("user_id"));
    Redirect::to("/auth/sign_in")
}

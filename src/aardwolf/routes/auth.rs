use ring::rand::SystemRandom;
use rocket::State;
use rocket::http::{Cookie, Cookies};
use rocket::response::Redirect;
use rocket::request::Form;
use rocket_contrib::Template;

use DbConn;
use models::user::User;
use forms::auth::{SignUpForm, SignInForm};

#[derive(FromForm)]
struct SignUpError {
    msg: String
}

#[get("/sign_up?<error>")]
fn sign_up_form_with_error(error: SignUpError) -> Template {
    let token = "some csrf token";
    Template::render("sign_up", hashmap!{ "token" => token, "error_msg" => error.msg.as_str() })
}

#[get("/sign_up")]
fn sign_up_form() -> Template {
    let token = "some csrf token";
    Template::render("sign_up", hashmap!{ "token" => token })
}

#[derive(FromForm)]
struct SignInError {
    msg: String
}

#[get("/sign_in?<error>")]
fn sign_in_form_with_error(error: SignInError) -> Template {
    let token = "some csrf token";
    Template::render("sign_in", hashmap!{ "token" => token, "error_msg" => error.msg.as_str() })
}

#[get("/sign_in")]
fn sign_in_form() -> Template {
    let token = "some csrf token";
    Template::render("sign_in", hashmap!{ "token" => token })
}

#[post("/sign_up", data = "<form>")]
fn sign_up(form: Form<SignUpForm>, gen: State<SystemRandom>, db: DbConn) -> Redirect {
    use controllers::auth;

    match auth::create_user_and_account(form.into_inner(), gen.inner(), &db) {
        Ok(_) => Redirect::to("/sign_in"),
        Err(e) => {
            println!("unable to create account: {:#?}", e);
            Redirect::to(&format!("/sign_up?msg={}", e))
        }
    }
}

#[post("/sign_in", data = "<form>")]
fn sign_in(form: Form<SignInForm>, db: DbConn, mut cookies: Cookies) -> Redirect {
    use controllers::auth;
    match auth::sign_in(&form.into_inner(), &db) {
        Ok(user) => {
            let mut cookie = Cookie::new("user_id", user.id.to_string());
            cookie.set_http_only(true);
            cookies.add_private(cookie);
            Redirect::to("/")
        },
        Err(e) => {
            println!("unable to log in: {:#?}", e);
            Redirect::to(&format!("/sign_in?msg={}", e))
        }
    }
}

#[derive(FromForm)]
struct ConfirmToken {
    pub token: String,
}

#[derive(Debug, Fail)]
#[fail(display = "Failed to confirm account")]
struct ConfirmError;

#[get("/confirmation?<token>")]
fn confirm(token: ConfirmToken, db: DbConn) -> Result<Redirect, ConfirmError> {
    use controllers::auth;

    Ok(match auth::confirm_account(&token.token, &db) {
        Ok(_) => Redirect::to("/sign_in"),
        Err(e) => {
            println!("unable to confirm account: {:#?}", e);
            return Err(ConfirmError);
        }
    })
}

#[post("/sign_out")]
fn sign_out(_user: User, mut cookies: Cookies) -> Redirect {
    cookies.remove_private(Cookie::named("user_id"));
    Redirect::to("/sign_in")
}

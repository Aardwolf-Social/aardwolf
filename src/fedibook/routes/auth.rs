use ring::rand::SystemRandom;
use rocket::State;
use rocket::http::{Cookie, Cookies};
use rocket::response::{self, Redirect};
use rocket::request::Form;
use rocket_contrib::Template;
use diesel::pg::PgConnection;
use r2d2;
use r2d2_diesel::ConnectionManager;

use DbConn;
use models::user::User;
use forms::auth::{SignUpForm, SignInForm};

#[get("/auth/sign_up")]
fn sign_up_form() -> Template {
    let token = "some csrf token";
    Template::render("sign_up", hashmap!{ "token" => token })
}

#[get("/auth/sign_in")]
fn sign_in_form() -> Template {
    let token = "some csrf token";
    Template::render("sign_in", hashmap!{ "token" => token })
}

#[post("/auth", data = "<form>")]
fn sign_up(form: Form<SignUpForm>, gen: State<SystemRandom>, db: DbConn) -> Redirect {
    use controllers::auth;

    match auth::create_user_and_account(form.into_inner(), gen.inner(), &db) {
        Ok(_) => Redirect::to("/auth/sign_in"),
        Err(e) => {
            // this is obviously inadequate for now, we'll need
            // to send an error message up to the user as well
            println!("unable to create account: {:#?}", e);
            Redirect::to("/auth/sign_up")
        }
    }
}

#[post("/auth/sign_in", data = "<form>")]
fn sign_in(form: Form<SignInForm>, db: DbConn, mut cookies: Cookies) -> Redirect {
    use controllers::auth;
    match auth::sign_in(&form.into_inner(), &db) {
        Ok(user) => {
            let mut cookie = Cookie::new("user_id", user.id.to_string());
            cookie.set_http_only(true);
            cookies.add_private(cookie);
            Redirect::to("/web")
        },
        Err(e) => {
            println!("unable to log in: {:#?}", e);
            Redirect::to("/auth/sign_in")
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

#[get("/auth/confirmation?<token>")]
fn confirm(token: ConfirmToken, db: DbConn) -> Result<Redirect, ConfirmError> {
    use controllers::auth;

    Ok(match auth::confirm_account(&token.token, &db) {
        Ok(_) => Redirect::to("/auth/sign_in"),
        Err(e) => {
            println!("unable to confirm account: {:#?}", e);
            return Err(ConfirmError);
        }
    })
}

#[post("/auth/sign_out")]
fn sign_out(user: User, mut cookies: Cookies) -> Redirect {
    cookies.remove_private(Cookie::named("user_id"));
    Redirect::to("/auth/sign_in")
}

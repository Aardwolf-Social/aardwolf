use rocket::http::{Cookie, Cookies};
use rocket::response::{self, Redirect};
use rocket::request::Form;
use rocket_contrib::Template;

use models::user::User;
use DbConn;

#[get("/web")]
fn home(user: User, db: DbConn) -> Template {
    let map = hashmap!{
        "email" => user.email,
    };
    Template::render("home", map)
}

#[get("/web", rank = 2)]
fn home_redirect() -> Redirect {
    Redirect::to("/auth/sign_in")
}

use rocket::response::Redirect;
use rocket_contrib::Template;

use models::user::User;
use DbConn;

#[get("/")]
fn home(user: User, _db: DbConn) -> Template {
    let map = hashmap!{
        "email" => user.email,
    };
    Template::render("home", map)
}

#[get("/", rank = 2)]
fn home_redirect() -> Redirect {
    Redirect::to("/sign_in")
}

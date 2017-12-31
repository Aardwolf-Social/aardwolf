use rocket::response::Redirect;
use rocket_contrib::Template;

use models::user::User;
use DbConn;

#[get("/web")]
fn home(user: User, _db: DbConn) -> Template {
    let map = hashmap!{
        "email" => user.email,
    };
    Template::render("home", map)
}

#[get("/web", rank = 2)]
fn home_redirect() -> Redirect {
    Redirect::to("/auth/sign_in")
}

// Adding route to /
#[get("/")]
fn index() -> &'static str {
     "
      This is Banjo's Dummy Index because he's not sure how to use Template::render yet
      ...
      Soon though...maybe?
    "
}

use std::path::{Path, PathBuf};

use rocket::response::{NamedFile, Redirect};
use rocket::response::status::NotFound;
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
    Redirect::to("/auth/sign_in")
}

#[cfg(debug_assertions)]
#[get("/assets/<file..>")]
fn assets(file: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = Path::new("web/").join(file);
    NamedFile::open(&path).map_err(|_| NotFound(format!("Bad path: {:?}", path)))
}

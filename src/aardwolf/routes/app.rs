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

//
// These are specific routes for static asset folders
// ideally they will be handled by Nginx/Apache/Web server
// but for development purposes we can handle them in Rocket :D
//

// Web root
#[cfg(debug_assertions)]
#[get("/web/<file..>")]
fn webroot(file: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = Path::new("web/").join(file);
    NamedFile::open(&path).map_err(|_| NotFound(format!("Bad path: {:?}", path)))
}

// Emoji folder
#[cfg(debug_assertions)]
#[get("/emoji/<file..>")]
fn emoji(file: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = Path::new("emoji/").join(file);
    NamedFile::open(&path).map_err(|_| NotFound(format!("Bad path: {:?}", path)))
}

// Fork-Awesome folder
#[cfg(debug_assertions)]
#[get("/Fork-Awesome-1.0.10/<file..>")]
fn fork_awesome(file: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = Path::new("Fork-Awesome-1.0.10/").join(file);
    NamedFile::open(&path).map_err(|_| NotFound(format!("Bad path: {:?}", path)))
}

// Images folder
#[cfg(debug_assertions)]
#[get("/images/<file..>")]
fn images(file: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = Path::new("web/images/").join(file);
    NamedFile::open(&path).map_err(|_| NotFound(format!("Bad path: {:?}", path)))
}

// Javascript folder
#[cfg(debug_assertions)]
#[get("/javascript/<file..>")]
fn javascript(file: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = Path::new("web/javascript/").join(file);
    NamedFile::open(&path).map_err(|_| NotFound(format!("Bad path: {:?}", path)))
}

// Stylesheets folder
#[cfg(debug_assertions)]
#[get("/stylesheets/<file..>")]
fn stylesheets(file: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = Path::new("web/stylesheets/").join(file);
    NamedFile::open(&path).map_err(|_| NotFound(format!("Bad path: {:?}", path)))
}

// Themes folder
#[cfg(debug_assertions)]
#[get("/themes/<file..>")]
fn themes(file: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = Path::new("web/themes/").join(file);
    NamedFile::open(&path).map_err(|_| NotFound(format!("Bad path: {:?}", path)))
}

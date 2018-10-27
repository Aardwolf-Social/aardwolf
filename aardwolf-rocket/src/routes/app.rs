use std::path::{Path, PathBuf};

use rocket::response::{
    status::NotFound,
    {NamedFile, Redirect},
};
use rocket_contrib::Template;

use aardwolf_types::SignedInUserWithEmail;
use DbConn;

#[get("/")]
fn home(user: SignedInUserWithEmail, _db: DbConn) -> Template {
    let map = hashmap!{
        "email" => user.1.to_verified()
            .map(|verified| verified.email().to_owned())
            .unwrap_or_else(|unverified| unverified.email().to_owned()),
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
    let path = Path::new("dist/").join(file);
    NamedFile::open(&path).map_err(|_| NotFound(format!("Bad path: {:?}", path)))
}

// Emoji folder
#[cfg(debug_assertions)]
#[get("/emoji/<file..>")]
fn emoji(file: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = Path::new("emoji/").join(file);
    NamedFile::open(&path).map_err(|_| NotFound(format!("Bad path: {:?}", path)))
}

// Themes folder
#[cfg(debug_assertions)]
#[get("/themes/<file..>")]
fn themes(file: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = Path::new("web/themes/").join(file);
    NamedFile::open(&path).map_err(|_| NotFound(format!("Bad path: {:?}", path)))
}

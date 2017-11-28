#![feature(try_from)]
#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate failure;
extern crate rocket_contrib;
extern crate serde;

extern crate _fedibook as fedibook;

use rocket::Rocket;
use rocket_contrib::Template;

fn rocket() -> Rocket {
    rocket::ignite()
        .mount("/api/v1", routes![
            fedibook::routes::applications::register_application
        ])
        .mount("/", routes![
            fedibook::routes::auth::sign_up_form,
            fedibook::routes::auth::sign_in_form,
            fedibook::routes::auth::sign_up,
            fedibook::routes::auth::sign_in,
        ])
        .attach(Template::fairing())
}

fn main() {
    rocket().launch();
}


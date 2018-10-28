/// Routes for dealing with applications
use failure::Error;
use rocket_contrib::Json;

use controllers;
use aardwolf_types::{
    apps::{AppId},
    forms::{app::CreateApp, traits::Validate},
};

#[post("/apps", data = "<app>")]
fn register_application(app: Json<CreateApp>) -> Result<Json<AppId>, Error> {
    let app = app.into_inner();
    Ok(Json(controllers::apps::create(app.validate()?)?))
}

/// Routes for dealing with applications
use failure::Error;
use rocket_contrib::Json;

use aardwolf_types::{
    apps::{AppId, AppIdBuilder},
    forms::{app::CreateApp, traits::Validate},
};

#[post("/apps", data = "<app>")]
fn register_application(app: Json<CreateApp>) -> Result<Json<AppId>, Error> {
    let _ = app.into_inner().validate()?;

    Ok(Json(
        AppIdBuilder::default()
            .id("foo")
            .client_id("bar")
            .client_secret("baz")
            .build()
            .unwrap(),
    ))
}

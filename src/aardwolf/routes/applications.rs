/// Routes for dealing with applications
use failure::Error;
use rocket_contrib::Json;

use controllers;
use types::apps::{App, AppId};
use types::scope::Scope;

mod deser_scope {
    use serde::{self, Deserialize, Deserializer};
    use types::scope::Scope;

    pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Scope, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse::<Scope>().map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Fail)]
enum CreateAppError {
    #[fail(display = "validation error when creating app")]
    ValidationError,
}

/// Represents the form that is POSTed to /api/v1/apps to create an application
#[derive(Deserialize)]
struct CreateApp {
    client_name: String,
    redirect_uris: String,
    #[serde(with = "deser_scope")]
    scopes: Scope,
    website: Option<String>,
}

impl CreateApp {
    fn validate(&self) -> Result<(), CreateAppError> {
        if self.client_name.is_empty() || self.client_name.len() > 256 {
            return Err(CreateAppError::ValidationError);
        }

        if self.redirect_uris.is_empty() {
            return Err(CreateAppError::ValidationError);
        }

        Ok(())
    }
}

impl<'a, 'b: 'a> From<&'b CreateApp> for App<'a> {
    fn from(app: &'b CreateApp) -> App<'a> {
        App {
            client_name: &app.client_name[..],
            redirect_uris: &app.redirect_uris[..],
            scopes: &app.scopes,
            website: &app.website,
        }
    }
}

#[post("/apps", data = "<app>")]
fn register_application(app: Json<CreateApp>) -> Result<Json<AppId>, Error> {
    let app = app.into_inner();
    app.validate()?;
    Ok(Json(controllers::apps::create(App::from(&app))?))
}

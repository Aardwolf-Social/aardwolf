use models::apps::{App, AppId, AppIdBuilder};

#[derive(Fail, Debug)]
#[fail(display = "Failed to create app.")]
pub(crate) struct CreateAppError;

pub(crate) fn create(app: App) -> Result<AppId, CreateAppError> {
    // store the app somewhere
    Ok(AppIdBuilder::default()
        .id("foo")
        .client_id("bar")
        .client_secret("baz")
        .build()
        .unwrap())
}

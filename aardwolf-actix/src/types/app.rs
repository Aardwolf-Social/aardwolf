use actix_web::{dev::FormConfig, Form, FromRequest, HttpRequest, State};
use futures::Future;

use aardwolf_types::{
    apps::App,
    forms::{app::CreateApp, traits::Validate},
};

use crate::{error::ErrorWrapper, AppConfig};

pub struct AppWrapper(pub App);

impl FromRequest<AppConfig> for AppWrapper {
    type Config = ();
    type Result = Box<dyn Future<Item = Self, Error = actix_web::error::Error>>;

    fn from_request(req: &HttpRequest<AppConfig>, _: &Self::Config) -> Self::Result {
        let state: State<AppConfig> = State::from_request(req, &Default::default());

        Box::new(Form::from_request(req, &FormConfig::default()).and_then(
            move |form: Form<CreateApp>| {
                form.into_inner()
                    .validate()
                    .map(AppWrapper)
                    .map_err(move |e| ErrorWrapper::new(state.clone(), e).into())
            },
        ))
    }
}

use aardwolf_types::forms::{
    personas::{PersonaCreationForm, ValidatedPersonaCreationForm},
    traits::Validate,
};
use actix_web::{dev::FormConfig, Form, FromRequest, HttpRequest, State};
use futures::Future;

use crate::{error::ErrorWrapper, AppConfig};

pub struct ValidPersonaCreationForm(pub ValidatedPersonaCreationForm);

impl FromRequest<AppConfig> for ValidPersonaCreationForm {
    type Config = ();
    type Result = Box<dyn Future<Item = Self, Error = actix_web::error::Error>>;

    fn from_request(req: &HttpRequest<AppConfig>, _: &Self::Config) -> Self::Result {
        let state = State::from_request(req, &Default::default());

        Box::new(Form::from_request(req, &FormConfig::default()).and_then(
            |form: Form<PersonaCreationForm>| {
                form.into_inner()
                    .validate()
                    .map(ValidPersonaCreationForm)
                    .map_err(move |e| ErrorWrapper::new(state.clone(), e).into())
            },
        ))
    }
}

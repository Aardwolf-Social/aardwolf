use aardwolf_types::forms::{
    auth::{SignInForm, SignUpForm, ValidatedSignInForm, ValidatedSignUpForm},
    traits::Validate,
};
use actix_web::{dev::FormConfig, Form, FromRequest, HttpRequest, State};
use futures::Future;

use crate::{error::ErrorWrapper, AppConfig};

pub struct ValidSignInForm(pub ValidatedSignInForm);
pub struct ValidSignUpForm(pub ValidatedSignUpForm);

impl FromRequest<AppConfig> for ValidSignInForm {
    type Config = ();
    type Result = Box<dyn Future<Item = Self, Error = actix_web::error::Error>>;

    fn from_request(req: &HttpRequest<AppConfig>, _: &Self::Config) -> Self::Result {
        let state = State::from_request(req, &Default::default());

        Box::new(Form::from_request(req, &FormConfig::default()).and_then(
            |form: Form<SignInForm>| {
                form.into_inner()
                    .validate()
                    .map(ValidSignInForm)
                    .map_err(move |e| ErrorWrapper::new(state.clone(), e).into())
            },
        ))
    }
}

impl FromRequest<AppConfig> for ValidSignUpForm {
    type Config = ();
    type Result = Box<dyn Future<Item = Self, Error = actix_web::error::Error>>;

    fn from_request(req: &HttpRequest<AppConfig>, _: &Self::Config) -> Self::Result {
        let state = State::from_request(req, &Default::default());

        Box::new(Form::from_request(req, &FormConfig::default()).and_then(
            |form: Form<SignUpForm>| {
                form.into_inner()
                    .validate()
                    .map(ValidSignUpForm)
                    .map_err(move |e| ErrorWrapper::new(state.clone(), e).into())
            },
        ))
    }
}

use crate::{apps::App, forms::traits::Validate, scope::Scope};

#[derive(Debug, Fail)]
pub enum CreateAppError {
    #[fail(display = "validation error when creating app")]
    ValidationError,
}

/// Represents the form that is POSTed to /api/v1/apps to create an application
#[derive(Deserialize)]
pub struct CreateApp {
    client_name: String,
    redirect_uris: String,
    scopes: Scope,
    website: Option<String>,
}

impl Validate<App, CreateAppError> for CreateApp {
    fn validate(self) -> Result<App, CreateAppError> {
        if self.client_name.is_empty() || self.client_name.len() > 256 {
            return Err(CreateAppError::ValidationError);
        }

        if self.redirect_uris.is_empty() {
            return Err(CreateAppError::ValidationError);
        }

        let CreateApp {
            client_name,
            redirect_uris,
            scopes,
            website,
        } = self;

        Ok(App {
            client_name,
            redirect_uris,
            scopes,
            website,
        })
    }
}

#[cfg(feature = "use-actix")]
mod actix {
    use actix_web::{dev::FormConfig, error::ResponseError, Form, FromRequest, HttpRequest};
    use futures::Future;

    use crate::{
        apps::App,
        forms::{
            app::{CreateApp, CreateAppError},
            traits::Validate,
        },
    };

    impl ResponseError for CreateAppError {}

    impl<S> FromRequest<S> for App
    where
        S: 'static,
    {
        type Config = ();
        type Result = Box<dyn Future<Item = Self, Error = actix_web::error::Error>>;

        fn from_request(req: &HttpRequest<S>, _: &Self::Config) -> Self::Result {
            Box::new(
                Form::from_request(req, &FormConfig::default()).and_then(
                    |form: Form<CreateApp>| form.into_inner().validate().map_err(From::from),
                ),
            )
        }
    }
}

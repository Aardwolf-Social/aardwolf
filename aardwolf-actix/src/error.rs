use aardwolf_models::user::PermissionError;
use aardwolf_types::{
    error::{AardwolfError, AardwolfErrorKind, ErrorJson, TemplateError, TemplateName},
    forms::{
        app::CreateAppError,
        auth::{
            ConfirmAccountFail, SignInFail, SignInFormValidationFail, SignUpFail,
            SignUpFormValidationFail,
        },
        personas::{PersonaCreationFail, PersonaDeletionFail, PersonaLookupError},
        user::UserLookupFail,
    },
};
use actix_web::{
    http::header::{CONTENT_TYPE, LOCATION},
    HttpResponse, ResponseError,
};

use crate::{db::DbActionError, AppConfig};

pub type RenderResult = Result<HttpResponse, RenderError>;

#[derive(Clone, Debug, Fail)]
#[fail(display = "Could not render template")]
pub struct RenderError;

impl ResponseError for RenderError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::SeeOther().header(LOCATION, "/").finish()
    }
}

#[derive(Debug, Fail)]
#[fail(display = "Redirect to {}", _0)]
pub struct RedirectError(String, Option<String>);

impl RedirectError {
    pub fn new(s: &str, msg: Option<&str>) -> Self {
        RedirectError(s.to_owned(), msg.map(|m| m.to_owned()))
    }
}

impl ResponseError for RedirectError {
    fn error_response(&self) -> HttpResponse {
        let msg = self
            .1
            .as_ref()
            .map(|m| format!("?msg={}", m))
            .unwrap_or("".to_owned());
        let location = format!("{}{}", self.0, msg);

        println!("Redirecting to {}", location);

        HttpResponse::SeeOther()
            .header(LOCATION, location.as_str())
            .finish()
    }
}

#[derive(Clone, Debug, Fail)]
#[fail(display = "{}", _1)]
pub struct ErrorWrapper<E>(AppConfig, pub E)
where
    E: AardwolfError;

impl<E> ErrorWrapper<E>
where
    E: AardwolfError,
{
    pub fn new(state: AppConfig, error: E) -> Self {
        ErrorWrapper(state, error)
    }
}

impl<E> AardwolfError for ErrorWrapper<E>
where
    E: AardwolfError,
{
    fn name(&self) -> &str {
        self.1.name()
    }

    fn kind(&self) -> AardwolfErrorKind {
        self.1.kind()
    }

    fn description(&self) -> String {
        self.1.description()
    }
}

impl TemplateError for ErrorWrapper<CreateAppError> {
    fn template(&self) -> TemplateName {
        TemplateName::new("sign_up")
    }
}

impl TemplateError for ErrorWrapper<SignInFormValidationFail> {
    fn template(&self) -> TemplateName {
        TemplateName::new("sign_in")
    }
}

impl TemplateError for ErrorWrapper<SignUpFormValidationFail> {
    fn template(&self) -> TemplateName {
        TemplateName::new("sign_up")
    }
}

impl TemplateError for ErrorWrapper<PersonaCreationFail> {
    fn template(&self) -> TemplateName {
        TemplateName::new("TODO")
    }
}

impl TemplateError for ErrorWrapper<DbActionError<PersonaCreationFail>> {
    fn template(&self) -> TemplateName {
        TemplateName::new("TODO")
    }
}

impl TemplateError for ErrorWrapper<DbActionError<PersonaLookupError>> {
    fn template(&self) -> TemplateName {
        TemplateName::new("TODO")
    }
}

impl TemplateError for ErrorWrapper<DbActionError<PermissionError>> {
    fn template(&self) -> TemplateName {
        TemplateName::new("TODO")
    }
}

impl TemplateError for ErrorWrapper<DbActionError<PersonaDeletionFail>> {
    fn template(&self) -> TemplateName {
        TemplateName::new("TODO")
    }
}

impl TemplateError for ErrorWrapper<DbActionError<ConfirmAccountFail>> {
    fn template(&self) -> TemplateName {
        TemplateName::new("TODO")
    }
}

impl TemplateError for ErrorWrapper<DbActionError<SignUpFail>> {
    fn template(&self) -> TemplateName {
        TemplateName::new("sign_up")
    }
}

impl TemplateError for ErrorWrapper<DbActionError<SignInFail>> {
    fn template(&self) -> TemplateName {
        TemplateName::new("sign_in")
    }
}

impl TemplateError for ErrorWrapper<DbActionError<UserLookupFail>> {
    fn template(&self) -> TemplateName {
        TemplateName::new("TODO")
    }
}

impl<E> ResponseError for ErrorWrapper<E>
where
    Self: TemplateError,
    E: AardwolfError + Clone,
{
    fn error_response(&self) -> HttpResponse {
        let mut res = match self.1.kind() {
            AardwolfErrorKind::Redirect(location) => {
                let mut res = HttpResponse::SeeOther();
                res.header(LOCATION, location);
                res
            }
            AardwolfErrorKind::BadRequest => HttpResponse::BadRequest(),
            AardwolfErrorKind::RequiresAuthentication => HttpResponse::Unauthorized(),
            AardwolfErrorKind::RequiresPermission => HttpResponse::Forbidden(),
            AardwolfErrorKind::NotFound => HttpResponse::NotFound(),
            AardwolfErrorKind::InternalServerError => HttpResponse::InternalServerError(),
        };

        let body = self
            .0
            .templates
            .render(self.template().name(), &ErrorJson::from(self.1.clone()))
            .unwrap_or("Failed to render template".to_owned());

        res.header(CONTENT_TYPE, "text/html").body(body)
    }
}

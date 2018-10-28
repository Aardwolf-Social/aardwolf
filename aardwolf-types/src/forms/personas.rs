use aardwolf_models::{
    base_actor::{persona::Persona, BaseActor},
    sql_types::{FollowPolicy, PostVisibility, Url},
    user::{PermissionError, PermissionedUser},
};
use diesel::{pg::PgConnection, result::Error as DieselError};
use url::ParseError as UrlParseError;

use crate::forms::traits::Validate;

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "use-rocket", derive(FromForm))]
pub struct PersonaCreationForm {
    display_name: String,
    follow_policy: FollowPolicy,
    default_visibility: PostVisibility,
    shortname: String,
    is_searchable: bool,
}

impl Validate<ValidatedPersonaCreationForm, PersonaCreationFail> for PersonaCreationForm {
    fn validate(self) -> Result<ValidatedPersonaCreationForm, PersonaCreationFail> {
        if self.display_name.is_empty() {
            return Err(PersonaCreationFail);
        }

        if self.shortname.is_empty() {
            return Err(PersonaCreationFail);
        }

        Ok(ValidatedPersonaCreationForm {
            display_name: self.display_name,
            follow_policy: self.follow_policy,
            profile_url: format!("https://localhost:8000/users/{}", self.shortname).parse()?,
            inbox_url: format!("https://localhost:8000/users/{}/inbox", self.shortname).parse()?,
            outbox_url: format!("https://localhost:8000/users/{}/outbox", self.shortname)
                .parse()?,
            default_visibility: self.default_visibility,
            shortname: self.shortname,
            is_searchable: self.is_searchable,
        })
    }
}

#[derive(Debug, Fail)]
#[fail(display = "Failed to validate persona")]
pub struct PersonaCreationFail;

impl From<UrlParseError> for PersonaCreationFail {
    fn from(_: UrlParseError) -> Self {
        PersonaCreationFail
    }
}

impl From<DieselError> for PersonaCreationFail {
    fn from(_: DieselError) -> Self {
        PersonaCreationFail
    }
}

impl From<PermissionError> for PersonaCreationFail {
    fn from(_: PermissionError) -> Self {
        PersonaCreationFail
    }
}

pub struct ValidatedPersonaCreationForm {
    display_name: String,
    follow_policy: FollowPolicy,
    profile_url: Url,
    inbox_url: Url,
    outbox_url: Url,
    default_visibility: PostVisibility,
    shortname: String,
    is_searchable: bool,
}

impl ValidatedPersonaCreationForm {
    pub fn create<U: PermissionedUser>(
        self,
        user: &U,
        db: &PgConnection,
    ) -> Result<(BaseActor, Persona), PersonaCreationFail> {
        let persona_maker = user.can_make_persona(db)?;

        Ok(persona_maker.create_persona(
            self.display_name,
            self.profile_url,
            self.inbox_url,
            self.outbox_url,
            self.follow_policy,
            self.default_visibility,
            self.is_searchable,
            None,
            self.shortname,
            db,
        )?)
    }
}

#[cfg(feature = "use-actix")]
mod actix {
    use actix_web::{dev::FormConfig, error::ResponseError, Form, FromRequest, HttpRequest};
    use futures::Future;

    use crate::forms::{
        personas::{PersonaCreationFail, PersonaCreationForm, ValidatedPersonaCreationForm},
        traits::Validate,
    };

    impl ResponseError for PersonaCreationFail {}

    impl<S> FromRequest<S> for ValidatedPersonaCreationForm
    where
        S: 'static,
    {
        type Config = ();
        type Result = Box<dyn Future<Item = Self, Error = actix_web::error::Error>>;

        fn from_request(req: &HttpRequest<S>, _: &Self::Config) -> Self::Result {
            Box::new(Form::from_request(req, &FormConfig::default()).and_then(
                |form: Form<PersonaCreationForm>| form.into_inner().validate().map_err(From::from),
            ))
        }
    }
}

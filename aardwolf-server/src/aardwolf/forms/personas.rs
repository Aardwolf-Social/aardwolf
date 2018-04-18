use aardwolf_models::{base_actor::{BaseActor, persona::Persona},
                      sql_types::{FollowPolicy, PostVisibility, Url},
                      user::{PermissionError, PermissionedUser}};
use diesel::{pg::PgConnection, result::Error as DieselError};
use url::ParseError as UrlParseError;

use forms::traits::Validate;

/*
 * BaseActor
 *
 * display_name: String,
 * profile_url: OrigUrl,
 * inbox_url: OrigUrl,
 * outbox_url: OrigUrl,
 * local_user: Option<&U>,
 * follow_policy: FollowPolicy,
 * original_json: Value,
 *
 * Persona
 *
 * default_visibility: PostVisibility,
 * is_searchable: bool,
 * avatar: Option<&Image>,
 * shortname: String,
 * base_actor: &BaseActor,
 */
#[derive(Debug, FromForm)]
pub(crate) struct PersonaCreationForm {
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
            outbox_url: format!("https://localhost:8000/users/{}/outbox", self.shortname).parse()?,
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

        persona_maker
            .create_persona(
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
            )
            .map_err(From::from)
    }
}

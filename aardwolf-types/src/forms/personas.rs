use aardwolf_models::{
    base_actor::{persona::Persona, BaseActor},
    sql_types::{FollowPolicy, PostVisibility, Url},
    user::{AuthenticatedUser, PermissionError, PermissionedUser, PersonaDeleter},
};
use diesel::{pg::PgConnection, result::Error as DieselError};
use url::ParseError as UrlParseError;

use crate::{
    error::{AardwolfError, AardwolfErrorKind},
    forms::traits::{DbAction, Validate},
};

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
            return Err(PersonaCreationFail::Validation);
        }

        if self.shortname.is_empty() {
            return Err(PersonaCreationFail::Validation);
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

#[derive(Clone, Debug, Fail)]
pub enum PersonaCreationFail {
    #[fail(display = "Failed to validate persona")]
    Validation,
    #[fail(display = "User doesn't have permission to create persona")]
    Permission,
    #[fail(display = "Error in database")]
    Database,
}

impl AardwolfError for PersonaCreationFail {
    fn name(&self) -> &str {
        "Persona Creation Fail"
    }

    fn kind(&self) -> AardwolfErrorKind {
        match *self {
            PersonaCreationFail::Validation => AardwolfErrorKind::BadRequest,
            PersonaCreationFail::Permission => AardwolfErrorKind::RequiresPermission,
            PersonaCreationFail::Database => AardwolfErrorKind::InternalServerError,
        }
    }

    fn description(&self) -> String {
        format!("{}", self)
    }
}

impl From<UrlParseError> for PersonaCreationFail {
    fn from(_: UrlParseError) -> Self {
        PersonaCreationFail::Validation
    }
}

impl From<DieselError> for PersonaCreationFail {
    fn from(_: DieselError) -> Self {
        PersonaCreationFail::Database
    }
}

impl From<PermissionError> for PersonaCreationFail {
    fn from(e: PermissionError) -> Self {
        match e {
            PermissionError::Diesel => PersonaCreationFail::Database,
            PermissionError::Permission => PersonaCreationFail::Permission,
        }
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

    pub fn with<U>(self, user: U) -> PersonaCreationOperation<U>
    where
        U: PermissionedUser,
    {
        PersonaCreationOperation { form: self, user }
    }
}

pub struct PersonaCreationOperation<U>
where
    U: PermissionedUser,
{
    form: ValidatedPersonaCreationForm,
    user: U,
}

impl<U> DbAction<(BaseActor, Persona), PersonaCreationFail> for PersonaCreationOperation<U>
where
    U: PermissionedUser,
{
    fn db_action(self, conn: &PgConnection) -> Result<(BaseActor, Persona), PersonaCreationFail> {
        self.form.create(&self.user, conn)
    }
}

#[derive(Clone, Debug, Fail)]
pub enum PersonaLookupError {
    #[fail(display = "Error in database")]
    Database,
    #[fail(display = "Persona not found")]
    NotFound,
}

impl From<DieselError> for PersonaLookupError {
    fn from(e: DieselError) -> Self {
        match e {
            DieselError::NotFound => PersonaLookupError::NotFound,
            _ => PersonaLookupError::Database,
        }
    }
}

impl AardwolfError for PersonaLookupError {
    fn name(&self) -> &str {
        "Persona Lookup Error"
    }

    fn kind(&self) -> AardwolfErrorKind {
        match *self {
            PersonaLookupError::Database => AardwolfErrorKind::InternalServerError,
            PersonaLookupError::NotFound => AardwolfErrorKind::NotFound,
        }
    }

    fn description(&self) -> String {
        format!("{}", self)
    }
}

pub struct GetPersonaById;

impl GetPersonaById {
    pub fn new() -> Self {
        GetPersonaById
    }

    pub fn with(self, id: i32) -> PersonaGetter {
        PersonaGetter(id)
    }
}

pub struct PersonaGetter(i32);

impl DbAction<Persona, PersonaLookupError> for PersonaGetter {
    fn db_action(self, conn: &PgConnection) -> Result<Persona, PersonaLookupError> {
        Persona::by_id(self.0, conn).map_err(From::from)
    }
}

pub struct UserCanDeletePersona(AuthenticatedUser);

impl UserCanDeletePersona {
    pub fn new(user: AuthenticatedUser) -> Self {
        UserCanDeletePersona(user)
    }

    pub fn with(self, persona: Persona) -> UserPersonaDeleter {
        UserPersonaDeleter(self.0, persona)
    }
}

pub struct UserPersonaDeleter(AuthenticatedUser, Persona);

/// TODO: Use a different error kind here that's more specific
impl DbAction<PersonaDeleter, PermissionError> for UserPersonaDeleter {
    fn db_action(self, conn: &PgConnection) -> Result<PersonaDeleter, PermissionError> {
        self.0.can_delete_persona(self.1, conn)
    }
}

#[derive(Clone, Debug, Fail)]
pub enum PersonaDeletionFail {
    #[fail(display = "Insufficient permissions")]
    Permission,
    #[fail(display = "Error in database")]
    Database,
    #[fail(display = "Persona not found")]
    NotFound,
}

impl From<DieselError> for PersonaDeletionFail {
    fn from(e: DieselError) -> Self {
        match e {
            DieselError::NotFound => PersonaDeletionFail::NotFound,
            _ => PersonaDeletionFail::Database,
        }
    }
}

impl From<PermissionError> for PersonaDeletionFail {
    fn from(e: PermissionError) -> Self {
        match e {
            PermissionError::Permission => PersonaDeletionFail::Permission,
            PermissionError::Diesel => PersonaDeletionFail::Database,
        }
    }
}

impl From<PersonaLookupError> for PersonaDeletionFail {
    fn from(e: PersonaLookupError) -> Self {
        match e {
            PersonaLookupError::Database => PersonaDeletionFail::Database,
            PersonaLookupError::NotFound => PersonaDeletionFail::NotFound,
        }
    }
}

impl AardwolfError for PersonaDeletionFail {
    fn name(&self) -> &str {
        "Persona Deletion Error"
    }

    fn kind(&self) -> AardwolfErrorKind {
        match *self {
            PersonaDeletionFail::Permission => AardwolfErrorKind::RequiresPermission,
            PersonaDeletionFail::Database => AardwolfErrorKind::InternalServerError,
            PersonaDeletionFail::NotFound => AardwolfErrorKind::NotFound,
        }
    }

    fn description(&self) -> String {
        format!("{}", self)
    }
}

pub struct DeletePersona;

impl DeletePersona {
    pub fn new() -> Self {
        DeletePersona
    }

    pub fn with(self, persona_deleter: PersonaDeleter) -> Delete {
        Delete(persona_deleter)
    }
}

pub struct Delete(PersonaDeleter);

impl DbAction<(), PersonaDeletionFail> for Delete {
    fn db_action(self, conn: &PgConnection) -> Result<(), PersonaDeletionFail> {
        self.0.delete_persona(conn).map_err(From::from)
    }
}

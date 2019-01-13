use aardwolf_models::sql_types::{FollowPolicy, PostVisibility, Url};

use crate::{
    traits::{Validate},
    wrapper::{ValidateWrapper, Wrapped},
    error::AardwolfFail,
};

#[derive(Clone, Debug, Deserialize)]
#[cfg_attr(feature = "use-rocket", derive(FromForm))]
pub struct PersonaCreationForm {
    csrf_token: String,
    display_name: String,
    follow_policy: FollowPolicy,
    default_visibility: PostVisibility,
    shortname: String,
    #[serde(default)]
    is_searchable: bool,
}

pub struct PersonaCreationFormState {
    pub display_name: String,
    pub follow_policy: FollowPolicy,
    pub default_visibility: PostVisibility,
    pub shortname: String,
    pub is_searchable: bool,
}

impl PersonaCreationForm {
    pub fn as_state(&self) -> PersonaCreationFormState {
        PersonaCreationFormState {
            display_name: self.display_name.clone(),
            follow_policy: self.follow_policy,
            default_visibility: self.default_visibility,
            shortname: self.shortname.clone(),
            is_searchable: self.is_searchable,
        }
    }
}

#[derive(Clone, Debug, Fail, Serialize)]
#[fail(display = "Failed to validate persona creation form")]
pub struct ValidatePersonaCreationFail {
    pub display_name: Option<String>,
    pub follow_policy: Option<String>,
    pub default_visibility: Option<String>,
    pub shortname: Option<String>,
    pub is_searchable: Option<String>,
}

impl ValidatePersonaCreationFail {
    pub fn is_empty(&self) -> bool {
        self.display_name.is_none() && self.follow_policy.is_none() && self.default_visibility.is_none() && self.shortname.is_none() && self.is_searchable.is_none()
    }
}

impl AardwolfFail for ValidatePersonaCreationFail {}

pub struct ValidatePersonaCreationForm(pub PersonaCreationForm);

impl Wrapped for ValidatePersonaCreationForm {
    type Wrapper = ValidateWrapper<Self, <Self as Validate>::Item, <Self as Validate>::Error>;
}

impl Validate for ValidatePersonaCreationForm {
    type Item = ValidatedPersonaCreationForm;
    type Error = ValidatePersonaCreationFail;

    fn validate(self) -> Result<ValidatedPersonaCreationForm, ValidatePersonaCreationFail> {
        let mut err = ValidatePersonaCreationFail {
            display_name: None,
            follow_policy: None,
            default_visibility: None,
            shortname: None,
            is_searchable: None,
        };

        if self.0.display_name.is_empty() {
            err.display_name = Some("Display Name cannot be empty".to_owned());
        }

        if self.0.shortname.is_empty() {
            err.shortname = Some("Username cannot be empty".to_owned());
        }

        if self.0.shortname.chars().any(|c| !c.is_alphanumeric()) {
            err.shortname = Some("Username cannot contain special characters".to_owned());
        }

        if self.0.shortname.len() > 30 {
            err.shortname = Some("Username is too long".to_owned());
        }

        if !err.is_empty() {
            return Err(err);
        }

        // Local accounts get a generic URL, since their URLs should be generated using the
        // local_uuid field.
        let generic_url: Url = "https://example.com".parse().unwrap();

        Ok(ValidatedPersonaCreationForm {
            display_name: self.0.display_name,
            follow_policy: self.0.follow_policy,
            profile_url: generic_url.clone(),
            inbox_url: generic_url.clone(),
            outbox_url: generic_url,
            default_visibility: self.0.default_visibility,
            shortname: self.0.shortname,
            is_searchable: self.0.is_searchable,
        })
    }
}

pub struct ValidatedPersonaCreationForm {
    pub(crate) display_name: String,
    pub(crate) follow_policy: FollowPolicy,
    pub(crate) profile_url: Url,
    pub(crate) inbox_url: Url,
    pub(crate) outbox_url: Url,
    pub(crate) default_visibility: PostVisibility,
    pub(crate) shortname: String,
    pub(crate) is_searchable: bool,
}

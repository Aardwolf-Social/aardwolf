use aardwolf_models::sql_types::{FollowPolicy, PostVisibility};

use crate::{
    error::AardwolfFail,
    traits::Validate,
    wrapper::{ValidateWrapper, Wrapped},
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
    pub display_name: Option<ValidateDisplayNameFail>,
    pub follow_policy: Option<ValidateFollowPolicyFail>,
    pub default_visibility: Option<ValidateDefaultVisibilityFail>,
    pub shortname: Option<ValidateShortnameFail>,
    pub is_searchable: Option<ValidateIsSearchableFail>,
}

impl ValidatePersonaCreationFail {
    pub fn is_empty(&self) -> bool {
        self.display_name.is_none()
            && self.follow_policy.is_none()
            && self.default_visibility.is_none()
            && self.shortname.is_none()
            && self.is_searchable.is_none()
    }
}

impl AardwolfFail for ValidatePersonaCreationFail {}

#[derive(Clone, Debug, Serialize)]
pub enum ValidateDisplayNameFail {
    Empty,
}

#[derive(Clone, Debug, Serialize)]
pub enum ValidateFollowPolicyFail {}

#[derive(Clone, Debug, Serialize)]
pub enum ValidateDefaultVisibilityFail {}

#[derive(Clone, Debug, Serialize)]
pub enum ValidateShortnameFail {
    Empty,
    SpecialCharacters,
    TooLong,
}

#[derive(Clone, Debug, Serialize)]
pub enum ValidateIsSearchableFail {}

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
            err.display_name = Some(ValidateDisplayNameFail::Empty);
        }

        if self.0.shortname.is_empty() {
            err.shortname = Some(ValidateShortnameFail::Empty);
        }

        if self.0.shortname.chars().any(|c| !c.is_alphanumeric()) {
            err.shortname = Some(ValidateShortnameFail::SpecialCharacters);
        }

        if self.0.shortname.len() > 30 {
            err.shortname = Some(ValidateShortnameFail::TooLong);
        }

        if !err.is_empty() {
            return Err(err);
        }

        Ok(ValidatedPersonaCreationForm {
            display_name: self.0.display_name,
            follow_policy: self.0.follow_policy,
            default_visibility: self.0.default_visibility,
            shortname: self.0.shortname,
            is_searchable: self.0.is_searchable,
        })
    }
}

pub struct ValidatedPersonaCreationForm {
    pub(crate) display_name: String,
    pub(crate) follow_policy: FollowPolicy,
    pub(crate) default_visibility: PostVisibility,
    pub(crate) shortname: String,
    pub(crate) is_searchable: bool,
}

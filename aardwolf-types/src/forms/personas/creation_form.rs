use aardwolf_models::sql_types::{FollowPolicy, PostVisibility, Url};

use crate::{
    forms::personas::PersonaCreationFail,
    traits::Validate,
    wrapper::{ValidateWrapper, Wrapped},
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

pub struct ValidatePersonaCreationForm(pub PersonaCreationForm, pub String);

impl Wrapped for ValidatePersonaCreationForm {
    type Wrapper = ValidateWrapper<Self, <Self as Validate>::Item, <Self as Validate>::Error>;
}

impl Validate for ValidatePersonaCreationForm {
    type Item = ValidatedPersonaCreationForm;
    type Error = PersonaCreationFail;

    fn validate(self) -> Result<ValidatedPersonaCreationForm, PersonaCreationFail> {
        if self.0.display_name.is_empty() {
            return Err(PersonaCreationFail::Validation);
        }

        if self.0.shortname.is_empty() {
            return Err(PersonaCreationFail::Validation);
        }

        if self.0.shortname.chars().any(|c| !c.is_alphanumeric()) {
            return Err(PersonaCreationFail::Validation);
        }

        if self.0.shortname.len() > 30 {
            return Err(PersonaCreationFail::Validation);
        }

        // Local accounts get a generic URL, since their URLs should be generated using the
        // local_uuid field.
        let generic_url: Url = "https://example.com".parse()?;

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

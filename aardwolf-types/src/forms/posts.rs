use aardwolf_models::sql_types::{Mime, PostVisibility};
use mime::TEXT_HTML;
use serde_derive::{Deserialize, Serialize};

use crate::{
    error::AardwolfFail,
    traits::Validate,
    wrapper::{ValidateWrapper, Wrapped},
};

#[derive(Clone, Debug, Deserialize)]
pub struct PostCreationForm {
    csrf_token: String,
    visibility: PostVisibility,
    #[serde(default)]
    name: Option<String>,
    source: String,
}

impl PostCreationForm {
    pub fn as_state(&self) -> PostCreationFormState {
        PostCreationFormState {
            visibility: self.visibility,
            name: self.name.clone(),
            source: self.source.clone(),
        }
    }
}

pub struct PostCreationFormState {
    pub visibility: PostVisibility,
    pub name: Option<String>,
    pub source: String,
}

#[derive(Clone, Debug, Fail, Serialize)]
#[fail(display = "Error validating post creation form")]
pub struct ValidatePostCreationFail {
    pub visibility: Option<ValidateVisibilityError>,
    pub source: Option<ValidateSourceError>,
    pub name: Option<ValidateNameError>,
}

#[derive(Clone, Debug, Serialize)]
pub enum ValidateVisibilityError {}

#[derive(Clone, Debug, Serialize)]
pub enum ValidateSourceError {
    Empty,
}

#[derive(Clone, Debug, Serialize)]
pub enum ValidateNameError {}

impl ValidatePostCreationFail {
    pub fn is_empty(&self) -> bool {
        self.visibility.is_none() && self.source.is_none() && self.name.is_none()
    }
}

impl AardwolfFail for ValidatePostCreationFail {}

pub struct ValidatePostCreationForm(pub PostCreationForm);

impl Wrapped for ValidatePostCreationForm {
    type Wrapper = ValidateWrapper<Self, <Self as Validate>::Item, <Self as Validate>::Error>;
}

impl Validate for ValidatePostCreationForm {
    type Item = ValidatedPostCreationForm;
    type Error = ValidatePostCreationFail;

    fn validate(self) -> Result<Self::Item, Self::Error> {
        let mut err = ValidatePostCreationFail {
            visibility: None,
            source: None,
            name: None,
        };

        let source = self.0.source.trim().to_owned();
        let content = source.clone(); // TODO: translate things here
        let visibility = self.0.visibility;
        let media_type = TEXT_HTML.into();

        let name = if let Some(name) = self.0.name {
            if name.trim().is_empty() {
                None
            } else {
                Some(name)
            }
        } else {
            None
        };

        if source.is_empty() {
            err.source = Some(ValidateSourceError::Empty);
        }

        if !err.is_empty() {
            return Err(err);
        }

        Ok(ValidatedPostCreationForm {
            media_type,
            visibility,
            content,
            source,
            name,
        })
    }
}

pub struct ValidatedPostCreationForm {
    pub(crate) media_type: Mime,
    pub(crate) visibility: PostVisibility,
    pub(crate) content: String,
    pub(crate) source: String,
    pub(crate) name: Option<String>,
}

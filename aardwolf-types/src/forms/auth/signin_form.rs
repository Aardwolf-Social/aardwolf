use aardwolf_models::user::local_auth::PlaintextPassword;

use crate::{
    error::AardwolfFail,
    traits::Validate,
    wrapper::{ValidateWrapper, Wrapped},
};

#[derive(Clone, Debug, Fail, Serialize)]
#[fail(display = "Missing required field")]
pub struct ValidateSignInFormFail {
    pub email: Option<String>,
    pub password: Option<String>,
}

impl ValidateSignInFormFail {
    pub fn is_empty(&self) -> bool {
        self.email.is_none() && self.password.is_none()
    }
}

impl AardwolfFail for ValidateSignInFormFail {}

pub struct SignInFormState {
    pub email: String,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "use-rocket", derive(FromForm))]
pub struct SignInForm {
    pub csrf_token: String,
    pub email: String,
    pub password: PlaintextPassword,
}

impl SignInForm {
    pub fn as_state(&self) -> SignInFormState {
        SignInFormState {
            email: self.email.clone(),
        }
    }
}

pub struct ValidateSignInForm(pub SignInForm);

impl Wrapped for ValidateSignInForm {
    type Wrapper = ValidateWrapper<Self, <Self as Validate>::Item, <Self as Validate>::Error>;
}

impl Validate for ValidateSignInForm {
    type Item = ValidatedSignInForm;
    type Error = ValidateSignInFormFail;

    fn validate(self) -> Result<ValidatedSignInForm, ValidateSignInFormFail> {
        let mut validation_error = ValidateSignInFormFail {
            email: None,
            password: None,
        };

        if self.0.email.is_empty() {
            validation_error.email = Some("Email must be present".to_owned());
        }

        if self.0.password.is_empty() {
            validation_error.password = Some("Password must be present".to_owned());
        }

        if validation_error.is_empty() {
            Ok(ValidatedSignInForm {
                email: self.0.email,
                password: self.0.password,
            })
        } else {
            Err(validation_error)
        }
    }
}

pub struct ValidatedSignInForm {
    pub(crate) email: String,
    pub(crate) password: PlaintextPassword,
}

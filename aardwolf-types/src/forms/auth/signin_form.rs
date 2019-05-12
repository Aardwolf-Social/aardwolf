use aardwolf_models::user::local_auth::PlaintextPassword;

use crate::{
    error::AardwolfFail,
    traits::Validate,
    wrapper::{ValidateWrapper, Wrapped},
};

#[derive(Clone, Debug, Fail, Serialize)]
#[fail(display = "Missing required field")]
pub struct ValidateSignInFormFail {
    pub email: Option<SignInEmailValidationFail>,
    pub password: Option<SignInPasswordValidationFail>,
}

impl ValidateSignInFormFail {
    pub fn is_empty(&self) -> bool {
        self.email.is_none() && self.password.is_none()
    }
}

impl AardwolfFail for ValidateSignInFormFail {}

#[derive(Clone, Debug, Serialize)]
pub enum SignInEmailValidationFail {
    Empty,
}

#[derive(Clone, Debug, Serialize)]
pub enum SignInPasswordValidationFail {
    Empty,
}

pub struct SignInFormState {
    pub email: String,
}

impl Default for SignInFormState {
    fn default() -> Self {
        SignInFormState {
            email: "".to_owned(),
        }
    }
}

#[derive(Debug, Deserialize)]
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
            validation_error.email = Some(SignInEmailValidationFail::Empty);
        }

        if self.0.password.is_empty() {
            validation_error.password = Some(SignInPasswordValidationFail::Empty);
        }

        if !validation_error.is_empty() {
            return Err(validation_error);
        }

        Ok(ValidatedSignInForm {
            email: self.0.email,
            password: self.0.password,
        })
    }
}

pub struct ValidatedSignInForm {
    pub(crate) email: String,
    pub(crate) password: PlaintextPassword,
}

use aardwolf_models::user::local_auth::{PlaintextPassword, ValidationError};

use crate::{
    error::AardwolfFail,
    traits::Validate,
    wrapper::{ValidateWrapper, Wrapped},
};

#[derive(Clone, Fail, Debug, Deserialize, Serialize)]
#[fail(display = "There was an error validating the form")]
pub struct ValidateSignUpFormFail {
    pub email: Option<String>,
    pub password: Option<String>,
    pub password_confirmation: Option<String>,
}

impl ValidateSignUpFormFail {
    pub fn is_empty(&self) -> bool {
        self.email.is_none() && self.password.is_none()
    }
}

impl From<ValidationError> for ValidateSignUpFormFail {
    fn from(e: ValidationError) -> Self {
        ValidateSignUpFormFail {
            email: None,
            password: {
                if e.no_match() {
                    Some("Passwords don't match".to_owned())
                } else if e.too_short() {
                    Some("Password is too short".to_owned())
                } else {
                    None
                }
            },
            password_confirmation: None,
        }
    }
}

impl AardwolfFail for ValidateSignUpFormFail {}

pub struct SignUpFormState {
    pub email: String,
}

impl Default for SignUpFormState {
    fn default() -> Self {
        SignUpFormState {
            email: "".to_owned(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "use-rocket", derive(FromForm))]
pub struct SignUpForm {
    pub csrf_token: String,
    pub email: String,
    pub password: PlaintextPassword,
    pub password_confirmation: PlaintextPassword,
}

impl SignUpForm {
    pub fn as_state(&self) -> SignUpFormState {
        SignUpFormState {
            email: self.email.clone(),
        }
    }
}

pub struct ValidateSignUpForm(pub SignUpForm);

impl Wrapped for ValidateSignUpForm {
    type Wrapper = ValidateWrapper<Self, <Self as Validate>::Item, <Self as Validate>::Error>;
}

impl Validate for ValidateSignUpForm {
    type Item = ValidatedSignUpForm;
    type Error = ValidateSignUpFormFail;

    fn validate(self) -> Result<ValidatedSignUpForm, ValidateSignUpFormFail> {
        let mut validation_error = ValidateSignUpFormFail {
            email: None,
            password: None,
            password_confirmation: None,
        };

        if self.0.email.is_empty() {
            validation_error.email = Some("Email must not be blank".to_owned());
        }

        if self.0.password.is_empty() {
            validation_error.password = Some("Password must not be blank".to_owned());
        }

        if self.0.password_confirmation.is_empty() {
            validation_error.password_confirmation =
                Some("Password Confirmation must not be blank".to_owned());
        }

        if validation_error.is_empty() {
            Ok(ValidatedSignUpForm {
                email: self.0.email,
                password: self.0.password,
                password_confirmation: self.0.password_confirmation,
            })
        } else {
            Err(validation_error)
        }
    }
}

pub struct ValidatedSignUpForm {
    pub(crate) email: String,
    pub(crate) password: PlaintextPassword,
    pub(crate) password_confirmation: PlaintextPassword,
}

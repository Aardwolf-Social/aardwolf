use aardwolf_models::user::local_auth::{PlaintextPassword, ValidationError};

use crate::{error::AardwolfFail, traits::Validate};

#[derive(Clone, Fail, Debug, Serialize)]
#[fail(display = "There was an error validating the form")]
pub struct ValidateSignUpFormFail {
    pub email: Option<SignUpEmailValidationFail>,
    pub password: Option<SignUpPasswordValidationFail>,
    pub password_confirmation: Option<SignUpPasswordConfirmationValidationFail>,
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
            password: if e.too_short() {
                Some(SignUpPasswordValidationFail::TooShort)
            } else {
                None
            },
            password_confirmation: if e.no_match() {
                Some(SignUpPasswordConfirmationValidationFail::Match)
            } else {
                None
            },
        }
    }
}

impl AardwolfFail for ValidateSignUpFormFail {}

#[derive(Clone, Debug, Serialize)]
pub enum SignUpEmailValidationFail {
    Empty,
    /// TODO: implement this
    Malformed,
}

#[derive(Clone, Debug, Serialize)]
pub enum SignUpPasswordValidationFail {
    Empty,
    /// TODO: implement this
    TooShort,
}

#[derive(Clone, Debug, Serialize)]
pub enum SignUpPasswordConfirmationValidationFail {
    Empty,
    /// TODO: implement this
    Match,
}

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
            validation_error.email = Some(SignUpEmailValidationFail::Empty);
        }

        if self.0.password.is_empty() {
            validation_error.password = Some(SignUpPasswordValidationFail::Empty);
        }

        if self.0.password_confirmation.is_empty() {
            validation_error.password_confirmation =
                Some(SignUpPasswordConfirmationValidationFail::Empty);
        }

        if !validation_error.is_empty() {
            return Err(validation_error);
        }

        Ok(ValidatedSignUpForm {
            email: self.0.email,
            password: self.0.password,
            password_confirmation: self.0.password_confirmation,
        })
    }
}

pub struct ValidatedSignUpForm {
    pub(crate) email: String,
    pub(crate) password: PlaintextPassword,
    pub(crate) password_confirmation: PlaintextPassword,
}

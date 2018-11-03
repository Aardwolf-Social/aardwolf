use aardwolf_models::user::local_auth::{PlaintextPassword, ValidationError};

use crate::{
    error::{AardwolfError, AardwolfErrorKind},
    forms::traits::Validate,
};

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "use-rocket", derive(FromForm))]
pub struct SignUpErrorMessage {
    pub msg: String,
}

#[derive(Clone, Fail, Debug, Deserialize, Serialize)]
#[fail(display = "There was an error validating the form")]
pub struct ValidateSignUpFormFail {
    pub email_length: bool,
    pub password_match: bool,
    pub password_length: bool,
}

impl From<ValidationError> for ValidateSignUpFormFail {
    fn from(e: ValidationError) -> Self {
        ValidateSignUpFormFail {
            email_length: false,
            password_match: e.no_match(),
            password_length: e.too_short(),
        }
    }
}

impl AardwolfError for ValidateSignUpFormFail {
    fn name(&self) -> &str {
        "Invalid Singup Form"
    }

    fn kind(&self) -> AardwolfErrorKind {
        AardwolfErrorKind::BadRequest
    }

    fn description(&self) -> String {
        format!("{}", self)
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

pub struct ValidateSignUpForm;

impl ValidateSignUpForm {
    pub fn with(self, form: SignUpForm) -> ValidateSignUpFormOperation {
        ValidateSignUpFormOperation(form)
    }
}

pub struct ValidateSignUpFormOperation(SignUpForm);

impl Validate<ValidatedSignUpForm, ValidateSignUpFormFail> for ValidateSignUpFormOperation {
    fn validate(self) -> Result<ValidatedSignUpForm, ValidateSignUpFormFail> {
        if self.0.email.is_empty() {
            Err(ValidateSignUpFormFail {
                email_length: true,
                password_match: false,
                password_length: false,
            })
        } else {
            Ok(ValidatedSignUpForm {
                email: self.0.email,
                password: self.0.password,
                password_confirmation: self.0.password_confirmation,
            })
        }
    }
}

pub struct ValidatedSignUpForm {
    pub(crate) email: String,
    pub(crate) password: PlaintextPassword,
    pub(crate) password_confirmation: PlaintextPassword,
}

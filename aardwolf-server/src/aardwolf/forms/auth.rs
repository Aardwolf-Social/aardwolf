use aardwolf_models::user::local_auth::{PlaintextPassword, ValidationError};

use forms::traits::Validate;

#[derive(Fail, Debug)]
#[fail(display = "There was an error validating the form")]
pub(crate) struct SignUpFormValidationFail {
    pub email_length: bool,
    pub password_match: bool,
    pub password_length: bool,
}

impl From<ValidationError> for SignUpFormValidationFail {
    fn from(e: ValidationError) -> Self {
        SignUpFormValidationFail {
            email_length: false,
            password_match: e.no_match(),
            password_length: e.too_short(),
        }
    }
}

#[derive(Debug, FromForm)]
pub(crate) struct SignUpForm {
    pub csrf_token: String,
    pub email: String,
    pub password: PlaintextPassword,
    pub password_confirmation: PlaintextPassword,
}

impl Validate<SignUpFormValidationFail> for SignUpForm {
    fn validate(&self) -> Result<(), SignUpFormValidationFail> {
        if self.email.is_empty() {
            return Err(SignUpFormValidationFail {
                email_length: true,
                password_match: false,
                password_length: false,
            });
        } else {
            return Ok(());
        }
    }
}

#[derive(Fail, Debug)]
pub(crate) enum SignInFormValidationFail {
    #[fail(display = "Field `email` is required")]
    EmptyEmailError,
}

#[derive(Debug, FromForm)]
pub(crate) struct SignInForm {
    pub csrf_token: String,
    pub email: String,
    pub password: PlaintextPassword,
}

impl Validate<SignInFormValidationFail> for SignInForm {
    fn validate(&self) -> Result<(), SignInFormValidationFail> {
        if self.email.is_empty() {
            return Err(SignInFormValidationFail::EmptyEmailError);
        } else {
            return Ok(());
        }
    }
}

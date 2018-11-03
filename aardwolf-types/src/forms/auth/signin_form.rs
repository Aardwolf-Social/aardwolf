use aardwolf_models::user::local_auth::PlaintextPassword;

use crate::{
    error::{AardwolfError, AardwolfErrorKind},
    forms::traits::Validate,
};

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "use-rocket", derive(FromForm))]
pub struct SignInErrorMessage {
    pub msg: String,
}

#[derive(Clone, Fail, Debug)]
pub enum ValidateSignInFormFail {
    #[fail(display = "Field `email` is required")]
    EmptyEmailError,
}

impl AardwolfError for ValidateSignInFormFail {
    fn name(&self) -> &str {
        "Invalid SignIn Form"
    }

    fn kind(&self) -> AardwolfErrorKind {
        match *self {
            ValidateSignInFormFail::EmptyEmailError => AardwolfErrorKind::BadRequest,
        }
    }

    fn description(&self) -> String {
        format!("{}", self)
    }
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "use-rocket", derive(FromForm))]
pub struct SignInForm {
    pub csrf_token: String,
    pub email: String,
    pub password: PlaintextPassword,
}

pub struct ValidateSignInForm;

impl ValidateSignInForm {
    pub fn with(self, form: SignInForm) -> ValidateSignInFormOperation {
        ValidateSignInFormOperation(form)
    }
}

pub struct ValidateSignInFormOperation(SignInForm);

impl Validate<ValidatedSignInForm, ValidateSignInFormFail> for ValidateSignInFormOperation {
    fn validate(self) -> Result<ValidatedSignInForm, ValidateSignInFormFail> {
        if self.0.email.is_empty() {
            Err(ValidateSignInFormFail::EmptyEmailError)
        } else {
            Ok(ValidatedSignInForm {
                email: self.0.email,
                password: self.0.password,
            })
        }
    }
}

pub struct ValidatedSignInForm {
    pub(crate) email: String,
    pub(crate) password: PlaintextPassword,
}
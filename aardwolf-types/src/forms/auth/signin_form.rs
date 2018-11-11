use aardwolf_models::user::local_auth::PlaintextPassword;

use crate::{
    error::AardwolfFail,
    traits::Validate,
    wrapper::{ValidateWrapper, Wrapped},
};

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "use-rocket", derive(FromForm))]
pub struct SignInErrorMessage {
    pub msg: String,
}

#[derive(Clone, Debug, Fail, Serialize)]
pub enum ValidateSignInFormFail {
    #[fail(display = "Field `email` is required")]
    EmptyEmailError,
}

impl AardwolfFail for ValidateSignInFormFail {}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "use-rocket", derive(FromForm))]
pub struct SignInForm {
    pub csrf_token: String,
    pub email: String,
    pub password: PlaintextPassword,
}

pub struct ValidateSignInForm(pub SignInForm);

impl Wrapped for ValidateSignInForm {
    type Wrapper = ValidateWrapper<Self, <Self as Validate>::Item, <Self as Validate>::Error>;
}

impl Validate for ValidateSignInForm {
    type Item = ValidatedSignInForm;
    type Error = ValidateSignInFormFail;

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

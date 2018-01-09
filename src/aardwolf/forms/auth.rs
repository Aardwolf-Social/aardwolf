use forms::traits::Validate;

#[derive(Fail, Debug)]
pub(crate) enum SignUpFormValidationFail {
    #[fail(display="Passwords do not match")]
    PasswordMatchError,
    #[fail(display="Field `email` is required")]
    EmptyEmailError,
    #[fail(display="Field `username` is required")]
    EmptyUsernameError,
    #[fail(display="Field `password` is required")]
    EmptyPasswordError,
}

#[derive(Debug, Clone, PartialEq, FromForm)]
pub(crate) struct SignUpForm {
    pub csrf_token: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub password_confirmation: String,
}

impl Validate <SignUpFormValidationFail> for SignUpForm {

    fn validate(&self) -> Result<(), SignUpFormValidationFail> {
        if self.email.is_empty() {
            return Err(SignUpFormValidationFail::EmptyEmailError)
        } else if self.username.is_empty() {
            return Err(SignUpFormValidationFail::EmptyUsernameError)
        } else if self.password.is_empty() {
            return Err(SignUpFormValidationFail::EmptyPasswordError)
        } else if self.password != self.password_confirmation {
            return Err(SignUpFormValidationFail::PasswordMatchError);
        } else {
            return Ok(())
        }
    }
}

#[derive(Fail, Debug)]
pub(crate) enum SignInFormValidationFail {
    #[fail(display="Field `email` is required")]
    EmptyEmailError,
    #[fail(display="Field `password` is required")]
    EmptyPasswordError,
}

#[derive(Debug, Clone, PartialEq, FromForm)]
pub(crate) struct SignInForm {
    pub csrf_token: String,
    pub email: String,
    pub password: String,
}

impl Validate <SignInFormValidationFail> for SignInForm {

    fn validate(&self) -> Result<(), SignInFormValidationFail> {
        if self.email.is_empty() {
            return Err(SignInFormValidationFail::EmptyEmailError)
        } else if self.password.is_empty() {
            return Err(SignInFormValidationFail::EmptyPasswordError)
        } else {
            return Ok(())
        }
    }
}

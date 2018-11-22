use aardwolf_models::user::{AuthenticatedUser, UnauthenticatedUser};
use diesel::{self, pg::PgConnection};

use crate::{
    error::AardwolfFail,
    forms::auth::{ValidateSignInFormFail, ValidatedSignInForm},
    traits::DbAction,
    wrapper::{DbActionWrapper, Wrapped},
};

pub struct SignIn(pub ValidatedSignInForm);

impl Wrapped for SignIn {
    type Wrapper = DbActionWrapper<Self, <Self as DbAction>::Item, <Self as DbAction>::Error>;
}

impl DbAction for SignIn {
    type Item = AuthenticatedUser;
    type Error = SignInFail;

    fn db_action(self, conn: &PgConnection) -> Result<AuthenticatedUser, SignInFail> {
        UnauthenticatedUser::by_email_for_auth(&self.0.email, conn)
            .map_err(|_| SignInFail::GenericLoginError)
            .and_then(|(user, _email, auth)| {
                user.log_in_local(auth, self.0.password)
                    .map_err(|_| SignInFail::GenericLoginError)
            })
    }
}

#[derive(Clone, Debug, Fail, Serialize)]
pub enum SignInFail {
    #[fail(display = "Sign in failed because {}", _0)]
    ValidationError(#[cause] ValidateSignInFormFail),
    // this is the generic "login failed" error the user will see
    #[fail(display = "Invalid username or password")]
    GenericLoginError,
}

impl AardwolfFail for SignInFail {}

impl From<ValidateSignInFormFail> for SignInFail {
    fn from(e: ValidateSignInFormFail) -> Self {
        SignInFail::ValidationError(e)
    }
}

impl From<diesel::result::Error> for SignInFail {
    fn from(_: diesel::result::Error) -> Self {
        SignInFail::GenericLoginError
    }
}

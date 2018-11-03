use aardwolf_models::user::{AuthenticatedUser, UnauthenticatedUser};
use diesel::{self, pg::PgConnection};

use crate::{
    error::{AardwolfError, AardwolfErrorKind},
    forms::{
        auth::{ValidateSignInFormFail, ValidatedSignInForm},
        traits::DbAction,
    },
};

pub struct SignIn;

impl SignIn {
    pub fn with(self, form: ValidatedSignInForm) -> SignInOperation {
        SignInOperation(form)
    }
}

pub struct SignInOperation(ValidatedSignInForm);

impl DbAction<AuthenticatedUser, SignInFail> for SignInOperation {
    fn db_action(self, conn: &PgConnection) -> Result<AuthenticatedUser, SignInFail> {
        UnauthenticatedUser::by_email_for_auth(&self.0.email, conn)
            .map_err(|_| SignInFail::GenericLoginError)
            .and_then(|(user, _email, auth)| {
                user.log_in_local(auth, self.0.password)
                    .map_err(|_| SignInFail::GenericLoginError)
            })
    }
}

#[derive(Clone, Fail, Debug)]
pub enum SignInFail {
    #[fail(display = "Sign up failed because {}", _0)]
    ValidationError(#[cause] ValidateSignInFormFail),
    // this is the generic "login failed" error the user will see
    #[fail(display = "Invalid username or password")]
    GenericLoginError,
}

impl AardwolfError for SignInFail {
    fn name(&self) -> &str {
        "SignIn Fail"
    }

    fn kind(&self) -> AardwolfErrorKind {
        match *self {
            SignInFail::ValidationError(_) => AardwolfErrorKind::BadRequest,
            SignInFail::GenericLoginError => AardwolfErrorKind::BadRequest,
        }
    }

    fn description(&self) -> String {
        format!("{}", self)
    }
}

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

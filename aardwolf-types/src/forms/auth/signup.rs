use aardwolf_models::user::{
    email::{CreationError, EmailToken, NewEmail, UnverifiedEmail},
    local_auth::{NewLocalAuth, PasswordCreationError},
    NewUser,
};
use diesel::{self, pg::PgConnection, Connection};

use crate::{
    error::AardwolfFail,
    forms::{
        auth::{ValidateSignUpFormFail, ValidatedSignUpForm},
        traits::DbAction,
    },
};

pub struct SignUp;

impl SignUp {
    pub fn with(self, form: ValidatedSignUpForm) -> SignUpOperation {
        SignUpOperation(form)
    }
}

pub struct SignUpOperation(ValidatedSignUpForm);

impl DbAction<(UnverifiedEmail, EmailToken), SignUpFail> for SignUpOperation {
    fn db_action(self, conn: &PgConnection) -> Result<(UnverifiedEmail, EmailToken), SignUpFail> {
        conn.transaction::<_, SignUpFail, _>(|| {
            let user = NewUser::new()
                .insert(conn)
                .map_err(|_| SignUpFail::UserCreateError)?;

            let user = match user.to_verified(conn).map_err(|_| SignUpFail::UserLookup)? {
                Ok(_unauthenticatec_user) => return Err(SignUpFail::VerifiedUser),
                Err(unverified_user) => unverified_user,
            };

            let _local_auth =
                NewLocalAuth::new_from_two(&user, self.0.password, self.0.password_confirmation)?
                    .insert(conn)
                    .map_err(|_| SignUpFail::LocalAuthCreateError)?;

            let (email, token) = NewEmail::new(self.0.email, &user)?;

            let email = email
                .insert(conn)
                .map_err(|_| SignUpFail::EmailCreateError)?;

            Ok((email, token))
        })
    }
}

#[derive(Clone, Debug, Fail, Serialize)]
pub enum SignUpFail {
    #[fail(display = "Sign up failed because {}", _0)]
    ValidationError(#[cause] ValidateSignUpFormFail),
    #[fail(display = "Failed to insert local_auth into database")]
    LocalAuthCreateError,
    #[fail(display = "Failed to insert user into database")]
    UserCreateError,
    #[fail(display = "Failed to insert email into database")]
    EmailCreateError,
    #[fail(display = "Failed to hash password")]
    PasswordHashError,
    #[fail(display = "Failed to create confirmation token")]
    CreateTokenError,
    #[fail(display = "New user shouldn't be verified")]
    VerifiedUser,
    #[fail(display = "Failed to lookup newly created user")]
    UserLookup,
    #[fail(display = "Failed to perform database transaction")]
    Transaction,
}

impl AardwolfFail for SignUpFail {}

impl From<ValidateSignUpFormFail> for SignUpFail {
    fn from(e: ValidateSignUpFormFail) -> SignUpFail {
        SignUpFail::ValidationError(e)
    }
}

impl From<diesel::result::Error> for SignUpFail {
    fn from(_: diesel::result::Error) -> Self {
        SignUpFail::Transaction
    }
}

impl From<PasswordCreationError> for SignUpFail {
    fn from(e: PasswordCreationError) -> Self {
        match e {
            PasswordCreationError::Validation(e) => SignUpFail::ValidationError(e.into()),
            PasswordCreationError::Bcrypt => SignUpFail::PasswordHashError,
        }
    }
}

impl From<CreationError> for SignUpFail {
    fn from(_: CreationError) -> Self {
        SignUpFail::CreateTokenError
    }
}

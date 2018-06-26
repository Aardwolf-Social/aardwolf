use aardwolf_models::user::{
    email::{CreationError, EmailToken, NewEmail, UnverifiedEmail},
    local_auth::{NewLocalAuth, PasswordCreationError, PlaintextPassword, ValidationError},
    {AuthenticatedUser, NewUser, UnauthenticatedUser},
};
<<<<<<< HEAD
use diesel;
use diesel::pg::PgConnection;
use diesel::Connection;
=======
use diesel::{self, pg::PgConnection, Connection};
>>>>>>> origin/master

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

impl Validate<ValidatedSignupForm, SignUpFormValidationFail> for SignUpForm {
    fn validate(self) -> Result<ValidatedSignupForm, SignUpFormValidationFail> {
        if self.email.is_empty() {
            Err(SignUpFormValidationFail {
                email_length: true,
                password_match: false,
                password_length: false,
            })
        } else {
            Ok(ValidatedSignupForm {
                email: self.email,
                password: self.password,
                password_confirmation: self.password_confirmation,
            })
        }
    }
}

pub(crate) struct ValidatedSignupForm {
    email: String,
    password: PlaintextPassword,
    password_confirmation: PlaintextPassword,
}

impl ValidatedSignupForm {
    pub fn create_user_and_auth(
        self,
        db: &PgConnection,
    ) -> Result<(UnverifiedEmail, EmailToken), SignUpFail> {
        db.transaction::<_, SignUpFail, _>(|| {
            let user = NewUser::new()
                .insert(db)
                .map_err(|_| SignUpFail::UserCreateError)?;

            let user = match user.to_verified(db).map_err(|_| SignUpFail::UserLookup)? {
                Ok(_unauthenticatec_user) => return Err(SignUpFail::VerifiedUser),
                Err(unverified_user) => unverified_user,
            };

            let _local_auth =
                NewLocalAuth::new_from_two(&user, self.password, self.password_confirmation)?
                    .insert(db)
                    .map_err(|_| SignUpFail::LocalAuthCreateError)?;

            let (email, token) = NewEmail::new(self.email, &user)?;

            let email = email.insert(db).map_err(|_| SignUpFail::EmailCreateError)?;

            Ok((email, token))
        })
    }
}

#[derive(Fail, Debug)]
pub(crate) enum SignUpFail {
    #[fail(display = "Sign up failed because {}", _0)]
    ValidationError(#[cause] SignUpFormValidationFail),
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

impl From<SignUpFormValidationFail> for SignUpFail {
    fn from(e: SignUpFormValidationFail) -> SignUpFail {
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

impl Validate<ValidatedSignInForm, SignInFormValidationFail> for SignInForm {
    fn validate(self) -> Result<ValidatedSignInForm, SignInFormValidationFail> {
        if self.email.is_empty() {
            Err(SignInFormValidationFail::EmptyEmailError)
        } else {
            Ok(ValidatedSignInForm {
                email: self.email,
                password: self.password,
            })
        }
    }
}

pub(crate) struct ValidatedSignInForm {
    email: String,
    password: PlaintextPassword,
}

impl ValidatedSignInForm {
    pub fn sign_in(self, db: &PgConnection) -> Result<AuthenticatedUser, SignInFail> {
        UnauthenticatedUser::by_email_for_auth(&self.email, db)
            .map_err(|_| SignInFail::GenericLoginError)
            .and_then(|(user, _email, auth)| {
                user.log_in_local(auth, self.password)
                    .map_err(|_| SignInFail::GenericLoginError)
            })
    }
}

#[derive(Fail, Debug)]
pub(crate) enum SignInFail {
    #[fail(display = "Sign up failed because {}", _0)]
    ValidationError(SignInFormValidationFail),
    // this is the generic "login failed" error the user will see
    #[fail(display = "Invalid username or password")]
    GenericLoginError,
}

impl From<SignInFormValidationFail> for SignInFail {
    fn from(e: SignInFormValidationFail) -> Self {
        SignInFail::ValidationError(e)
    }
}

impl From<diesel::result::Error> for SignInFail {
    fn from(_: diesel::result::Error) -> Self {
        SignInFail::GenericLoginError
    }
}

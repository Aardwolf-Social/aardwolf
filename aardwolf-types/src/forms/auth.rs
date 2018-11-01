use aardwolf_models::user::{
    email::{CreationError, EmailToken, EmailVerificationToken, NewEmail, UnverifiedEmail},
    local_auth::{NewLocalAuth, PasswordCreationError, PlaintextPassword, ValidationError},
    {AuthenticatedUser, NewUser, UnauthenticatedUser},
};
use diesel::{self, pg::PgConnection, Connection};

use crate::{
    error::{AardwolfError, AardwolfErrorKind},
    forms::traits::{DbAction, Validate},
};

#[derive(Clone, Fail, Debug, Deserialize, Serialize)]
#[fail(display = "There was an error validating the form")]
pub struct SignUpFormValidationFail {
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

impl AardwolfError for SignUpFormValidationFail {
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

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "use-rocket", derive(FromForm))]
pub struct SignUpError {
    pub msg: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "use-rocket", derive(FromForm))]
pub struct SignInError {
    pub msg: String,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "use-rocket", derive(FromForm))]
pub struct SignUpForm {
    pub csrf_token: String,
    pub email: String,
    pub password: PlaintextPassword,
    pub password_confirmation: PlaintextPassword,
}

impl Validate<ValidatedSignUpForm, SignUpFormValidationFail> for SignUpForm {
    fn validate(self) -> Result<ValidatedSignUpForm, SignUpFormValidationFail> {
        if self.email.is_empty() {
            Err(SignUpFormValidationFail {
                email_length: true,
                password_match: false,
                password_length: false,
            })
        } else {
            Ok(ValidatedSignUpForm {
                email: self.email,
                password: self.password,
                password_confirmation: self.password_confirmation,
            })
        }
    }
}

pub struct ValidatedSignUpForm {
    email: String,
    password: PlaintextPassword,
    password_confirmation: PlaintextPassword,
}

impl DbAction<(UnverifiedEmail, EmailToken), SignUpFail> for ValidatedSignUpForm {
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
                NewLocalAuth::new_from_two(&user, self.password, self.password_confirmation)?
                    .insert(conn)
                    .map_err(|_| SignUpFail::LocalAuthCreateError)?;

            let (email, token) = NewEmail::new(self.email, &user)?;

            let email = email
                .insert(conn)
                .map_err(|_| SignUpFail::EmailCreateError)?;

            Ok((email, token))
        })
    }
}

#[derive(Clone, Fail, Debug)]
pub enum SignUpFail {
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

impl AardwolfError for SignUpFail {
    fn name(&self) -> &str {
        "Signup Fail"
    }

    fn kind(&self) -> AardwolfErrorKind {
        match *self {
            SignUpFail::ValidationError(_) => AardwolfErrorKind::BadRequest,
            SignUpFail::LocalAuthCreateError
            | SignUpFail::UserCreateError
            | SignUpFail::EmailCreateError
            | SignUpFail::PasswordHashError
            | SignUpFail::CreateTokenError
            | SignUpFail::VerifiedUser
            | SignUpFail::Transaction => AardwolfErrorKind::InternalServerError,
            SignUpFail::UserLookup => AardwolfErrorKind::NotFound,
        }
    }

    fn description(&self) -> String {
        format!("{}", self)
    }
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

#[derive(Clone, Fail, Debug)]
pub enum SignInFormValidationFail {
    #[fail(display = "Field `email` is required")]
    EmptyEmailError,
}

impl AardwolfError for SignInFormValidationFail {
    fn name(&self) -> &str {
        "Invalid SignIn Form"
    }

    fn kind(&self) -> AardwolfErrorKind {
        match *self {
            SignInFormValidationFail::EmptyEmailError => AardwolfErrorKind::BadRequest,
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

pub struct ValidatedSignInForm {
    email: String,
    password: PlaintextPassword,
}

impl DbAction<AuthenticatedUser, SignInFail> for ValidatedSignInForm {
    fn db_action(self, conn: &PgConnection) -> Result<AuthenticatedUser, SignInFail> {
        UnauthenticatedUser::by_email_for_auth(&self.email, conn)
            .map_err(|_| SignInFail::GenericLoginError)
            .and_then(|(user, _email, auth)| {
                user.log_in_local(auth, self.password)
                    .map_err(|_| SignInFail::GenericLoginError)
            })
    }
}

#[derive(Clone, Fail, Debug)]
pub enum SignInFail {
    #[fail(display = "Sign up failed because {}", _0)]
    ValidationError(#[cause] SignInFormValidationFail),
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

#[derive(Clone, Debug, Fail)]
pub enum ConfirmAccountFail {
    #[fail(display = "email was not found")]
    EmailNotFound,
    #[fail(display = "account already confirmed")]
    Confirmed,
    #[fail(display = "Failed to lookup newly created user")]
    UserLookup,
    #[fail(display = "Failed to verify email")]
    Verify,
}

impl AardwolfError for ConfirmAccountFail {
    fn name(&self) -> &str {
        "Confirm Account Fail"
    }

    fn kind(&self) -> AardwolfErrorKind {
        match *self {
            ConfirmAccountFail::Confirmed => AardwolfErrorKind::BadRequest,
            ConfirmAccountFail::EmailNotFound | ConfirmAccountFail::UserLookup => {
                AardwolfErrorKind::NotFound
            }
            ConfirmAccountFail::Verify => AardwolfErrorKind::InternalServerError,
        }
    }

    fn description(&self) -> String {
        format!("{}", self)
    }
}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "use-rocket", derive(FromForm))]
pub struct ConfirmToken {
    id: i32,
    token: EmailVerificationToken,
}

impl DbAction<AuthenticatedUser, ConfirmAccountFail> for ConfirmToken {
    fn db_action(self, conn: &PgConnection) -> Result<AuthenticatedUser, ConfirmAccountFail> {
        let (unauthenticated_user, email) = UnauthenticatedUser::by_email_id(self.id, conn)
            .map_err(|_| ConfirmAccountFail::EmailNotFound)?;

        info!(
            "Found user and email, {:?} - {:?}",
            unauthenticated_user, email
        );

        let user = match unauthenticated_user
            .to_verified(conn)
            .map_err(|_| ConfirmAccountFail::UserLookup)?
        {
            Ok(unauthenticated_user) => {
                error!("User already verified: {:?}", unauthenticated_user);
                return Err(ConfirmAccountFail::Confirmed);
            }
            Err(unverified_user) => unverified_user,
        };

        info!("User is not yet verified");

        let email = match email.to_verified() {
            Ok(verified_email) => {
                error!(
                    "Tried to verify already verified email: {}",
                    verified_email.email()
                );
                return Err(ConfirmAccountFail::Confirmed);
            }
            Err(unverified_email) => unverified_email,
        };

        info!("Email is not yet verified");

        let (user, _email) = user
            .verify(email, self.token)
            .map_err(|_| ConfirmAccountFail::Verify)?
            .store_verify(conn)
            .map_err(|e| {
                error!("Could not store verified user: {}, {:?}", e, e);
                ConfirmAccountFail::Confirmed
            })?;

        info!("Verified user and email");

        Ok(user)
    }
}

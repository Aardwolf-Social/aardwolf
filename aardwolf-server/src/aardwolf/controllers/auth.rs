use aardwolf_models::user::local_auth::{NewLocalAuth, PasswordCreationError};
use aardwolf_models::user::email::{CreationError, EmailVerificationToken, NewEmail};
use aardwolf_models::user::{AuthenticatedUser, NewUser, UnauthenticatedUser};
use diesel;
use diesel::Connection;
use diesel::pg::PgConnection;

use forms::auth::{SignInForm, SignInFormValidationFail, SignUpForm, SignUpFormValidationFail};
use forms::traits::Validate;

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

pub(crate) fn create_user_and_account(
    form: SignUpForm,
    db: &PgConnection,
) -> Result<(), SignUpFail> {
    // validation goes here
    form.validate()?;

    db.transaction::<_, SignUpFail, _>(|| {
        let user = NewUser::new()
            .insert(db)
            .map_err(|_| SignUpFail::UserCreateError)?;

        let user = match user.to_verified(db).map_err(|_| SignUpFail::UserLookup)? {
            Ok(_unauthenticatec_user) => return Err(SignUpFail::VerifiedUser),
            Err(unverified_user) => unverified_user,
        };

        let _local_auth =
            NewLocalAuth::new_from_two(&user, form.password, form.password_confirmation)?
                .insert(db)
                .map_err(|_| SignUpFail::LocalAuthCreateError)?;

        let (email, token) = NewEmail::new(form.email, &user)?;

        let email = email.insert(db).map_err(|_| SignUpFail::EmailCreateError)?;

        // just printing this out for now so we can copy/paste into the browser to confirm accounts,
        // but obviously this will need to be emailed to the user
        println!(
            "confirmation token url: /auth/confirmation?id={}&token={}",
            email.id(),
            token
        );

        Ok(())
    })
}

#[derive(Debug, Fail)]
pub(crate) enum ConfirmAccountFail {
    #[fail(display = "email was not found")]
    EmailNotFound,
    #[fail(display = "account already confirmed")]
    Confirmed,
    #[fail(display = "Failed to lookup newly created user")]
    UserLookup,
    #[fail(display = "Failed to verify email")]
    Verify,
}

pub(crate) fn confirm_account(
    email_id: i32,
    token: EmailVerificationToken,
    db: &PgConnection,
) -> Result<AuthenticatedUser, ConfirmAccountFail> {
    let (unauthenticated_user, email) = UnauthenticatedUser::by_email_id(email_id, db)
        .map_err(|_| ConfirmAccountFail::EmailNotFound)?;

    let user = match unauthenticated_user
        .to_verified(db)
        .map_err(|_| ConfirmAccountFail::UserLookup)?
    {
        Ok(_unauthenticatec_user) => return Err(ConfirmAccountFail::Confirmed),
        Err(unverified_user) => unverified_user,
    };

    let email = match email.to_verified() {
        Ok(_verified_email) => return Err(ConfirmAccountFail::Confirmed),
        Err(unverified_email) => unverified_email,
    };

    let (user, _email) = user.verify(email, token)
        .map_err(|_| ConfirmAccountFail::Verify)?
        .store_verify(db)
        .map_err(|_| ConfirmAccountFail::Confirmed)?;

    Ok(user)
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

pub(crate) fn sign_in(
    form: SignInForm,
    db: &PgConnection,
) -> Result<AuthenticatedUser, SignInFail> {
    form.validate()?;

    // TODO: check csrf token

    let (user, _email, local_auth) = UnauthenticatedUser::by_email_for_auth(&form.email, db)
        .map_err(|_| SignInFail::GenericLoginError)?;

    let user = user.log_in_local(local_auth, form.password)
        .map_err(|_| SignInFail::GenericLoginError)?;

    Ok(user)
}

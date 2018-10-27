use aardwolf_models::user::{
    email::EmailVerificationToken,
    {AuthenticatedUser, UnauthenticatedUser},
};
use diesel::pg::PgConnection;

use aardwolf_types::forms::{
    auth::{SignInFail, SignInForm, SignUpFail, SignUpForm},
    traits::Validate,
};

pub(crate) fn create_user_and_account(
    form: SignUpForm,
    db: &PgConnection,
) -> Result<(), SignUpFail> {
    // validation goes here
    let (email, token) = form.validate()?.create_user_and_auth(db)?;

    // just printing this out for now so we can copy/paste into the browser to confirm accounts,
    // but obviously this will need to be emailed to the user
    println!(
        "confirmation token url: /auth/confirmation?id={}&token={}",
        email.id(),
        token
    );

    Ok(())
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

    let (user, _email) = user
        .verify(email, token)
        .map_err(|_| ConfirmAccountFail::Verify)?
        .store_verify(db)
        .map_err(|_| ConfirmAccountFail::Confirmed)?;

    Ok(user)
}

pub(crate) fn sign_in(
    form: SignInForm,
    db: &PgConnection,
) -> Result<AuthenticatedUser, SignInFail> {
    // TODO: check csrf token (this will probably be a request guard)
    form.validate()?.sign_in(db)
}

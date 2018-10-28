use aardwolf_models::user::AuthenticatedUser;
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

pub(crate) fn sign_in(
    form: SignInForm,
    db: &PgConnection,
) -> Result<AuthenticatedUser, SignInFail> {
    // TODO: check csrf token (this will probably be a request guard)
    form.validate()?.sign_in(db)
}

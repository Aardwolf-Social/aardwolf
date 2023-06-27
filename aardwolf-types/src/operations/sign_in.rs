use aardwolf_models::user::{AuthenticatedUser, UnauthenticatedUser};
use diesel::{self, pg::PgConnection};

use crate::{
    error::AardwolfFail,
    forms::auth::{ValidateSignInFormFail, ValidatedSignInForm},
    traits::DbAction,
};

/// This operation authenticates a user
pub struct SignIn(pub ValidatedSignInForm);

impl DbAction for SignIn {
    type Item = AuthenticatedUser;
    type Error = SignInFail;

    fn db_action(self, conn: &mut PgConnection) -> Result<AuthenticatedUser, SignInFail> {
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
    /// The login form was invalid for some reason
    ValidationError(#[cause] ValidateSignInFormFail),

    #[fail(display = "Invalid username or password")]
    /// The generic "login failed" error the user will see
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

#[cfg(test)]
mod tests {
    use aardwolf_test_helpers::models::{
        create_plaintext_password, gen_string, make_verified_authenticated_user, with_connection,
    };
    use diesel::pg::PgConnection;
    use failure::Error;

    use crate::{forms::auth::ValidatedSignInForm, operations::sign_in::SignIn, traits::DbAction};

    fn setup<F>(f: F)
    where
        F: FnOnce(&mut PgConnection, ValidatedSignInForm) -> Result<(), Error>,
    {
        with_connection(|conn| {
            let pass = gen_string();

            let (auth_user, email) = make_verified_authenticated_user(conn, &pass)?;
            let password = create_plaintext_password(&pass)?;

            let form = ValidatedSignInForm {
                email: email.email().to_owned(),
                password,
            };

            f(conn, form)
        })
    }

    #[test]
    fn log_in_user() {
        setup(|conn, form| {
            let operation = SignIn(form);

            assert!(operation.db_action(conn).is_ok());
            Ok(())
        })
    }

    #[test]
    fn fail_log_in_with_bad_password() {
        setup(|conn, mut form| {
            form.password = create_plaintext_password("not the password")?;
            let operation = SignIn(form);

            assert!(operation.db_action(conn).is_err());
            Ok(())
        })
    }

    #[test]
    fn fail_log_in_with_bad_email() {
        setup(|conn, mut form| {
            form.email = "not the email".to_owned();
            let operation = SignIn(form);

            assert!(operation.db_action(conn).is_err());
            Ok(())
        })
    }
}

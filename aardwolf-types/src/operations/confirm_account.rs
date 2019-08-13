use aardwolf_models::user::{
    email::EmailVerificationToken,
    {AuthenticatedUser, UnauthenticatedUser},
};
use diesel::pg::PgConnection;

use crate::{error::AardwolfFail, traits::DbAction};

#[derive(Debug, Deserialize)]
/// The token type required to confirm an account
///
/// It contains the email's ID and an email verification token
pub struct ConfirmAccountToken {
    id: i32,
    token: EmailVerificationToken,
}

/// This operation confirms an unconfirmed account
///
/// It will fail if an account is already confirmed.
pub struct ConfirmAccount(pub ConfirmAccountToken);

impl DbAction for ConfirmAccount {
    type Item = AuthenticatedUser;
    type Error = ConfirmAccountFail;

    fn db_action(self, conn: &PgConnection) -> Result<AuthenticatedUser, ConfirmAccountFail> {
        let (unauthenticated_user, email) = UnauthenticatedUser::by_email_id(self.0.id, conn)
            .map_err(|_| ConfirmAccountFail::EmailNotFound)?;

        info!(
            "Found user and email, {:?} - {:?}",
            unauthenticated_user, email
        );

        let user = match unauthenticated_user
            .into_verified(conn)
            .map_err(|_| ConfirmAccountFail::UserLookup)?
        {
            Ok(unauthenticated_user) => {
                error!("User already verified: {:?}", unauthenticated_user);
                return Err(ConfirmAccountFail::Confirmed);
            }
            Err(unverified_user) => unverified_user,
        };

        info!("User is not yet verified");

        let email = match email.into_verified() {
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
            .verify(email, self.0.token)
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

#[derive(Clone, Debug, Fail, Serialize)]
/// An error representing different ways confirming an account can fail
pub enum ConfirmAccountFail {
    #[fail(display = "email was not found")]
    /// The email being confirmed doesn't exist
    EmailNotFound,

    #[fail(display = "account already confirmed")]
    /// The email or user has alraedy been verified
    Confirmed,

    #[fail(display = "Failed to lookup newly created user")]
    /// There was an error determining if the user is already verified or not
    UserLookup,

    #[fail(display = "Failed to verify email")]
    /// There was an error performing the verification operation
    Verify,
}

impl AardwolfFail for ConfirmAccountFail {}

#[cfg(test)]
mod tests {
    use aardwolf_models::user::email::{EmailToken, UnverifiedEmail};
    use aardwolf_test_helpers::models::{
        transmute_email_token, with_connection, with_unverified_email, with_unverified_user,
    };
    use diesel::pg::PgConnection;
    use failure::Error;

    use crate::{
        operations::confirm_account::{ConfirmAccount, ConfirmAccountToken},
        traits::DbAction,
    };

    fn setup<F>(f: F)
    where
        F: FnOnce(&PgConnection, UnverifiedEmail, EmailToken) -> Result<(), Error>,
    {
        with_connection(|conn| setup_with_conn(conn, f))
    }

    fn setup_with_conn<F>(conn: &PgConnection, f: F) -> Result<(), Error>
    where
        F: FnOnce(&PgConnection, UnverifiedEmail, EmailToken) -> Result<(), Error>,
    {
        with_unverified_user(conn, |user| {
            with_unverified_email(conn, &user, |email, token| f(conn, email, token))
        })
    }

    #[test]
    fn verifies_user_and_email() {
        setup(|conn, email, token| {
            let token = transmute_email_token(&token)?;

            let operation = ConfirmAccount(ConfirmAccountToken {
                id: email.id(),
                token,
            });

            assert!(operation.db_action(conn).is_ok());
            Ok(())
        })
    }

    #[test]
    fn verifies_two_users() {
        with_connection(|conn| {
            setup_with_conn(conn, |conn, email, token| {
                let token = transmute_email_token(&token)?;

                let operation = ConfirmAccount(ConfirmAccountToken {
                    id: email.id(),
                    token,
                });

                assert!(operation.db_action(conn).is_ok());
                Ok(())
            })?;

            setup_with_conn(conn, |conn, email, token| {
                let token = transmute_email_token(&token)?;

                let operation = ConfirmAccount(ConfirmAccountToken {
                    id: email.id(),
                    token,
                });

                assert!(operation.db_action(conn).is_ok());
                Ok(())
            })
        })
    }

    #[test]
    fn fails_to_verify_verified_user() {
        setup(|conn, email, email_token| {
            let token = transmute_email_token(&email_token)?;

            let operation = ConfirmAccount(ConfirmAccountToken {
                id: email.id(),
                token,
            });

            assert!(operation.db_action(conn).is_ok());

            let token = transmute_email_token(&email_token)?;

            let operation = ConfirmAccount(ConfirmAccountToken {
                id: email.id(),
                token,
            });

            assert!(operation.db_action(conn).is_err());

            Ok(())
        })
    }
}

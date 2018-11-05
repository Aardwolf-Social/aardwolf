use aardwolf_models::user::{
    email::EmailVerificationToken,
    {AuthenticatedUser, UnauthenticatedUser},
};
use diesel::pg::PgConnection;

use crate::{error::AardwolfFail, forms::traits::DbAction};

#[derive(Clone, Debug, Fail, Serialize)]
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

impl AardwolfFail for ConfirmAccountFail {}

#[derive(Debug, Deserialize)]
#[cfg_attr(feature = "use-rocket", derive(FromForm))]
pub struct ConfirmationToken {
    id: i32,
    token: EmailVerificationToken,
}

pub struct ConfirmToken;

impl ConfirmToken {
    pub fn with(self, token: ConfirmationToken) -> ConfirmTokenOperation {
        ConfirmTokenOperation(token)
    }
}

pub struct ConfirmTokenOperation(ConfirmationToken);

impl DbAction<AuthenticatedUser, ConfirmAccountFail> for ConfirmTokenOperation {
    fn db_action(self, conn: &PgConnection) -> Result<AuthenticatedUser, ConfirmAccountFail> {
        let (unauthenticated_user, email) = UnauthenticatedUser::by_email_id(self.0.id, conn)
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

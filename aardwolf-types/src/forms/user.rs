use aardwolf_models::user::{email::Email, AuthenticatedUser, UserLike};
use diesel::pg::PgConnection;

use crate::{
    error::AardwolfFail,
    traits::DbAction,
    wrapper::{DbActionWrapper, Wrapped},
};

#[derive(Clone, Debug, Fail, Serialize)]
pub enum FetchUserFail {
    #[fail(display = "Error in database")]
    Database,
    #[fail(display = "User not found")]
    NotFound,
}

impl From<diesel::result::Error> for FetchUserFail {
    fn from(e: diesel::result::Error) -> Self {
        match e {
            diesel::result::Error::NotFound => FetchUserFail::NotFound,
            _ => FetchUserFail::Database,
        }
    }
}

impl AardwolfFail for FetchUserFail {}

pub struct FetchUser(pub i32);

impl Wrapped for FetchUser {
    type Wrapper = DbActionWrapper<Self, <Self as DbAction>::Item, <Self as DbAction>::Error>;
}

impl DbAction for FetchUser {
    type Item = AuthenticatedUser;
    type Error = FetchUserFail;

    fn db_action(self, conn: &PgConnection) -> Result<AuthenticatedUser, FetchUserFail> {
        AuthenticatedUser::get_authenticated_user_by_id(self.0, &conn).map_err(From::from)
    }
}

pub struct FetchUserAndEmail(pub i32);

impl Wrapped for FetchUserAndEmail {
    type Wrapper = DbActionWrapper<Self, <Self as DbAction>::Item, <Self as DbAction>::Error>;
}

impl DbAction for FetchUserAndEmail {
    type Item = (AuthenticatedUser, Email);
    type Error = FetchUserFail;

    fn db_action(self, conn: &PgConnection) -> Result<(AuthenticatedUser, Email), FetchUserFail> {
        let user = AuthenticatedUser::get_authenticated_user_by_id(self.0, &conn)?;

        let email = match user.primary_email() {
            Some(primary_email) => Email::by_id(primary_email, &conn)?,
            None => Email::first_by_user_id(self.0, &conn)?,
        };

        Ok((user, email))
    }
}

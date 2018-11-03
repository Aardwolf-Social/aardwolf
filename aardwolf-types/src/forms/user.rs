use aardwolf_models::user::{email::Email, AuthenticatedUser, UserLike};
use diesel::pg::PgConnection;

use crate::{
    error::{AardwolfError, AardwolfErrorKind},
    forms::traits::DbAction,
};

#[derive(Clone, Debug, Fail)]
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

impl AardwolfError for FetchUserFail {
    fn name(&self) -> &str {
        "User Lookup Fail"
    }

    fn kind(&self) -> AardwolfErrorKind {
        match *self {
            FetchUserFail::Database => AardwolfErrorKind::InternalServerError,
            FetchUserFail::NotFound => AardwolfErrorKind::NotFound,
        }
    }

    fn description(&self) -> String {
        format!("{}", self)
    }
}

pub struct FetchUser;

impl FetchUser {
    pub fn with(self, id: i32) -> FetchUserOperation {
        FetchUserOperation(id)
    }
}

pub struct FetchUserOperation(i32);

impl DbAction<AuthenticatedUser, FetchUserFail> for FetchUserOperation {
    fn db_action(self, conn: &PgConnection) -> Result<AuthenticatedUser, FetchUserFail> {
        AuthenticatedUser::get_authenticated_user_by_id(self.0, &conn).map_err(From::from)
    }
}

pub struct FetchUserAndEmail;

impl FetchUserAndEmail {
    pub fn with(self, id: i32) -> FetchUserAndEmailOperation {
        FetchUserAndEmailOperation(id)
    }
}

pub struct FetchUserAndEmailOperation(i32);

impl DbAction<(AuthenticatedUser, Email), FetchUserFail> for FetchUserAndEmailOperation {
    fn db_action(self, conn: &PgConnection) -> Result<(AuthenticatedUser, Email), FetchUserFail> {
        let user = AuthenticatedUser::get_authenticated_user_by_id(self.0, &conn)?;

        let email = match user.primary_email() {
            Some(primary_email) => Email::by_id(primary_email, &conn)?,
            None => Email::first_by_user_id(self.0, &conn)?,
        };

        Ok((user, email))
    }
}

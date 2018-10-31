use aardwolf_models::user::{email::Email, AuthenticatedUser, UserLike};
use diesel::pg::PgConnection;

use crate::{
    error::{AardwolfError, AardwolfErrorKind},
    forms::traits::DbAction,
};

#[derive(Clone, Debug, Fail)]
pub enum UserLookpFail {
    #[fail(display = "Error in database")]
    Database,
    #[fail(display = "User not found")]
    NotFound,
}

impl From<diesel::result::Error> for UserLookpFail {
    fn from(e: diesel::result::Error) -> Self {
        match e {
            diesel::result::Error::NotFound => UserLookpFail::NotFound,
            _ => UserLookpFail::Database,
        }
    }
}

impl AardwolfError for UserLookpFail {
    fn name(&self) -> &str {
        "User Lookup Fail"
    }

    fn kind(&self) -> AardwolfErrorKind {
        match *self {
            UserLookpFail::Database => AardwolfErrorKind::InternalServerError,
            UserLookpFail::NotFound => AardwolfErrorKind::NotFound,
        }
    }

    fn description(&self) -> String {
        format!("{}", self)
    }
}

pub struct GetUserById(i32);

impl GetUserById {
    pub fn new(id: i32) -> Self {
        GetUserById(id)
    }
}

impl DbAction<AuthenticatedUser, UserLookpFail> for GetUserById {
    fn db_action(self, conn: &PgConnection) -> Result<AuthenticatedUser, UserLookpFail> {
        AuthenticatedUser::get_authenticated_user_by_id(self.0, &conn).map_err(From::from)
    }
}

pub struct GetUserAndEmailById(i32);

impl GetUserAndEmailById {
    pub fn new(id: i32) -> Self {
        GetUserAndEmailById(id)
    }
}

impl DbAction<(AuthenticatedUser, Email), UserLookpFail> for GetUserAndEmailById {
    fn db_action(self, conn: &PgConnection) -> Result<(AuthenticatedUser, Email), UserLookpFail> {
        let user = AuthenticatedUser::get_authenticated_user_by_id(self.0, &conn)?;

        let email = match user.primary_email() {
            Some(primary_email) => Email::by_id(primary_email, &conn)?,
            None => Email::first_by_user_id(self.0, &conn)?,
        };

        Ok((user, email))
    }
}

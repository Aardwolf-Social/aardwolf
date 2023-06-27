use aardwolf_models::user::AuthenticatedUser;
use diesel::pg::PgConnection;

use crate::{error::AardwolfFail, traits::DbAction};

#[derive(Clone, Debug, Fail, Serialize)]
pub enum FetchAuthenticatedUserFail {
    #[fail(display = "Error in database")]
    Database,
    #[fail(display = "User not found")]
    NotFound,
}

impl From<diesel::result::Error> for FetchAuthenticatedUserFail {
    fn from(e: diesel::result::Error) -> Self {
        match e {
            diesel::result::Error::NotFound => FetchAuthenticatedUserFail::NotFound,
            _ => FetchAuthenticatedUserFail::Database,
        }
    }
}

impl AardwolfFail for FetchAuthenticatedUserFail {}

pub struct FetchAuthenticatedUser(pub i32);

impl DbAction for FetchAuthenticatedUser {
    type Item = AuthenticatedUser;
    type Error = FetchAuthenticatedUserFail;

    fn db_action(
        self,
        conn: &mut PgConnection,
    ) -> Result<AuthenticatedUser, FetchAuthenticatedUserFail> {
        AuthenticatedUser::get_authenticated_user_by_id(self.0, conn).map_err(From::from)
    }
}

#[cfg(test)]
mod tests {
    use aardwolf_models::user::UserLike;
    use aardwolf_test_helpers::models::{
        gen_string, make_unverified_authenticated_user, make_verified_authenticated_user,
        with_connection,
    };

    use crate::{operations::fetch_authenticated_user::FetchAuthenticatedUser, traits::DbAction};

    #[test]
    fn fetches_verified_user() {
        with_connection(|conn| {
            let (user, email) = make_verified_authenticated_user(conn, &gen_string())?;
            let operation = FetchAuthenticatedUser(user.id());

            assert!(operation.db_action(conn).is_ok());
            Ok(())
        })
    }

    #[test]
    fn fetches_unverified_user() {
        with_connection(|conn| {
            let user = make_unverified_authenticated_user(conn, &gen_string())?;
            let operation = FetchAuthenticatedUser(user.id());

            assert!(operation.db_action(conn).is_ok());
            Ok(())
        })
    }

    #[test]
    fn doesnt_fetch_nonexistant_user() {
        with_connection(|conn| {
            let user = make_unverified_authenticated_user(conn, &gen_string())?;
            let operation = FetchAuthenticatedUser(user.id() + 1);

            assert!(operation.db_action(conn).is_err());
            Ok(())
        })
    }
}

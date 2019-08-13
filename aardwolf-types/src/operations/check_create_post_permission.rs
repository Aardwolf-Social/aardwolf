use aardwolf_models::{
    base_actor::BaseActor,
    user::{LocalPostCreator, PermissionError, PermissionedUser},
};
use diesel::pg::PgConnection;

use crate::{error::AardwolfFail, traits::DbAction};

pub struct CheckCreatePostPermission<U>(pub U, pub BaseActor)
where
    U: PermissionedUser + Clone;

impl<U> DbAction for CheckCreatePostPermission<U>
where
    U: PermissionedUser + Clone,
{
    type Item = LocalPostCreator;
    type Error = CheckCreatePostPermissionFail;

    fn db_action(self, conn: &PgConnection) -> Result<Self::Item, Self::Error> {
        Ok(self.0.can_post(self.1, conn)?)
    }
}

#[derive(Clone, Debug, Fail, Serialize)]
pub enum CheckCreatePostPermissionFail {
    #[fail(display = "Could not check user permissions")]
    /// There was an error checking the permission of the user
    Database,

    #[fail(display = "User does not haver permission to create persona")]
    /// The user doesn't have permission to create a persona
    Permission,
}

impl From<PermissionError> for CheckCreatePostPermissionFail {
    fn from(e: PermissionError) -> Self {
        match e {
            PermissionError::Diesel => CheckCreatePostPermissionFail::Database,
            PermissionError::Permission => CheckCreatePostPermissionFail::Permission,
        }
    }
}

impl AardwolfFail for CheckCreatePostPermissionFail {}

#[cfg(test)]
mod tests {
    use aardwolf_test_helpers::models::{
        gen_string, make_verified_authenticated_user, make_verified_user_with_persona,
        with_base_actor, with_connection,
    };

    use crate::{
        operations::check_create_post_permission::CheckCreatePostPermission, traits::DbAction,
    };

    #[test]
    fn verified_user_can_create_post() {
        with_connection(|conn| {
            make_verified_user_with_persona(conn, &gen_string()?, |user, actor, _persona| {
                let operation = CheckCreatePostPermission(user, actor);

                assert!(operation.db_action(conn).is_ok());
                Ok(())
            })
        })
    }

    #[test]
    fn verified_user_cannot_impersonate_persona() {
        with_connection(|conn| {
            make_verified_authenticated_user(conn, &gen_string()?, |user, _email| {
                with_base_actor(conn, |actor| {
                    let operation = CheckCreatePostPermission(user, actor);

                    assert!(operation.db_action(conn).is_err());
                    Ok(())
                })
            })
        })
    }
}

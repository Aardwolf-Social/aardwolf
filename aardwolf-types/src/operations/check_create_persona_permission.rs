use aardwolf_models::user::{LocalPersonaCreator, PermissionError, PermissionedUser};
use diesel::pg::PgConnection;

use crate::{error::AardwolfFail, traits::DbAction};

/// This operation checks whether a user has permissiont to create a persona
///
/// If the user does have permission, a persona creator is returned.
pub struct CheckCreatePersonaPermission<U>(pub U)
where
    U: PermissionedUser + Clone;

impl<U> DbAction for CheckCreatePersonaPermission<U>
where
    U: PermissionedUser + Clone + Send + 'static,
{
    type Item = LocalPersonaCreator<U>;
    type Error = CheckCreatePersonaPermissionFail;

    fn db_action(
        self,
        conn: &mut PgConnection,
    ) -> Result<LocalPersonaCreator<U>, CheckCreatePersonaPermissionFail> {
        Ok(self.0.can_make_persona(conn)?)
    }
}

#[derive(Clone, Debug, Fail, Serialize)]
pub enum CheckCreatePersonaPermissionFail {
    #[fail(display = "Could not check user permissions")]
    /// There was an error checking the permission of the user
    Database,

    #[fail(display = "User does not haver permission to create persona")]
    /// The user doesn't have permission to create a persona
    Permission,
}

impl From<PermissionError> for CheckCreatePersonaPermissionFail {
    fn from(e: PermissionError) -> Self {
        match e {
            PermissionError::Diesel => CheckCreatePersonaPermissionFail::Database,
            PermissionError::Permission => CheckCreatePersonaPermissionFail::Permission,
        }
    }
}

impl AardwolfFail for CheckCreatePersonaPermissionFail {}

#[cfg(test)]
mod tests {
    use aardwolf_test_helpers::models::{
        gen_string, make_unverified_authenticated_user, make_verified_authenticated_user,
        with_connection,
    };

    use crate::{
        operations::check_create_persona_permission::CheckCreatePersonaPermission, traits::DbAction,
    };

    #[test]
    fn verified_user_can_create_persona() {
        with_connection(|conn| {
            let (user, _) = make_verified_authenticated_user(conn, &gen_string())?;
            let operation = CheckCreatePersonaPermission(user);

            assert!(operation.db_action(conn).is_ok());
            Ok(())
        })
    }

    #[test]
    fn unverified_user_cannot_create_persona() {
        with_connection(|conn| {
            let user = make_unverified_authenticated_user(conn, &gen_string())?;
            let operation = CheckCreatePersonaPermission(user);

            assert!(operation.db_action(conn).is_err());
            Ok(())
        })
    }
}

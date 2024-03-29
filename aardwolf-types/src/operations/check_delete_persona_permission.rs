use aardwolf_models::{
    base_actor::persona::Persona,
    user::{AuthenticatedUser, PermissionError, PermissionedUser, PersonaDeleter},
};
use diesel::pg::PgConnection;

use crate::{error::AardwolfFail, traits::DbAction};
use thiserror::Error;

pub struct CheckDeletePersonaPermission(pub AuthenticatedUser, pub Persona);

impl DbAction for CheckDeletePersonaPermission {
    type Item = PersonaDeleter;
    type Error = CheckDeletePersonaPermissionFail;

    fn db_action(
        self,
        conn: &mut PgConnection,
    ) -> Result<PersonaDeleter, CheckDeletePersonaPermissionFail> {
        Ok(self.0.can_delete_persona(self.1, conn)?)
    }
}

#[derive(Debug, Clone, Error, Serialize)]
pub enum CheckDeletePersonaPermissionFail {
    #[error("User does not have permission to delete persona")]
    Permission,
    #[error("Error accessing database to check permissions")]
    Database,
}

impl From<PermissionError> for CheckDeletePersonaPermissionFail {
    fn from(e: PermissionError) -> Self {
        match e {
            PermissionError::Diesel => CheckDeletePersonaPermissionFail::Database,
            PermissionError::Permission => CheckDeletePersonaPermissionFail::Permission,
        }
    }
}

impl AardwolfFail for CheckDeletePersonaPermissionFail {}

#[cfg(test)]
mod tests {
    use aardwolf_models::{base_actor::persona::Persona, user::AuthenticatedUser};
    use aardwolf_test_helpers::models::{
        gen_string, make_persona, make_unverified_authenticated_user,
        make_verified_authenticated_user, user_make_base_actor, with_connection,
    };
    use anyhow::Error;
    use diesel::pg::PgConnection;

    use crate::{
        operations::check_delete_persona_permission::CheckDeletePersonaPermission, traits::DbAction,
    };

    fn setup_with_connection<F>(conn: &mut PgConnection, f: F) -> Result<(), Error>
    where
        F: FnOnce(&mut PgConnection, AuthenticatedUser, Persona) -> Result<(), Error>,
    {
        let (user, email) = make_verified_authenticated_user(conn, &gen_string())?;
        let base_actor = user_make_base_actor(conn, &user)?;
        let persona = make_persona(conn, &base_actor)?;

        f(conn, user, persona)
    }

    #[test]
    fn verified_user_can_delete_their_persona() {
        with_connection(|conn| {
            setup_with_connection(conn, |conn, user, persona| {
                let operation = CheckDeletePersonaPermission(user, persona);

                assert!(operation.db_action(conn).is_ok());
                Ok(())
            })
        })
    }

    #[test]
    fn verified_user_cannot_delete_another_users_persona() {
        with_connection(|conn| {
            setup_with_connection(conn, |conn, _user, persona| {
                let (user2, _email) = make_verified_authenticated_user(conn, &gen_string())?;
                let operation = CheckDeletePersonaPermission(user2, persona);

                assert!(operation.db_action(conn).is_err());
                Ok(())
            })
        })
    }

    #[test]
    fn unverified_user_cannot_delete_another_users_persona() {
        with_connection(|conn| {
            setup_with_connection(conn, |conn, _user, persona| {
                let user2 = make_unverified_authenticated_user(conn, &gen_string())?;
                let operation = CheckDeletePersonaPermission(user2, persona);

                assert!(operation.db_action(conn).is_err());
                Ok(())
            })
        })
    }
}

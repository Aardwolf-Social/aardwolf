use aardwolf_models::user::PersonaDeleter;
use diesel::{pg::PgConnection, result::Error as DieselError};

use crate::{
    error::AardwolfFail,
    operations::{
        check_delete_persona_permission::CheckDeletePersonaPermissionFail,
        fetch_persona::FetchPersonaFail,
    },
    traits::DbAction,
};

pub struct DeletePersona(pub PersonaDeleter);

impl DbAction for DeletePersona {
    type Item = ();
    type Error = DeletePersonaFail;

    fn db_action(self, conn: &PgConnection) -> Result<(), DeletePersonaFail> {
        self.0.delete_persona(conn).map_err(From::from)
    }
}

#[derive(Clone, Debug, Fail, Serialize)]
pub enum DeletePersonaFail {
    #[fail(display = "Insufficient permissions")]
    Permission,
    #[fail(display = "Error in database")]
    Database,
    #[fail(display = "Persona not found")]
    NotFound,
}

impl From<DieselError> for DeletePersonaFail {
    fn from(e: DieselError) -> Self {
        match e {
            DieselError::NotFound => DeletePersonaFail::NotFound,
            _ => DeletePersonaFail::Database,
        }
    }
}

impl From<CheckDeletePersonaPermissionFail> for DeletePersonaFail {
    fn from(e: CheckDeletePersonaPermissionFail) -> Self {
        match e {
            CheckDeletePersonaPermissionFail::Permission => DeletePersonaFail::Permission,
            CheckDeletePersonaPermissionFail::Database => DeletePersonaFail::Database,
        }
    }
}

impl From<FetchPersonaFail> for DeletePersonaFail {
    fn from(e: FetchPersonaFail) -> Self {
        match e {
            FetchPersonaFail::Database => DeletePersonaFail::Database,
            FetchPersonaFail::NotFound => DeletePersonaFail::NotFound,
        }
    }
}

impl AardwolfFail for DeletePersonaFail {}

#[cfg(test)]
mod tests {
    use aardwolf_models::user::PermissionedUser;
    use aardwolf_test_helpers::models::{
        gen_string, make_verified_authenticated_user, user_with_base_actor, with_connection,
        with_persona,
    };

    use crate::{operations::delete_persona::DeletePersona, traits::DbAction};

    #[test]
    fn deleting_persona_works() {
        with_connection(|conn| {
            make_verified_authenticated_user(conn, &gen_string()?, |user, _email| {
                user_with_base_actor(conn, &user, |base_actor| {
                    with_persona(conn, &base_actor, |persona| {
                        let deleter = user.can_delete_persona(persona, conn)?;

                        let operation = DeletePersona(deleter);

                        assert!(operation.db_action(conn).is_ok());
                        Ok(())
                    })
                })
            })
        })
    }
}

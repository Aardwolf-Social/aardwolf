use aardwolf_models::base_actor::persona::Persona;
use diesel::{pg::PgConnection, result::Error as DieselError};

use crate::{
    error::AardwolfFail,
    traits::DbAction,
    wrapper::{DbActionWrapper, Wrapped},
};

pub struct FetchPersona(pub i32);

impl Wrapped for FetchPersona {
    type Wrapper = DbActionWrapper<Self, <Self as DbAction>::Item, <Self as DbAction>::Error>;
}

impl DbAction for FetchPersona {
    type Item = Persona;
    type Error = FetchPersonaFail;

    fn db_action(self, conn: &PgConnection) -> Result<Persona, FetchPersonaFail> {
        Persona::by_id(self.0, conn).map_err(From::from)
    }
}

#[derive(Clone, Debug, Fail, Serialize)]
pub enum FetchPersonaFail {
    #[fail(display = "Error in database")]
    Database,
    #[fail(display = "Persona not found")]
    NotFound,
}

impl From<DieselError> for FetchPersonaFail {
    fn from(e: DieselError) -> Self {
        match e {
            DieselError::NotFound => FetchPersonaFail::NotFound,
            _ => FetchPersonaFail::Database,
        }
    }
}

impl AardwolfFail for FetchPersonaFail {}

#[cfg(test)]
mod tests {
    use aardwolf_models::base_actor::persona::Persona;
    use aardwolf_test_helpers::models::{with_base_actor, with_connection, with_persona};
    use diesel::pg::PgConnection;
    use failure::Error;

    use crate::{operations::fetch_persona::FetchPersona, traits::DbAction};

    fn setup<F>(f: F)
    where
        F: FnOnce(&PgConnection, Persona) -> Result<(), Error>,
    {
        with_connection(|conn| {
            with_base_actor(conn, |base_actor| {
                with_persona(conn, &base_actor, |persona| f(conn, persona))
            })
        })
    }

    #[test]
    fn fetches_persona() {
        setup(|conn, persona| {
            let operation = FetchPersona(persona.id());

            assert!(operation.db_action(conn).is_ok());
            Ok(())
        })
    }

    #[test]
    fn doesnt_fetch_invalid_persona() {
        setup(|conn, persona| {
            let operation = FetchPersona(persona.id() + 1);

            assert!(operation.db_action(conn).is_err());
            Ok(())
        })
    }
}

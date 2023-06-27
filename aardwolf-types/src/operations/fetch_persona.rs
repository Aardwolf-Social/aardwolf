use aardwolf_models::base_actor::persona::Persona;
use diesel::{pg::PgConnection, result::Error as DieselError};

use crate::{error::AardwolfFail, traits::DbAction};

pub struct FetchPersona(pub i32);

impl DbAction for FetchPersona {
    type Item = Persona;
    type Error = FetchPersonaFail;

    fn db_action(self, conn: &mut PgConnection) -> Result<Persona, FetchPersonaFail> {
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
    use aardwolf_test_helpers::models::{make_base_actor, make_persona, with_connection};
    use diesel::pg::PgConnection;
    use failure::Error;

    use crate::{operations::fetch_persona::FetchPersona, traits::DbAction};

    fn setup<F>(f: F)
    where
        F: FnOnce(&mut PgConnection, Persona) -> Result<(), Error>,
    {
        with_connection(|conn| {
            let base_actor = make_base_actor(conn)?;
            let persona = make_persona(conn, &base_actor)?;

            f(conn, persona)
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

use aardwolf_models::base_actor::BaseActor;
use diesel::{pg::PgConnection, result::Error as DieselError};
use thiserror::Error;

use crate::{error::AardwolfFail, traits::DbAction};

pub struct FetchBaseActor(pub i32);

impl DbAction for FetchBaseActor {
    type Item = BaseActor;
    type Error = FetchBaseActorFail;

    fn db_action(self, conn: &mut PgConnection) -> Result<BaseActor, FetchBaseActorFail> {
        BaseActor::by_persona_id(self.0, conn).map_err(From::from)
    }
}

#[derive(Clone, Debug, Error, Serialize)]
pub enum FetchBaseActorFail {
    #[error("Error in database")]
    Database,
    #[error("BaseActor not found")]
    NotFound,
}

impl From<DieselError> for FetchBaseActorFail {
    fn from(e: DieselError) -> Self {
        match e {
            DieselError::NotFound => FetchBaseActorFail::NotFound,
            _ => FetchBaseActorFail::Database,
        }
    }
}

impl AardwolfFail for FetchBaseActorFail {}

#[cfg(test)]
mod tests {
    use aardwolf_models::base_actor::persona::Persona;
    use aardwolf_test_helpers::models::{make_base_actor, make_persona, with_connection};
    use anyhow::Error;
    use diesel::pg::PgConnection;

    use crate::{operations::fetch_base_actor::FetchBaseActor, traits::DbAction};

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
    fn fetches_base_actor() {
        setup(|conn, persona| {
            let operation = FetchBaseActor(persona.id());

            assert!(operation.db_action(conn).is_ok());
            Ok(())
        })
    }

    #[test]
    fn doesnt_fetch_invalid_base_actor() {
        setup(|conn, persona| {
            let operation = FetchBaseActor(persona.id() + 1);

            assert!(operation.db_action(conn).is_err());
            Ok(())
        })
    }
}

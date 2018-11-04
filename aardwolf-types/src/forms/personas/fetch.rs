use aardwolf_models::base_actor::persona::Persona;
use diesel::{pg::PgConnection, result::Error as DieselError};

use crate::{error::AardwolfFail, forms::traits::DbAction};

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

pub struct FetchPersona;

impl FetchPersona {
    pub fn with(self, id: i32) -> FetchPersonaOperation {
        FetchPersonaOperation(id)
    }
}

pub struct FetchPersonaOperation(i32);

impl DbAction<Persona, FetchPersonaFail> for FetchPersonaOperation {
    fn db_action(self, conn: &PgConnection) -> Result<Persona, FetchPersonaFail> {
        Persona::by_id(self.0, conn).map_err(From::from)
    }
}

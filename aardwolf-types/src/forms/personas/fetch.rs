use aardwolf_models::base_actor::persona::Persona;
use diesel::{pg::PgConnection, result::Error as DieselError};

use crate::{
    error::AardwolfFail,
    traits::DbAction,
    wrapper::{DbActionWrapper, Wrapped},
};

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

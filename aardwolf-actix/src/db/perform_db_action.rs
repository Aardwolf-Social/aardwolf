use std::marker::PhantomData;

use aardwolf_types::forms::traits::DbAction;
use actix_web::error::ResponseError;
use crate::actix::{Handler, Message};
use failure::Fail;

use crate::db::Db;

#[derive(Debug, Fail)]
pub enum DbActionError<E>
where
    E: Fail,
{
    #[fail(display = "Error in action {}", _0)]
    Action(#[cause] E),
    #[fail(display = "Error in connection {}", _0)]
    Connection(#[cause] r2d2::Error),
}

impl<E> ResponseError for DbActionError<E> where E: Fail {}

impl<E> From<r2d2::Error> for DbActionError<E>
where
    E: Fail,
{
    fn from(e: r2d2::Error) -> Self {
        DbActionError::Connection(e)
    }
}

pub struct PerformDbAction<D, T, E>
where
    D: DbAction<T, E>,
    E: Fail,
{
    db_action: D,
    item: PhantomData<T>,
    error: PhantomData<E>,
}

impl<D, T, E> PerformDbAction<D, T, E>
where
    D: DbAction<T, E>,
    E: Fail,
{
    pub fn new(db_action: D) -> Self {
        PerformDbAction {
            db_action,
            item: PhantomData,
            error: PhantomData,
        }
    }
}

impl<D, T, E> Message for PerformDbAction<D, T, E>
where
    D: DbAction<T, E>,
    E: Fail,
    T: 'static,
{
    type Result = Result<T, DbActionError<E>>;
}

impl<D, T, E> Handler<PerformDbAction<D, T, E>> for Db
where
    D: DbAction<T, E>,
    E: Fail,
    T: 'static,
{
    type Result = <PerformDbAction<D, T, E> as Message>::Result;

    fn handle(&mut self, msg: PerformDbAction<D, T, E>, _: &mut Self::Context) -> Self::Result {
        let conn = self.get_connection()?;

        msg.db_action
            .db_action(&conn)
            .map_err(DbActionError::Action)
    }
}

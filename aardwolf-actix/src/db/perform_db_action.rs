use std::marker::PhantomData;

use crate::actix::{Handler, MailboxError, Message};
use aardwolf_types::{error::AardwolfFail, traits::DbAction};
use failure::Fail;
use serde_derive::Serialize;

use crate::db::Db;

#[derive(Clone, Debug, Fail, Serialize)]
pub enum DbActionError<E>
where
    E: AardwolfFail,
{
    #[fail(display = "Error in action {}", _0)]
    Action(#[cause] E),
    #[fail(display = "Error in connection to database")]
    Connection,
    #[fail(display = "Error communicating db actor")]
    Mailbox,
}

impl<E> DbActionError<E>
where
    E: AardwolfFail,
{
    pub fn map_err<F>(self) -> DbActionError<F>
    where
        F: AardwolfFail + From<E>,
    {
        match self {
            DbActionError::Action(e) => DbActionError::Action(e.into()),
            DbActionError::Connection => DbActionError::Connection,
            DbActionError::Mailbox => DbActionError::Mailbox,
        }
    }
}

impl<E> From<MailboxError> for DbActionError<E>
where
    E: AardwolfFail,
{
    fn from(_: MailboxError) -> Self {
        DbActionError::Mailbox
    }
}

impl<E> AardwolfFail for DbActionError<E> where E: AardwolfFail {}

impl<E> From<r2d2::Error> for DbActionError<E>
where
    E: AardwolfFail,
{
    fn from(_: r2d2::Error) -> Self {
        DbActionError::Connection
    }
}

pub struct PerformDbAction<D, T, E>
where
    D: DbAction<Item = T, Error = E>,
    E: AardwolfFail,
{
    db_action: D,
    item: PhantomData<T>,
    error: PhantomData<E>,
}

impl<D, T, E> PerformDbAction<D, T, E>
where
    D: DbAction<Item = T, Error = E>,
    E: AardwolfFail,
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
    D: DbAction<Item = T, Error = E>,
    E: AardwolfFail,
    T: 'static,
{
    type Result = Result<T, DbActionError<E>>;
}

impl<D, T, E> Handler<PerformDbAction<D, T, E>> for Db
where
    D: DbAction<Item = T, Error = E>,
    E: AardwolfFail,
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

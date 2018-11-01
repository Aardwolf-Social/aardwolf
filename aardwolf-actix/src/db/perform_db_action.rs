use std::marker::PhantomData;

use aardwolf_types::{
    error::{AardwolfError, AardwolfErrorKind, TemplateError},
    forms::traits::DbAction,
};
use crate::actix::{Handler, Message};
use failure::Fail;
use futures::Future;

use crate::{db::Db, error::ErrorWrapper, AppConfig};

pub fn execute_db_query<D, T, E>(
    state: AppConfig,
    db_action: D,
) -> impl Future<Item = T, Error = actix_web::error::Error>
where
    D: DbAction<T, E> + Send + 'static,
    E: AardwolfError + Clone,
    ErrorWrapper<DbActionError<E>>: TemplateError + Clone,
    T: Send + 'static,
{
    state
        .db
        .send(PerformDbAction::new(db_action))
        .then(|res| match res {
            Ok(item_res) => match item_res {
                Ok(item) => Ok(item),
                Err(e) => Err(ErrorWrapper::new(state, e).into()),
            },
            Err(e) => Err(e.into()),
        })
}

#[derive(Clone, Debug, Fail)]
pub enum DbActionError<E>
where
    E: Fail,
{
    #[fail(display = "Error in action {}", _0)]
    Action(#[cause] E),
    #[fail(display = "Error in connection")]
    Connection,
}

impl<E> AardwolfError for DbActionError<E>
where
    E: AardwolfError,
{
    fn name(&self) -> &str {
        "Database Action Error"
    }

    fn kind(&self) -> AardwolfErrorKind {
        match *self {
            DbActionError::Connection => AardwolfErrorKind::InternalServerError,
            DbActionError::Action(ref e) => e.kind(),
        }
    }

    fn description(&self) -> String {
        format!("{}", self)
    }
}

impl<E> From<r2d2::Error> for DbActionError<E>
where
    E: Fail,
{
    fn from(_: r2d2::Error) -> Self {
        DbActionError::Connection
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

use failure::Fail;
use std::sync::Arc;
use tokio::sync::Mutex;

pub trait Validate {
    type Item;
    type Error: Fail;

    fn validate(self) -> Result<Self::Item, Self::Error>;
}

#[cfg(not(feature = "with-actix"))]
pub use default_impls::DbAction;

#[cfg(feature = "with-actix")]
pub use actix_web_impls::{DbAction, DbActionError};

#[cfg(not(feature = "with-actix"))]
mod default_impls {
    use diesel::pg::PgConnection;

    pub trait DbAction {
        type Item;
        type Error: Fail;

        fn db_action(self, conn: &PgConnection) -> Result<Self::Item, Self::Error>;
    }
}

#[cfg(feature = "with-actix")]
mod actix_web_impls {
    use actix_rt::blocking::BlockingError;
    use actix_web::web::block;
    use diesel::{
        r2d2::{self, ConnectionManager, Pool},
        PgConnection,
    };
    use failure::Fail;
    use futures::{future::BoxFuture, FutureExt, TryFutureExt};
    use r2d2::PooledConnection;

    #[derive(Debug, Fail)]
    pub enum DbActionError<E>
    where
        E: Fail,
    {
        #[fail(display = "Error in Db Action: {}", _0)]
        Error(#[cause] E),

        #[fail(display = "Error in pooling: {}", _0)]
        Pool(#[cause] r2d2::Error),

        #[fail(display = "Db Action was canceled")]
        Canceled,
    }

    impl<E> From<r2d2::Error> for DbActionError<E>
    where
        E: Fail,
    {
        fn from(e: r2d2::Error) -> Self {
            DbActionError::Pool(e)
        }
    }

    impl<E> From<BlockingError<DbActionError<E>>> for DbActionError<E>
    where
        E: Fail,
    {
        fn from(e: BlockingError<DbActionError<E>>) -> Self {
            match e {
                BlockingError::Error(err) => err,
                BlockingError::Canceled => DbActionError::Canceled,
            }
        }
    }

    pub trait DbAction {
        type Item: Send + 'static;
        type Error: Fail;

        fn db_action(self, conn: &mut PgConnection) -> Result<Self::Item, Self::Error>;

        fn run(
            self,
            pool: Arc<Mutex<Pool<ConnectionManager<PgConnection>>>>,
        ) -> BoxFuture<'static, Result<Self::Item, DbActionError<Self::Error>>>
        where
            Self: Sized + Send + 'static,
        {
            let pool = pool.clone();
            Box::pin(async move {
                let mut conn = pool.lock().await.map_err(DbActionError::Pool)?.get().map_err(DbActionError::Pool)?;
                self.db_action(&mut conn).map_err(DbActionError::Error)
            })
        }
    }
}
pub trait Validate {
    type Item;
    type Error: std::error::Error;

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
    use actix_web::{error::BlockingError, web::block};
    use diesel::r2d2::ConnectionManager;
    use diesel::PgConnection;
    use futures::future::{BoxFuture, FutureExt, TryFutureExt};
    use r2d2::Pool;
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum DbActionError<E>
    where
        E: std::error::Error,
    {
        #[error("Error in Db Action, {}", _0)]
        Error(#[source] E),

        #[error("Error in pooling, {}", _0)]
        Pool(#[source] r2d2::Error),

        #[error("Error in thread, {}", _0)]
        Thread(#[source] BlockingError),

        #[error("Db Action was canceled")]
        Canceled,
    }

    impl<E> From<BlockingError> for DbActionError<E>
    where
        E: std::error::Error,
    {
        fn from(e: BlockingError) -> Self {
            DbActionError::Thread(e)
        }
    }

    impl<E> From<r2d2::Error> for DbActionError<E>
    where
        E: std::error::Error,
    {
        fn from(e: r2d2::Error) -> Self {
            DbActionError::Pool(e)
        }
    }

    pub trait DbAction {
        type Item: Send + 'static;
        type Error: std::error::Error + Send;

        fn db_action(self, conn: &mut PgConnection) -> Result<Self::Item, Self::Error>;

        fn run(
            self,
            pool: Pool<ConnectionManager<PgConnection>>,
        ) -> BoxFuture<'static, Result<Self::Item, DbActionError<Self::Error>>>
        where
            Self: Sized + Send + 'static,
        {
            let result = block(move || -> Result<Self::Item, DbActionError<Self::Error>> {
                let conn = &mut *pool.get()?;

                self.db_action(conn).map_err(DbActionError::Error)
            })
            .map_err(DbActionError::from);

            // Flatten nested result
            let result = result.map(|result| result.and_then(|inner| inner));

            result.boxed()
        }
    }
}

use failure::Fail;

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
    use actix_web::{error::BlockingError, web::block};
    use diesel::r2d2::ConnectionManager;
    use diesel::PgConnection;
    use failure::Fail;
    use futures::{
        compat::Future01CompatExt,
        future::{BoxFuture, FutureExt, TryFutureExt},
    };
    use r2d2::Pool;

    #[derive(Debug, Fail)]
    pub enum DbActionError<E>
    where
        E: Fail,
    {
        #[fail(display = "Error in Db Action, {}", _0)]
        Error(#[cause] E),

        #[fail(display = "Error in pooling, {}", _0)]
        Pool(#[cause] r2d2::Error),

        #[fail(display = "Db Action was canceled")]
        Canceled,
    }

    impl<E> From<BlockingError<E>> for DbActionError<E>
    where
        E: Fail,
    {
        fn from(e: BlockingError<E>) -> Self {
            match e {
                BlockingError::Error(e) => DbActionError::Error(e),
                BlockingError::Canceled => DbActionError::Canceled,
            }
        }
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
                BlockingError::Error(e) => e,
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
            pool: Pool<ConnectionManager<PgConnection>>,
        ) -> BoxFuture<'static, Result<Self::Item, DbActionError<Self::Error>>>
        where
            Self: Sized + Send + 'static,
        {
            block::<_, _, DbActionError<Self::Error>>(move || {
                let conn = &mut *pool.get()?;
                let res = self.db_action(conn).map_err(DbActionError::Error)?;
                Ok(res)
            })
            .compat()
            .map_err(DbActionError::from)
            .boxed()
        }
    }
}

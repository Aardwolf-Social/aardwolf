use aardwolf_templates::Renderable;
use aardwolf_types::{
    error::AardwolfFail,
    traits::{DbAction, Export, Validate},
    wrapper::{DbActionWrapper, ExportFail, ExportWrapper, ValidateWrapper},
};
use actix_web::{http::header::LOCATION, HttpResponse};
use failure::Fail;
use futures::{
    compat::Future01CompatExt,
    future::{ready, BoxFuture, FutureExt, Ready},
};
use std::{fmt, future::Future};

use crate::{
    db::{DbActionError, PerformDbAction},
    AppConfig, WithRucte,
};

pub use aardwolf_types::wrapper::Wrapped;

#[derive(Clone, Fail)]
pub enum Impossible {}

impl fmt::Display for Impossible {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Not possible...")
    }
}

impl fmt::Debug for Impossible {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Not possible...")
    }
}

pub enum Respond<R>
where
    R: Renderable,
{
    Ok(R),
    Created(R),
    NotFound(R),
}

impl<R> Wrapped for Respond<R>
where
    R: Renderable,
{
    type Wrapper = Respond<R>;
}

impl<R> Action<HttpResponse, Impossible> for Respond<R>
where
    R: Renderable,
{
    type Future = Ready<Result<HttpResponse, Impossible>>;

    fn action(self, _: AppConfig) -> Self::Future {
        let h = match self {
            Respond::Ok(r) => HttpResponse::Ok().with_ructe(r),
            Respond::Created(r) => HttpResponse::Created().with_ructe(r),
            Respond::NotFound(r) => HttpResponse::NotFound().with_ructe(r),
        };

        ready(Ok(h))
    }
}

pub struct Redirect(pub String);

impl Wrapped for Redirect {
    type Wrapper = Redirect;
}

impl Action<HttpResponse, Impossible> for Redirect {
    type Future = Ready<Result<HttpResponse, Impossible>>;

    fn action(self, _: AppConfig) -> Self::Future {
        ready(Ok(HttpResponse::SeeOther()
            .header(LOCATION, self.0)
            .finish()))
    }
}

pub trait Action<T, E>
where
    E: Fail,
{
    type Future: Future<Output = Result<T, E>>;

    fn action(self, state: AppConfig) -> Self::Future;
}

impl<E, T> Action<T, ExportFail> for ExportWrapper<E, T>
where
    E: Export<Item = T>,
    T: Send + 'static,
{
    type Future = Ready<Result<T, ExportFail>>;

    fn action(self, _: AppConfig) -> Self::Future {
        ready(Ok(self.0.export()))
    }
}

impl<V, T, E> Action<T, E> for ValidateWrapper<V, T, E>
where
    V: Validate<Item = T, Error = E>,
    T: Send + 'static,
    E: AardwolfFail,
{
    type Future = Ready<Result<T, E>>;

    fn action(self, _: AppConfig) -> Self::Future {
        ready(self.0.validate())
    }
}

async fn db_action_inner<D, T, E>(
    action: DbActionWrapper<D, T, E>,
    state: AppConfig,
) -> Result<T, DbActionError<E>>
where
    D: DbAction<Item = T, Error = E> + Send + 'static,
    T: Send + 'static,
    E: AardwolfFail,
{
    let res = state.db.send(PerformDbAction::new(action.0)).compat().await;

    match res {
        Ok(item_res) => match item_res {
            Ok(item) => Ok(item),
            Err(e) => Err(e),
        },
        Err(e) => Err(DbActionError::from(e)),
    }
}

impl<D, T, E> Action<T, DbActionError<E>> for DbActionWrapper<D, T, E>
where
    D: DbAction<Item = T, Error = E> + Send + 'static,
    T: Send + 'static,
    E: AardwolfFail,
{
    type Future = BoxFuture<'static, Result<T, DbActionError<E>>>;

    fn action(self, state: AppConfig) -> Self::Future {
        db_action_inner(self, state).boxed()
    }
}

#[macro_export]
/// The perform macro executes a series of `Action`s in order
///
/// It allows for fallible operations to be chained without the hassle of manually calling the
/// methods.
///
/// Example usage:
/// ```rust,ignore
/// async fn do_thing(form: Form, user: User, db: &PgConnection) -> Result<(), Error> {
///     perform!(db, [
///         (validated = ValidateForm(form)),
///         (updater = GetUpdater(user, validated)),
///         (_ = UpdateRecord(updater)),
///     ]);
/// }
/// ```
///
/// which could be expressed as the following without the macro
///
/// ```rust,ignore
/// async fn do_thing(form: Form, user: User, state: AppConfig) -> Result<(), Error> {
///     use futures::{Future, future::IntoFuture};
///     use aardwolf_types::traits::{Validate, DbAction};
///
///     let validated = ValidateForm(form).validate().await?;
///     let updater = GetUpdater(user, validated).db_action(state.clone()).await?;
///
///     UpdateRecord(updater).db_action(state.clone()).await?;
///
///     Ok(())
/// }
/// ```
macro_rules! perform {
 ( $state:expr, [] ) => {{
     ()
 }};
 ( $state:expr, [($store:pat = $operation:expr),] ) => {{
     use $crate::action::{Action, Wrapped};

     $operation
         .wrap()
         .action($state.clone())
         .await?
 }};
 (
     $state:expr,
     [
         ($store:pat = $operation:expr),
         $(($stores:pat = $operations:expr),)*
     ]
 ) => {{
     use $crate::action::{Action, Wrapped};

     let $store = $operation
         .wrap()
         .action($state.clone())
         .await?;

     perform!($state, [
          $(($stores = $operations),)*
     ])
 }};
}

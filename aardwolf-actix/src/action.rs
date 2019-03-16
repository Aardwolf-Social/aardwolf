use aardwolf_types::{
    error::AardwolfFail,
    traits::{DbAction, Export, Validate},
    wrapper::{DbActionWrapper, ExportFail, ExportWrapper, ValidateWrapper},
};
use futures::future::Future;

use crate::{
    db::{DbActionError, PerformDbAction},
    AppConfig,
};

pub use aardwolf_types::wrapper::Wrapped;

pub trait Action<T, E>
where
    E: AardwolfFail,
{
    fn action(self, state: AppConfig) -> Box<dyn Future<Item = T, Error = E> + Send>;
}

impl<E, T> Action<T, ExportFail> for ExportWrapper<E, T>
where
    E: Export<Item = T>,
    T: Send + 'static,
{
    fn action(self, _: AppConfig) -> Box<dyn Future<Item = T, Error = ExportFail> + Send> {
        use futures::future::IntoFuture;

        Box::new(Ok(self.0.export()).into_future())
    }
}

impl<V, T, E> Action<T, E> for ValidateWrapper<V, T, E>
where
    V: Validate<Item = T, Error = E>,
    T: Send + 'static,
    E: AardwolfFail,
{
    fn action(self, _: AppConfig) -> Box<dyn Future<Item = T, Error = E> + Send> {
        use futures::future::IntoFuture;

        Box::new(self.0.validate().into_future())
    }
}

impl<D, T, E> Action<T, DbActionError<E>> for DbActionWrapper<D, T, E>
where
    D: DbAction<Item = T, Error = E> + Send + 'static,
    T: Send + 'static,
    E: AardwolfFail,
{
    fn action(
        self,
        state: AppConfig,
    ) -> Box<dyn Future<Item = T, Error = DbActionError<E>> + Send> {
        let fut = state
            .db
            .send(PerformDbAction::new(self.0))
            .then(|res| match res {
                Ok(item_res) => match item_res {
                    Ok(item) => Ok(item),
                    Err(e) => Err(e),
                },
                Err(e) => Err(DbActionError::from(e)),
            });

        Box::new(fut)
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
/// fn do_thing(form: Form, user: User, db: &PgConnection) -> impl Future<Item = (), Error = Error> {
///     perform!(db, Error, [
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
/// fn do_thing(form: Form, user: User, state: AppConfig) -> Result<(), Error> {
///     use futures::{Future, future::IntoFuture};
///     use aardwolf_types::traits::{Validate, DbAction};
///
///     ValidateForm(form)
///         .validate()
///         .into_future()
///         .from_err::<Error>()
///         .and_then(move |validated| {
///             GetUpdater(user, validated)
///                 .db_action(state.clone())
///                 .from_err::<Error>()
///         })
///         .and_then(move |updater| {
///             UpdateRecord(updater)
///                 .db_action(state.clone())
///                 .from_err::<Error>()
///         })
///         .map(|_| ())
/// }
/// ```
macro_rules! perform {
 ( $state:expr, $error_type:ty, [] ) => {{
     use futures::future::IntoFuture;

     (Ok(()) as Result<(), $error_type>).into_future()
 }};
 ( $state:expr, $error_type:ty, [($store:pat = $operation:expr),] ) => {{
     use $crate::action::{Action, Wrapped};

     $operation
         .wrap()
         .action($state.clone())
         .from_err::<$error_type>()
 }};
 (
     $state:expr,
     $error_type:ty,
     [
         ($store:pat = $operation:expr),
         $(($stores:pat = $operations:expr),)*
     ]
 ) => {{
     use $crate::action::{Action, Wrapped};

     $operation
         .wrap()
         .action($state.clone())
         .from_err::<$error_type>()
         .and_then(move |item| {
             let $store = item;

             perform!($state, $error_type, [
                  $(($stores = $operations),)*
             ])
         })
 }};
}

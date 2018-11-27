use aardwolf_types::{
    error::AardwolfFail,
    traits::{DbAction, Validate},
    wrapper::{DbActionWrapper, ValidateWrapper},
};
use diesel::pg::PgConnection;

pub use aardwolf_types::wrapper::Wrapped;

pub trait Action<T, E>
where
    E: AardwolfFail,
{
    fn action(self, db: &PgConnection) -> Result<T, E>;
}

impl<V, T, E> Action<T, E> for ValidateWrapper<V, T, E>
where
    V: Validate<Item = T, Error = E>,
    T: Send + 'static,
    E: AardwolfFail,
{
    fn action(self, _: &PgConnection) -> Result<T, E> {
        self.0.validate()
    }
}

impl<D, T, E> Action<T, E> for DbActionWrapper<D, T, E>
where
    D: DbAction<Item = T, Error = E> + Send + 'static,
    T: Send + 'static,
    E: AardwolfFail,
{
    fn action(self, db: &PgConnection) -> Result<T, E> {
        self.0.db_action(db)
    }
}

#[macro_export]
/// The perform macro executes a series of `Action`s in order
///
/// It allows for fallible operations to be chained without the hassle of manually calling the
/// methods.
///
/// For synchronous operations (e.g. this crate) it isn't as useful. An example usage of this macro
/// is the following.
///
/// ```rust,ignore
/// fn do_thing(form: Form, user: User, db: &PgConnection) -> Result<(), Error> {
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
/// fn do_thing(form: Form, user: User, db: &PgConection) -> Result<(), Error> {
///     use aardwolf_types::traits::{Validate, DbAction};
///
///     let validated = ValidateForm(form).validate()?;
///     let updater = GetUpdater(user, validated).db_action(db)?;
///     UpdateRecord(updater).db_action(db)?;
///
///     Ok(())
/// }
/// ```
macro_rules! perform {
 ( $state:expr, $error_type:ty, [] ) => {{
     Ok(()) as Result<(), $error_type>
 }};
 ( $state:expr, $error_type:ty, [($store:pat = $operation:expr),] ) => {{
     use $crate::action::{Action, Wrapped};

     $operation
         .wrap()
         .action($state.clone())
         .map_err(|e| {
             let e: $error_type = e.into();
             e
         })
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
         .map_err(|e| {
             let e: $error_type = e.into();
             e
         })
         .and_then(move |item| {
             let $store = item;

             perform!($state, $error_type, [
                  $(($stores = $operations),)*
             ])
         })
 }};
}

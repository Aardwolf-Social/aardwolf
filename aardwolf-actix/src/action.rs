use std::marker::PhantomData;

use aardwolf_types::{
    error::AardwolfFail,
    forms::traits::{DbAction, Validate},
};
use futures::future::Future;

use crate::{
    db::{DbActionError, PerformDbAction},
    AppConfig,
};

pub trait Action<T, E>
where
    E: AardwolfFail,
{
    fn action(self, state: AppConfig) -> Box<dyn Future<Item = T, Error = E> + Send>;
}

pub struct ValidateWrapper<V, T, E>(V, PhantomData<T>, PhantomData<E>)
where
    V: Validate<T, E>,
    E: AardwolfFail;

impl<V, T, E> ValidateWrapper<V, T, E>
where
    V: Validate<T, E>,
    E: AardwolfFail,
{
    pub fn new(validate: V) -> Self {
        ValidateWrapper(validate, PhantomData, PhantomData)
    }
}

impl<V, T, E> From<V> for ValidateWrapper<V, T, E>
where
    V: Validate<T, E>,
    E: AardwolfFail,
{
    fn from(v: V) -> Self {
        ValidateWrapper::new(v)
    }
}

impl<V, T, E> Action<T, E> for ValidateWrapper<V, T, E>
where
    V: Validate<T, E>,
    T: Send + 'static,
    E: AardwolfFail,
{
    fn action(self, _: AppConfig) -> Box<dyn Future<Item = T, Error = E> + Send> {
        use futures::future::IntoFuture;

        Box::new(self.0.validate().into_future())
    }
}

pub struct DbActionWrapper<D, T, E>(D, PhantomData<T>, PhantomData<E>)
where
    D: DbAction<T, E>,
    E: AardwolfFail;

impl<D, T, E> DbActionWrapper<D, T, E>
where
    D: DbAction<T, E>,
    E: AardwolfFail,
{
    pub fn new(db_action: D) -> Self {
        DbActionWrapper(db_action, PhantomData, PhantomData)
    }
}

impl<D, T, E> From<D> for DbActionWrapper<D, T, E>
where
    D: DbAction<T, E>,
    E: AardwolfFail,
{
    fn from(d: D) -> Self {
        DbActionWrapper::new(d)
    }
}

impl<D, T, E> Action<T, DbActionError<E>> for DbActionWrapper<D, T, E>
where
    D: DbAction<T, E> + Send + 'static,
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
macro_rules! perform {
 ( $state:expr, $start:expr, $error_type:ty, [] ) => {{
     use futures::future::IntoFuture;

     (Ok($start) as Result<_, $error_type>).into_future()
 }};
 (
     $state:expr,
     $start:expr,
     $error_type:ty,
     [
         ($wrapper:ty => $first:expr),
         $(($wrappers:ty => $rest:expr),)*
     ]
 ) => {{
     use $crate::action::Action;

     let wrapper: $wrapper = $first.with($start).into();

     let fut = wrapper.action($state.clone());

     perform_inner!($state, $error_type, fut, [ $(($wrappers => $rest),)* ])
 }};
}

macro_rules! perform_inner {
    (
        $state:expr,
        $error_type:ty,
        $first:expr,
        []
    ) => {{
        use futures::future::IntoFuture;

        $first
            .into_future()
            .from_err::<$error_type>()
    }};
    (
        $state:expr,
        $error_type:ty,
        $first:expr,
        [
            ($wrapper:ty => $item:expr),
            $(($wrappers:ty => $items:expr),)*
        ]
    ) => {{
        use futures::future::IntoFuture;
        use $crate::action::Action;

        $first
            .into_future()
            .from_err::<$error_type>()
            .and_then(move |item| {
                let wrapper: $wrapper = $item.with(item).into();

                let fut = wrapper.action($state.clone());

                perform_inner!($state, $error_type, fut, [ $(($wrappers => $items),)* ])
            })
    }};
}

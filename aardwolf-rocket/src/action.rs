use std::marker::PhantomData;

use aardwolf_types::forms::traits::{DbAction, Validate};
use diesel::pg::PgConnection;
use failure::Fail;

pub trait Action<T, E>
where
    E: Fail,
{
    fn action(self, db: &PgConnection) -> Result<T, E>;
}

pub struct ValidateWrapper<V, T, E>(V, PhantomData<T>, PhantomData<E>)
where
    V: Validate<T, E>,
    E: Fail;

impl<V, T, E> ValidateWrapper<V, T, E>
where
    V: Validate<T, E>,
    E: Fail,
{
    pub fn new(validate: V) -> Self {
        ValidateWrapper(validate, PhantomData, PhantomData)
    }
}

impl<V, T, E> From<V> for ValidateWrapper<V, T, E>
where
    V: Validate<T, E>,
    E: Fail,
{
    fn from(v: V) -> Self {
        ValidateWrapper::new(v)
    }
}

impl<V, T, E> Action<T, E> for ValidateWrapper<V, T, E>
where
    V: Validate<T, E>,
    T: Send + 'static,
    E: Fail,
{
    fn action(self, _: &PgConnection) -> Result<T, E> {
        self.0.validate()
    }
}

pub struct DbActionWrapper<D, T, E>(D, PhantomData<T>, PhantomData<E>)
where
    D: DbAction<T, E>,
    E: Fail;

impl<D, T, E> DbActionWrapper<D, T, E>
where
    D: DbAction<T, E>,
    E: Fail,
{
    pub fn new(db_action: D) -> Self {
        DbActionWrapper(db_action, PhantomData, PhantomData)
    }
}

impl<D, T, E> From<D> for DbActionWrapper<D, T, E>
where
    D: DbAction<T, E>,
    E: Fail,
{
    fn from(d: D) -> Self {
        DbActionWrapper::new(d)
    }
}

impl<D, T, E> Action<T, E> for DbActionWrapper<D, T, E>
where
    D: DbAction<T, E> + Send + 'static,
    T: Send + 'static,
    E: Fail,
{
    fn action(self, db: &PgConnection) -> Result<T, E> {
        self.0.db_action(db)
    }
}

#[macro_export]
macro_rules! perform {
 ( $state:expr, $start:expr, $error_type:ty, [] ) => {{
     Ok($start) as Result<_, $error_type>
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

     let res = wrapper.action($state.clone());

     perform_inner!($state, $error_type, res, [ $(($wrappers => $rest),)* ])
 }};
}

macro_rules! perform_inner {
    (
        $state:expr,
        $error_type:ty,
        $first:expr,
        []
    ) => {{
        $first.map_err(|e| {
            let e: $error_type = e.into();
            e
        })
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
        use $crate::action::Action;

        $first
            .map_err(|e| {
                let e: $error_type = e.into();
                e
            })
            .and_then(move |item| {
                let wrapper: $wrapper = $item.with(item).into();

                let res = wrapper.action($state.clone());

                perform_inner!($state, $error_type, res, [ $(($wrappers => $items),)* ])
            })
    }};
}

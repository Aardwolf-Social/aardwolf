use crate::traits::{DbAction, Validate};
use failure::Fail;

pub trait Wrapped: Sized {
    type Wrapper: From<Self>;

    fn wrap(self) -> Self::Wrapper {
        self.into()
    }
}

pub struct DbActionWrapper<D, T, E>(pub D)
where
    D: DbAction<Item = T, Error = E>,
    E: Fail;

impl<D, T, E> DbActionWrapper<D, T, E>
where
    D: DbAction<Item = T, Error = E>,
    E: Fail,
{
    pub fn new(db_action: D) -> Self {
        DbActionWrapper(db_action)
    }
}

impl<D, T, E> From<D> for DbActionWrapper<D, T, E>
where
    D: DbAction<Item = T, Error = E>,
    E: Fail,
{
    fn from(d: D) -> Self {
        DbActionWrapper::new(d)
    }
}

pub struct ValidateWrapper<V, T, E>(pub V)
where
    V: Validate<Item = T, Error = E>,
    E: Fail;

impl<V, T, E> ValidateWrapper<V, T, E>
where
    V: Validate<Item = T, Error = E>,
    E: Fail,
{
    pub fn new(validate: V) -> Self {
        ValidateWrapper(validate)
    }
}

impl<V, T, E> From<V> for ValidateWrapper<V, T, E>
where
    V: Validate<Item = T, Error = E>,
    E: Fail,
{
    fn from(v: V) -> Self {
        ValidateWrapper::new(v)
    }
}

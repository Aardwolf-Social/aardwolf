use crate::{
    error::AardwolfFail,
    traits::{DbAction, Export, Validate},
};

use failure::Fail;
use serde_derive::Serialize;

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

pub struct ExportWrapper<E, T>(pub E)
where
    E: Export<Item = T>;

impl<E, T> ExportWrapper<E, T>
where
    E: Export<Item = T>,
{
    pub fn new(export: E) -> Self {
        ExportWrapper(export)
    }
}

impl<E, T> From<E> for ExportWrapper<E, T>
where
    E: Export<Item = T>,
{
    fn from(e: E) -> Self {
        ExportWrapper::new(e)
    }
}

pub struct ExportKind<T>(pub T);

impl<T> Export for ExportKind<T> {
    type Item = T;

    fn export(self) -> Self::Item {
        self.0
    }
}

impl<T> Wrapped for ExportKind<T> {
    type Wrapper = ExportWrapper<ExportKind<T>, T>;
}

#[derive(Clone, Debug, Fail, Serialize)]
#[fail(display = "Failed to export")]
pub struct ExportFail;

impl AardwolfFail for ExportFail {}

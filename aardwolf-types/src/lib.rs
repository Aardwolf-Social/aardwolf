#![cfg_attr(
    feature = "use-rocket",
    feature(custom_derive, proc_macro_hygiene, decl_macro)
)]

#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
#[cfg(feature = "use-rocket")]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

pub mod apps;
pub mod error;
pub mod forms;
pub mod scope;
pub mod traits;
pub mod wrapper;

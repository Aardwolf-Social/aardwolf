#![cfg_attr(feature = "use-rocket", feature(plugin))]
#![cfg_attr(feature = "use-rocket", feature(custom_derive))]
#![cfg_attr(feature = "use-rocket", plugin(rocket_codegen))]

#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

pub mod apps;
pub mod forms;
pub mod scope;

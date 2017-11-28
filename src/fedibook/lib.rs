#![recursion_limit="128"]
#![feature(try_from)]
#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate failure;
extern crate rocket_contrib;
extern crate serde;
#[macro_use] extern crate failure_derive;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate derive_builder;
#[macro_use] extern crate collection_macros;
#[macro_use] extern crate diesel;

pub mod models;
pub mod controllers;
pub mod routes;
pub mod schema;

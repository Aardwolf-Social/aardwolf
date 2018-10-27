#![cfg_attr(feature = "use-rocket", feature(plugin))]
#![cfg_attr(feature = "use-rocket", feature(custom_derive))]
#![cfg_attr(feature = "use-rocket", plugin(rocket_codegen))]

#[macro_use] extern crate derive_builder;
#[macro_use] extern crate failure;
#[macro_use] extern crate serde_derive;

pub mod forms;
pub mod apps;
pub mod scope;
mod user;

pub use self::user::{SignedInUser, SignedInUserWithEmail};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

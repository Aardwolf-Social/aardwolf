// Using direct import of i18n crate example
// https://crates.io/crates/rust-i18n

// You must import in each files when you wants use `t!` macro.
use rust_i18n::t;

rust_i18n::i18n!("locales");

fn main() {
    println!("{}", t!("hello"));

    // Use `available_locales!` method to get all available locales.
    println!("{:?}", rust_i18n::available_locales!());
}
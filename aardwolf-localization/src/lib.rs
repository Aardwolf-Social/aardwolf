// Load macro and init translations.
// https://crates.io/crates/rust-i18n

// Load I18n macro, for allow you use `t!` macro in anywhere.
#[macro_use]
extern crate rust_i18n;

// Config fallback missing translations to "en" locale.
// Use `fallback` option to set fallback locale.
i18n!("locales", fallback = "en-us");
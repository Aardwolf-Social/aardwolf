#![allow(clippy::inline_fn_without_body)]
#![allow(clippy::into_iter_on_ref)]

use gettext_macros::{compile_i18n, include_i18n, init_i18n};
use rocket_i18n::Translations;

init_i18n!("aardwolf", en, pl);

include!(concat!(env!("OUT_DIR"), "/templates.rs"));

// Directories
pub mod asides;
pub mod containers;
pub mod elements;
pub mod error;
pub mod home;
pub mod posts;

// Root-level files
mod first_login;
mod sign_in;
mod sign_up;

pub use self::{first_login::*, sign_in::*, sign_up::*};

use self::{
    asides::Shortcuts,
    elements::{
        Alert, AlertKind, Input, InputCheckbox, InputEmail, InputPassword, InputSelect, InputText,
        InputTextarea,
    },
};

pub trait Renderable {
    fn render(&self, _: &mut dyn std::io::Write) -> std::io::Result<()>;
}

/// Returns an empty Translations object to disable translations due to issues with the gettext library.
pub fn managed_state() -> Translations {
    // gettext is not behaving correctly, so translations are disabled until a replacement is found
    Vec::new()
}

#![feature(proc_macro_hygiene)]
#![allow(clippy::inline_fn_without_body)]
#![allow(clippy::into_iter_on_ref)]

use rocket_i18n::Translations;
use gettext_macros::{compile_i18n, include_i18n, init_i18n};

init_i18n!("aardwolf", en, pl);

include!("../compiled_templates/templates.rs");

// Directories
mod asides;
mod containers;
mod elements;
mod error;
mod home;
mod posts;

// Root-level files
mod first_login;
mod sign_in;
mod sign_up;


pub use self::{
    first_login::FirstLogin,
    sign_in::SignIn,
    sign_up::SignUp,
};

use self::{
    shortcuts::Shortcuts,
    elements::{
        Alert, AlertKind, InputCheckbox, InputEmail, Input, InputPassword, InputSelect, InputText,
        InputTextarea,
    },
};


pub trait Renderable {
    fn render(&self, _: &mut dyn std::io::Write) -> std::io::Result<()>;
}

pub fn managed_state() -> Translations {
    include_i18n!()
}

compile_i18n!();

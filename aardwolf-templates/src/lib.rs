#![feature(proc_macro_hygiene)]
#![allow(clippy::inline_fn_without_body)]
#![allow(clippy::into_iter_on_ref)]

use actix_i18n::Translations;
use gettext_macros::{compile_i18n, include_i18n, init_i18n};

init_i18n!("aardwolf", en, pl);

include!("../compiled_templates/templates.rs");

mod first_login;
mod home;
mod shortcuts;
mod sign_in;
mod sign_up;
mod ui;

pub use self::{
    first_login::FirstLogin,
    home::{Home, NewPost},
    sign_in::SignIn,
    sign_up::SignUp,
};

use self::{
    shortcuts::Shortcuts,
    ui::{
        Alert, AlertKind, CheckboxInput, EmailInput, Input, PasswordInput, SelectInput, TextInput,
        TextareaInput,
    },
};

pub trait Renderable {
    fn render(&self, _: &mut std::io::Write) -> std::io::Result<()>;
}

pub fn managed_state() -> Translations {
    include_i18n!()
}

compile_i18n!();

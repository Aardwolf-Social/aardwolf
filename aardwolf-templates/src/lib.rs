extern crate aardwolf_types;
extern crate aardwolf_models;
extern crate gettext;
extern crate rocket_i18n;
include!("../compiled_templates/templates.rs");

mod first_login;
mod home;
mod shortcuts;
mod sign_in;
mod sign_up;
mod ui;

pub use self::{home::Home, sign_in::SignIn, sign_up::SignUp, first_login::FirstLogin};

use self::{
    shortcuts::Shortcuts,
    ui::{Alert, AlertKind, EmailInput, Input, PasswordInput, TextInput, SelectInput, SelectOption, CheckboxInput},
};

pub trait Renderable {
    fn render(&self, &mut std::io::Write) -> std::io::Result<()>;
}

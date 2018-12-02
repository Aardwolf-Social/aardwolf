extern crate aardwolf_types;
extern crate gettext;
extern crate rocket_i18n;
include!("../compiled_templates/templates.rs");

mod home;
mod kitbashing;
mod shortcuts;
mod sign_in;
mod sign_up;

pub use self::{home::Home, sign_in::SignIn, sign_up::SignUp};

use self::{
    shortcuts::Shortcuts,
    ui::{Alert, AlertKind, EmailInput, Input, PasswordInput, TextInput},
};

pub trait Renderable {
    fn render(&self, &mut std::io::Write) -> std::io::Result<()>;
}

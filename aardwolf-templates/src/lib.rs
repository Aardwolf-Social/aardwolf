extern crate aardwolf_types;
extern crate gettext;
extern crate rocket_i18n;
include!("../compiled_templates/templates.rs");

mod home;
mod shortcuts;
mod sign_in;
mod sign_up;
mod widgets;

pub use self::{home::Home, sign_in::SignIn, sign_up::SignUp};

use self::{
    shortcuts::Shortcuts,
    widgets::{Alert, AlertKind, EmailInput, Icon, Input, PasswordInput, TextInput},
};

pub trait Renderable {
    fn render(&self, &mut std::io::Write) -> std::io::Result<()>;
}

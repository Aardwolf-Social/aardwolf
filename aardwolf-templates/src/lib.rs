#![allow(clippy::inline_fn_without_body)]
#![allow(clippy::into_iter_on_ref)]
include!("../compiled_templates/templates.rs");

mod first_login;
mod home;
mod shortcuts;
mod sign_in;
mod sign_up;
mod widgets;

pub use self::{first_login::FirstLogin, home::Home, sign_in::SignIn, sign_up::SignUp};

use self::{
    shortcuts::Shortcuts,
<<<<<<< HEAD
    widgets::{Alert, AlertKind, EmailInput, Icon, Input, PasswordInput, TextInput},
=======
    ui::{
        Alert, AlertKind, CheckboxInput, EmailInput, Input, PasswordInput, SelectInput,
        SelectOption, TextInput,
    },
>>>>>>> master
};

pub trait Renderable {
    fn render(&self, _: &mut std::io::Write) -> std::io::Result<()>;
}

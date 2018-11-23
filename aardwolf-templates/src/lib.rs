extern crate aardwolf_types;
extern crate gettext;
extern crate rocket_i18n;
include!("../compiled_templates/templates.rs");

use aardwolf_types::forms::auth::{ValidateSignInFormFail, ValidateSignUpFormFail};
use gettext::Catalog;

pub trait Renderable {
    fn render(self, &mut std::io::Write) -> std::io::Result<()>;
}

impl<'a> Renderable for SignIn<'a> {
    fn render(self, write: &mut std::io::Write) -> std::io::Result<()> {
        templates::sign_in(write, self)
    }
}

impl<'a> Renderable for SignUp<'a> {
    fn render(self, write: &mut std::io::Write) -> std::io::Result<()> {
        templates::sign_up(write, self)
    }
}

impl<'a> Renderable for Home<'a> {
    fn render(self, write: &mut std::io::Write) -> std::io::Result<()> {
        templates::home(write, self)
    }
}

pub struct Home<'a> {
    catalog: &'a Catalog,
    shortcuts: Shortcuts<'a>,
}

impl<'a> Home<'a> {
    pub fn new(catalog: &'a Catalog, profile_link: &'a str, username: &'a str) -> Self {
        Home {
            catalog,
            shortcuts: Shortcuts {
                catalog,
                profile_link,
                username,
            },
        }
    }
}

pub struct Shortcuts<'a> {
    catalog: &'a Catalog,
    profile_link: &'a str,
    username: &'a str,
}

pub struct SignIn<'a> {
    catalog: &'a Catalog,
    csrf: &'a str,
    alert: Option<Alert<'a>>,
    email: EmailInput<'a>,
    password: PasswordInput<'a>,
}

impl<'a> SignIn<'a> {
    pub fn new(
        catalog: &'a Catalog,
        csrf: &'a str,
        email: &'a str,
        validation_error: Option<&'a ValidateSignInFormFail>,
        server_error: bool,
    ) -> Self {
        SignIn {
            catalog,
            csrf,
            alert: if server_error {
                Some(Alert {
                    catalog,
                    kind: AlertKind::Error,
                    message: "There was an error logging in",
                })
            } else {
                None
            },
            email: EmailInput {
                catalog,
                name: "email",
                label: "E-Mail Address",
                placeholder: Some("E-Mail Address"),
                value: email,
                error: validation_error.and_then(|e| e.email.as_ref()),
            },
            password: PasswordInput {
                catalog,
                name: "password",
                label: "Password",
                placeholder: Some("Password"),
                error: validation_error.and_then(|e| e.password.as_ref()),
            },
        }
    }
}

pub struct SignUp<'a> {
    catalog: &'a Catalog,
    csrf: &'a str,
    alert: Option<Alert<'a>>,
    email: EmailInput<'a>,
    password: PasswordInput<'a>,
    password_confirmation: PasswordInput<'a>,
}

impl<'a> SignUp<'a> {
    pub fn new(
        catalog: &'a Catalog,
        csrf: &'a str,
        email: &'a str,
        validation_error: Option<&'a ValidateSignUpFormFail>,
        server_error: bool,
    ) -> Self {
        SignUp {
            catalog,
            csrf,
            alert: if server_error {
                Some(Alert {
                    catalog,
                    kind: AlertKind::Error,
                    message: "There was an error creating your account",
                })
            } else {
                None
            },
            email: EmailInput {
                catalog,
                name: "email",
                label: "E-Mail Address",
                placeholder: Some("E-Mail Address"),
                value: email,
                error: validation_error.and_then(|e| e.email.as_ref()),
            },
            password: PasswordInput {
                catalog,
                name: "password",
                label: "Password",
                placeholder: Some("Password"),
                error: validation_error.and_then(|e| e.password.as_ref()),
            },
            password_confirmation: PasswordInput {
                catalog,
                name: "password_confirmation",
                label: "Password Confirmation",
                placeholder: Some("Password Confirmation"),
                error: validation_error.and_then(|e| e.password_confirmation.as_ref()),
            },
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AlertKind {
    Error,
    Warning,
    Info,
}

impl std::fmt::Display for AlertKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match *self {
            AlertKind::Error => "error",
            AlertKind::Warning => "warning",
            AlertKind::Info => "info",
        };

        write!(f, "{}", s)
    }
}

pub struct Alert<'a> {
    catalog: &'a Catalog,
    kind: AlertKind,
    message: &'a str,
}

pub struct Input<'a> {
    catalog: &'a Catalog,
    kind: &'a str,
    name: &'a str,
    label: Option<&'a str>,
    icon: Option<&'a str>,
    placeholder: Option<&'a str>,
    value: &'a str,
    error: Option<&'a String>,
}

impl<'a> From<PasswordInput<'a>> for Input<'a> {
    fn from(p: PasswordInput<'a>) -> Self {
        let PasswordInput {
            catalog,
            name,
            label,
            placeholder,
            error,
        } = p;

        Input {
            catalog,
            kind: "password",
            name,
            label: Some(label),
            placeholder,
            icon: Some("lock"),
            value: "",
            error,
        }
    }
}

impl<'a> From<EmailInput<'a>> for Input<'a> {
    fn from(e: EmailInput<'a>) -> Self {
        let EmailInput {
            catalog,
            name,
            label,
            placeholder,
            value,
            error,
        } = e;

        Input {
            catalog,
            kind: "email",
            name,
            label: Some(label),
            placeholder,
            icon: Some("envelope"),
            value,
            error,
        }
    }
}

impl<'a> From<TextInput<'a>> for Input<'a> {
    fn from(e: TextInput<'a>) -> Self {
        let TextInput {
            catalog,
            name,
            label,
            placeholder,
            icon,
            value,
            error,
        } = e;

        Input {
            catalog,
            kind: "text",
            name,
            label: Some(label),
            placeholder,
            icon,
            value,
            error,
        }
    }
}

pub struct PasswordInput<'a> {
    catalog: &'a Catalog,
    name: &'a str,
    label: &'a str,
    placeholder: Option<&'a str>,
    error: Option<&'a String>,
}

pub struct EmailInput<'a> {
    catalog: &'a Catalog,
    name: &'a str,
    label: &'a str,
    placeholder: Option<&'a str>,
    value: &'a str,
    error: Option<&'a String>,
}

pub struct TextInput<'a> {
    catalog: &'a Catalog,
    name: &'a str,
    label: &'a str,
    placeholder: Option<&'a str>,
    icon: Option<&'a str>,
    value: &'a str,
    error: Option<&'a String>,
}

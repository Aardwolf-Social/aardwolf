extern crate aardwolf_types;
extern crate gettext;
extern crate rocket_i18n;
include!("../compiled_templates/templates.rs");

use aardwolf_types::forms::auth::{ValidateSignInFormFail, ValidateSignUpFormFail};
use gettext::Catalog;

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
                    message: "There was an error creating your account",
                })
            } else {
                None
            },
            email: EmailInput {
                catalog,
                name: "email",
                placeholder: Some("E-Mail Address"),
                value: email,
                error: validation_error.and_then(|e| e.email.as_ref()),
            },
            password: PasswordInput {
                catalog,
                name: "password",
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
                placeholder: Some("E-Mail Address"),
                value: email,
                error: validation_error.and_then(|e| e.email.as_ref()),
            },
            password: PasswordInput {
                catalog,
                name: "password",
                placeholder: Some("Password"),
                error: validation_error.and_then(|e| e.password.as_ref()),
            },
            password_confirmation: PasswordInput {
                catalog,
                name: "password_confirmation",
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

pub struct Alert<'a> {
    catalog: &'a Catalog,
    kind: AlertKind,
    message: &'a str,
}

pub struct Input<'a> {
    catalog: &'a Catalog,
    kind: &'a str,
    name: &'a str,
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
            placeholder,
            error,
        } = p;

        Input {
            catalog,
            kind: "password",
            name,
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
            placeholder,
            value,
            error,
        } = e;

        Input {
            catalog,
            kind: "email",
            name,
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
            placeholder,
            icon,
            value,
            error,
        } = e;

        Input {
            catalog,
            kind: "text",
            name,
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
    placeholder: Option<&'a str>,
    error: Option<&'a String>,
}

pub struct EmailInput<'a> {
    catalog: &'a Catalog,
    name: &'a str,
    placeholder: Option<&'a str>,
    value: &'a str,
    error: Option<&'a String>,
}

pub struct TextInput<'a> {
    catalog: &'a Catalog,
    name: &'a str,
    placeholder: Option<&'a str>,
    icon: Option<&'a str>,
    value: &'a str,
    error: Option<&'a String>,
}

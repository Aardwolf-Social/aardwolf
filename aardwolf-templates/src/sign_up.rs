use aardwolf_types::forms::auth::ValidateSignUpFormFail;
use gettext::Catalog;

use crate::{Alert, AlertKind, EmailInput, PasswordInput, Renderable};

pub struct SignUp<'a> {
    pub(crate) catalog: &'a Catalog,
    pub(crate) csrf: &'a str,
    pub(crate) alert: Option<Alert<'a>>,
    pub(crate) email: EmailInput<'a>,
    pub(crate) password: PasswordInput<'a>,
    pub(crate) password_confirmation: PasswordInput<'a>,
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

impl<'a> Renderable for SignUp<'a> {
    fn render(&self, write: &mut std::io::Write) -> std::io::Result<()> {
        crate::templates::sign_up(write, self)
    }
}

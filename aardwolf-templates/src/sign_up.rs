use aardwolf_types::forms::auth::ValidateSignUpFormFail;
use gettext::Catalog;
use gettext_macros::i18n;

use crate::{Alert, AlertKind, EmailInput, PasswordInput, Renderable};

pub struct SignUp<'a> {
    pub(crate) catalog: &'a Catalog,
    pub(crate) csrf: &'a str,
    pub(crate) alert: Option<Alert>,
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
                    kind: AlertKind::Error,
                    message: i18n!(catalog, "There was an error creating your account"),
                })
            } else {
                None
            },
            email: EmailInput {
                name: "email",
                label: i18n!(catalog, "E-Mail Address"),
                placeholder: Some(i18n!(catalog, "E-Mail Address")),
                value: email,
                error: validation_error.and_then(|e| e.email.clone()),
            },
            password: PasswordInput {
                name: "password",
                label: i18n!(catalog, "Password"),
                placeholder: Some(i18n!(catalog, "Password")),
                error: validation_error.and_then(|e| e.password.clone()),
            },
            password_confirmation: PasswordInput {
                name: "password_confirmation",
                label: i18n!(catalog, "Password Confirmation"),
                placeholder: Some(i18n!(catalog, "Password Confirmation")),
                error: validation_error.and_then(|e| e.password_confirmation.clone()),
            },
        }
    }
}

impl<'a> Renderable for SignUp<'a> {
    fn render(&self, write: &mut std::io::Write) -> std::io::Result<()> {
        crate::templates::sign_up(write, self)
    }
}

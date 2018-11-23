use aardwolf_types::forms::auth::ValidateSignInFormFail;
use gettext::Catalog;

use crate::{Alert, AlertKind, EmailInput, PasswordInput, Renderable};

pub struct SignIn<'a> {
    pub(crate) catalog: &'a Catalog,
    pub(crate) csrf: &'a str,
    pub(crate) alert: Option<Alert<'a>>,
    pub(crate) email: EmailInput<'a>,
    pub(crate) password: PasswordInput<'a>,
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

impl<'a> Renderable for SignIn<'a> {
    fn render(self, write: &mut std::io::Write) -> std::io::Result<()> {
        crate::templates::sign_in(write, self)
    }
}

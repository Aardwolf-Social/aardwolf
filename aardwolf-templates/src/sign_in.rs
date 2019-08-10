use aardwolf_types::forms::auth::{
    SignInEmailValidationFail, SignInFormState, SignInPasswordValidationFail,
    ValidateSignInFormFail,
};
use gettext::Catalog;
use gettext_macros::i18n;

use crate::{Alert, AlertKind, EmailInput, PasswordInput, Renderable};

pub struct SignIn<'a> {
    pub(crate) catalog: &'a Catalog,
    pub(crate) csrf: &'a str,
    pub(crate) alert: Option<Alert>,
    pub(crate) email: EmailInput<'a>,
    pub(crate) password: PasswordInput<'a>,
}

impl<'a> SignIn<'a> {
    pub fn new(
        catalog: &'a Catalog,
        csrf: &'a str,
        state: &'a SignInFormState,
        validation_error: Option<&'a ValidateSignInFormFail>,
        server_error: bool,
    ) -> Self {
        SignIn {
            catalog,
            csrf,
            alert: if server_error {
                Some(Alert {
                    kind: AlertKind::Error,
                    message: i18n!(catalog, "There was an error logging in"),
                })
            } else {
                None
            },
            email: EmailInput {
                name: "email",
                label: i18n!(catalog, "E-Mail Address"),
                placeholder: Some(i18n!(catalog, "E-Mail Address")),
                value: &state.email,
                error: validation_error.and_then(|e| {
                    e.email.as_ref().map(|e| match *e {
                        SignInEmailValidationFail::Empty => i18n!(catalog, "Email cannot be empty"),
                    })
                }),
            },
            password: PasswordInput {
                name: "password",
                label: i18n!(catalog, "Password"),
                placeholder: Some(i18n!(catalog, "Password")),
                error: validation_error.and_then(|e| {
                    e.password.as_ref().map(|e| match *e {
                        SignInPasswordValidationFail::Empty => {
                            i18n!(catalog, "Password cannot be empty")
                        }
                    })
                }),
            },
        }
    }
}

impl<'a> Renderable for SignIn<'a> {
    fn render(&self, write: &mut dyn std::io::Write) -> std::io::Result<()> {
        crate::templates::sign_in(write, self)
    }
}

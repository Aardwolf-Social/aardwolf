use aardwolf_types::forms::auth::{
    SignUpEmailValidationFail, SignUpFormState, SignUpPasswordConfirmationValidationFail,
    SignUpPasswordValidationFail, ValidateSignUpFormFail,
};
use gettext::Catalog;
use gettext_macros::i18n;

use crate::{Alert, AlertKind, InputEmail, InputPassword, Renderable};

pub struct SignUp<'a> {
    pub(crate) catalog: &'a Catalog,
    pub(crate) csrf: &'a str,
    pub(crate) alert: Option<Alert>,
    pub(crate) email: InputEmail<'a>,
    pub(crate) password: InputPassword<'a>,
    pub(crate) password_confirmation: InputPassword<'a>,
}

impl<'a> SignUp<'a> {
    pub fn new(
        catalog: &'a Catalog,
        csrf: &'a str,
        state: &'a SignUpFormState,
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
            email: InputEmail {
                name: "email",
                label: i18n!(catalog, "E-Mail Address"),
                placeholder: Some(i18n!(catalog, "E-Mail Address")),
                value: &state.email,
                error: validation_error.and_then(|e| {
                    e.email.as_ref().map(|e| match *e {
                        SignUpEmailValidationFail::Empty => i18n!(catalog, "Email cannot be empty"),
                        SignUpEmailValidationFail::Malformed => {
                            i18n!(catalog, "Invalid email address")
                        }
                    })
                }),
            },
            password: InputPassword {
                name: "password",
                label: i18n!(catalog, "Password"),
                placeholder: Some(i18n!(catalog, "Password")),
                error: validation_error.and_then(|e| {
                    e.password.as_ref().map(|e| match *e {
                        SignUpPasswordValidationFail::Empty => {
                            i18n!(catalog, "Password cannot be empty")
                        }
                        SignUpPasswordValidationFail::TooShort => {
                            i18n!(catalog, "Password is too short")
                        }
                    })
                }),
            },
            password_confirmation: InputPassword {
                name: "password_confirmation",
                label: i18n!(catalog, "Password Confirmation"),
                placeholder: Some(i18n!(catalog, "Password Confirmation")),
                error: validation_error.and_then(|e| {
                    e.password_confirmation.as_ref().map(|e| match *e {
                        SignUpPasswordConfirmationValidationFail::Empty => {
                            i18n!(catalog, "Password confirmation cannot be empty")
                        }
                        SignUpPasswordConfirmationValidationFail::Match => {
                            i18n!(catalog, "Password confirmation must match password")
                        }
                    })
                }),
            },
        }
    }
}

impl<'a> Renderable for SignUp<'a> {
    fn render(&self, write: &mut dyn std::io::Write) -> std::io::Result<()> {
        crate::templates::sign_up(write, self)
    }
}

use std::io::{self, Write};
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use ::templates::{Html,ToHtml};
use aardwolf_types::forms::auth::{SignUpFormState, ValidateSignUpFormFail};
use gettext::Catalog;
use rocket_i18n::i18n;
use crate::templates::{base, ui::{alert, email_input, password_input}};

pub fn sign_up(out: &mut Write, catalog: &Catalog, state: SignUpFormState, csrf: &str, validation_error: Option<ValidateSignUpFormFail>, server_error: Option<String>)
-> io::Result<()> {
base(out, catalog, i18n!(catalog, "Aardwolf | Sign Up"), |out| {
write!(out, "\n<header>\n    <h2 class=\"title\">")?;
i18n!(catalog, "Aardwolf Instance").to_html(out)?;
write!(out, "</h2>\n    <div style=\"text-align: right\">\n        <a href=\"sign_in\">")?;
i18n!(catalog, "Have an Account? - Login").to_html(out)?;
write!(out, "</a>\n    </div>\n</header>\n<!-- End Page Header -->\n<section class=\"section\">\n    <div class=\"container\">\n\n        <div class=\"columns is-mobile\">\n            <div class=\"column\">\n                <h1 class=\"title\">\n                    ")?;
i18n!(catalog, "About Aardwolf").to_html(out)?;
write!(out, "\n                </h1>\n                <p class=\"subtitle\">\n                ")?;
i18n!(catalog, "This is who we are!").to_html(out)?;
write!(out, "\n                </p>\n                ")?;
i18n!(catalog, "really-long-platform-description").to_html(out)?;
write!(out, "\n            </div>\n            <div class=\"column\">\n                <h1 class=\"title\">\n                    ")?;
i18n!(catalog, "Create an Account").to_html(out)?;
write!(out, "\n                </h1>\n                <p class=\"subtitle\">\n                ")?;
i18n!(catalog, "Feel free to sign up!").to_html(out)?;
write!(out, "\n                </p>\n                <form method=\"POST\" action=\"/auth/sign_up\">\n                    <input type=\"hidden\" name=\"csrf_token\" value=\"")?;
csrf.to_html(out)?;
write!(out, "\">\n                    ")?;
if let Some(error) = validation_error {
write!(out, "\n                        ")?;
email_input(out, catalog, "email", "E-Mail Address", &state.email, error.email)?;
write!(out, "\n                        ")?;
password_input(out, catalog, "password", "Password", error.password)?;
write!(out, "\n                        ")?;
password_input(out, catalog, "password_confirmation", "Confirm Password", error.password_confirmation)?;
write!(out, "\n                    ")?;
} else {
write!(out, "\n                        ")?;
if let Some(error) = server_error {
write!(out, "\n                            ")?;
alert(out, catalog, "error", &error)?;
write!(out, "\n                            ")?;
email_input(out, catalog, "email", "E-Mail Address", &state.email, None)?;
write!(out, "\n                            ")?;
password_input(out, catalog, "password", "Password", None)?;
write!(out, "\n                            ")?;
password_input(out, catalog, "password_confirmation", "Confirm Password", None)?;
write!(out, "\n                        ")?;
} else {
write!(out, "\n                            ")?;
email_input(out, catalog, "email", "E-mail Address", &state.email, None)?;
write!(out, "\n                            ")?;
password_input(out, catalog, "password", "Password", None)?;
write!(out, "\n                            ")?;
password_input(out, catalog, "password_confirmation", "Confirm Password", None)?;
write!(out, "\n                        ")?;
}
write!(out, "\n                    ")?;
}
write!(out, "\n                    <button>")?;
i18n!(catalog, "Sign Up").to_html(out)?;
write!(out, "</button>\n                </form>\n            </div>\n        </div>\n    </div>\n</section>\n")?;

Ok(())
}
)?;
write!(out, "\n")?;
Ok(())
}

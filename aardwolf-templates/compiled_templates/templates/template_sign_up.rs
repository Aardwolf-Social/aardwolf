use std::io::{self, Write};
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use ::templates::{Html,ToHtml};
use rocket_i18n::i18n;
use crate::{SignUp, templates::{base, ui::{alert, email_input, password_input}}};

pub fn sign_up(out: &mut Write, sign_up: SignUp)
-> io::Result<()> {
base(out, sign_up.catalog, "Aardwolf | Sign Up", |out| {
write!(out, "\n<header>\n    <h2 class=\"title\">")?;
i18n!(sign_up.catalog, "Aardwolf Instance").to_html(out)?;
write!(out, "</h2>\n    <div style=\"text-align: right\">\n        <a href=\"sign_in\">")?;
i18n!(sign_up.catalog, "Have an Account? - Login").to_html(out)?;
write!(out, "</a>\n    </div>\n</header>\n<!-- End Page Header -->\n<section class=\"section\">\n    <div class=\"container\">\n\n        <div class=\"columns is-mobile\">\n            <div class=\"column\">\n                <h1 class=\"title\">\n                    ")?;
i18n!(sign_up.catalog, "About Aardwolf").to_html(out)?;
write!(out, "\n                </h1>\n                <p class=\"subtitle\">\n                ")?;
i18n!(sign_up.catalog, "This is who we are!").to_html(out)?;
write!(out, "\n                </p>\n                ")?;
i18n!(sign_up.catalog, "really-long-platform-description").to_html(out)?;
write!(out, "\n            </div>\n            <div class=\"column\">\n                <h1 class=\"title\">\n                    ")?;
i18n!(sign_up.catalog, "Create an Account").to_html(out)?;
write!(out, "\n                </h1>\n                <p class=\"subtitle\">\n                ")?;
i18n!(sign_up.catalog, "Feel free to sign up!").to_html(out)?;
write!(out, "\n                </p>\n                <form method=\"POST\" action=\"/auth/sign_up\">\n                    <input type=\"hidden\" name=\"csrf_token\" value=\"")?;
sign_up.csrf.to_html(out)?;
write!(out, "\">\n                    ")?;
if let Some(a) = sign_up.alert {
write!(out, "\n                        ")?;
alert(out, a)?;
write!(out, "\n                    ")?;
}
write!(out, "\n\n                    ")?;
email_input(out, sign_up.email)?;
write!(out, "\n                    ")?;
password_input(out, sign_up.password)?;
write!(out, "\n                    ")?;
password_input(out, sign_up.password_confirmation)?;
write!(out, "\n                    <button>")?;
i18n!(sign_up.catalog, "Sign Up").to_html(out)?;
write!(out, "</button>\n                </form>\n            </div>\n        </div>\n    </div>\n</section>\n")?;

Ok(())
}
)?;
write!(out, "\n")?;
Ok(())
}

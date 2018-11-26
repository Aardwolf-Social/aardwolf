use std::io::{self, Write};
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use ::templates::{Html,ToHtml};
use rocket_i18n::i18n;
use crate::{SignIn, templates::{base, ui::{alert, email_input, password_input}}};

pub fn sign_in(out: &mut Write, sign_in: SignIn)
-> io::Result<()> {
base(out, sign_in.catalog, "Aardwolf | Sign In", |out| {
write!(out, "\n<header>\n    <h2 class=\"title\">")?;
i18n!(sign_in.catalog, "Aardwolf Instance").to_html(out)?;
write!(out, "</h2>\n    <div style=\"text-align: right\">\n        <a href=\"sign_up\">")?;
i18n!(sign_in.catalog, "Need an Account? - Create one!").to_html(out)?;
write!(out, "</a>\n    </div>\n</header>\n<!-- End Page Header -->\n<section class=\"section\">\n    <div class=\"container\">\n\n        <div class=\"columns is-mobile\">\n            <div class=\"column\">\n                <h1 class=\"title\">\n                    ")?;
i18n!(sign_in.catalog, "About Aardwolf").to_html(out)?;
write!(out, "\n                </h1>\n                <p class=\"subtitle\">\n                ")?;
i18n!(sign_in.catalog, "This is who we are!").to_html(out)?;
write!(out, "\n                </p>\n                ")?;
i18n!(sign_in.catalog, "really-long-platform-description").to_html(out)?;
write!(out, "\n            </div>\n            <div class=\"column\">\n                <h1 class=\"title\">\n                    ")?;
i18n!(sign_in.catalog, "Login").to_html(out)?;
write!(out, "\n                </h1>\n                <p class=\"subtitle\">\n                ")?;
i18n!(sign_in.catalog, "Welcome back!").to_html(out)?;
write!(out, "\n                </p>\n                ")?;
if let Some(a) = sign_in.alert {
write!(out, "\n                    ")?;
alert(out, a)?;
write!(out, "\n                ")?;
}
write!(out, "\n                <form method=\"POST\" action=\"/auth/sign_in\">\n                    <input type=\"hidden\" name=\"csrf_token\" value=\"")?;
sign_in.csrf.to_html(out)?;
write!(out, "\">\n                    ")?;
email_input(out, sign_in.email)?;
write!(out, "\n                    ")?;
password_input(out, sign_in.password)?;
write!(out, "\n                    <button>")?;
i18n!(sign_in.catalog, "Log In").to_html(out)?;
write!(out, "</button>\n                </form>\n            </div>\n        </div>\n    </div>\n</section>\n")?;

Ok(())
}
)?;
write!(out, "\n")?;
Ok(())
}

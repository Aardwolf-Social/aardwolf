use std::io::{self, Write};
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use ::templates::{Html,ToHtml};
use aardwolf_types::forms::auth::SignUpErrorMessage;
use gettext::Catalog;
use rocket_i18n::i18n;
use crate::templates::base;

pub fn sign_up(out: &mut Write, catalog: Catalog, csrf: &str, domain: &str, error: Option<SignUpErrorMessage>)
-> io::Result<()> {
base(out, catalog.clone(), i18n!(catalog, "Aardwolf | Sign In"), |out| {
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
write!(out, "\n                </p>\n                ")?;
if let Some(error) = error {
write!(out, "\n                    <span style=\"color: red;\">")?;
i18n!(catalog, &error.msg).to_html(out)?;
write!(out, "</span>\n                ")?;
}
write!(out, "\n                <form method=\"POST\" action=\"/auth/sign_up\">\n                    <input type=\"hidden\" name=\"csrf_token\" value=\"")?;
csrf.to_html(out)?;
write!(out, "\">\n                    <span class=\"icon icon-user\"></span>\n                    <input type=\"text\" name=\"username\" id=\"username\" placeholder=\"")?;
["username@", domain].join("").to_html(out)?;
write!(out, "\">\n                    <span class=\"icon icon-envelope\"></span>\n                    <input type=\"email\" name=\"email\" id=\"email\" placeholder=\"E-mail address\">\n                    <span class=\"icon icon-lock\"></span>\n                    <input type=\"password\" name=\"password\" id=\"password\" placeholder=\"Password\">\n                    <span class=\"icon icon-lock\"></span>\n                    <input type=\"password\" name=\"password_confirmation\", id=\"password_confirmation\" placeholder=\"Confirm password\">\n                    <button>")?;
i18n!(catalog, "Sign Up").to_html(out)?;
write!(out, "</button>\n                </form>\n            </div>\n        </div>\n    </div>\n</section>\n")?;

Ok(())
}
)?;
write!(out, "\n")?;
Ok(())
}

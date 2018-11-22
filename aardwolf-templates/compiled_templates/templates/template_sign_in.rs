use std::io::{self, Write};
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use ::templates::{Html,ToHtml};
use gettext::Catalog;
use rocket_i18n::i18n;
use crate::templates::base;

pub fn sign_in(out: &mut Write, catalog: Catalog, csrf: &str)
-> io::Result<()> {
base(out, catalog.clone(), i18n!(catalog, "Aardwolf | Sign In"), |out| {
write!(out, "\n<header>\n    <h2 class=\"title\">")?;
i18n!(catalog, "Aardwolf Instance").to_html(out)?;
write!(out, "</h2>\n    <div style=\"text-align: right\">\n        <a href=\"sign_up\">")?;
i18n!(catalog, "Need an Account? - Create one!").to_html(out)?;
write!(out, "</a>\n    </div>\n</header>\n<!-- End Page Header -->\n<section class=\"section\">\n    <div class=\"container\">\n\n        <div class=\"columns is-mobile\">\n            <div class=\"column\">\n                <h1 class=\"title\">\n                    ")?;
i18n!(catalog, "About Aardwolf").to_html(out)?;
write!(out, "\n                </h1>\n                <p class=\"subtitle\">\n                ")?;
i18n!(catalog, "This is who we are!").to_html(out)?;
write!(out, "\n                </p>\n                ")?;
i18n!(catalog, "really-long-platform-description").to_html(out)?;
write!(out, "\n            </div>\n            <div class=\"column\">\n                <h1 class=\"title\">\n                    ")?;
i18n!(catalog, "Login").to_html(out)?;
write!(out, "\n                </h1>\n                <p class=\"subtitle\">\n                ")?;
i18n!(catalog, "Welcome back!").to_html(out)?;
write!(out, "\n                </p>\n                <span style=\"color: red;\">")?;
i18n!(catalog, "Sample Error").to_html(out)?;
write!(out, "</span>\n                <form method=\"POST\" action=\"/auth/sign_in\">\n                    <input type=\"hidden\" name=\"csrf_token\" value=\"")?;
csrf.to_html(out)?;
write!(out, "\">\n                    <input type=\"email\" name=\"email\" id=\"email\" placeholder=\"E-mail address\">\n                    <input type=\"password\" name=\"password\" id=\"password\" placeholder=\"Password\">\n                    <button>")?;
i18n!(catalog, "Log In").to_html(out)?;
write!(out, "</button>\n                </form>\n            </div>\n        </div>\n    </div>\n</section>\n")?;

Ok(())
}
)?;
write!(out, "\n")?;
Ok(())
}

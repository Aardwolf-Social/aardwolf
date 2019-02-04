use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
<<<<<<< HEAD
use rocket_i18n::i18n;
use crate::{SignIn, templates::{base, widgets::{alert, icon, password_input}, email::{email_input}}};
=======
use gettext_macros::i18n;
use crate::{SignIn, templates::{base, widgets::{alert, email_input, password_input}}};
<<<<<<< Updated upstream
>>>>>>> 4171ff0286ed26c81a011663cd6f23b09128cf9f
=======
>>>>>>> Stashed changes

pub fn sign_in(out: &mut Write, sign_in: &SignIn) -> io::Result<()> {
base(out, sign_in.catalog, "Aardwolf | Sign In", |out| {
out.write_all(b"\n<header>\n    <h2 class=\"title\">")?;
i18n!(sign_in.catalog, "Aardwolf Instance").to_html(out)?;
out.write_all(b"</h2>\n    <div style=\"text-align: right\">\n        <a href=\"sign_up\">")?;
i18n!(sign_in.catalog, "Need an Account? - Create one!").to_html(out)?;
out.write_all(b"</a>\n    </div>\n</header>\n<!-- End Page Header -->\n<section class=\"section\">\n    <div class=\"container\">\n\n        <div class=\"columns is-mobile\">\n            <div class=\"column\">\n                <h1 class=\"title\">\n                    ")?;
i18n!(sign_in.catalog, "About Aardwolf").to_html(out)?;
out.write_all(b"\n                </h1>\n                <p class=\"subtitle\">\n                ")?;
i18n!(sign_in.catalog, "This is who we are!").to_html(out)?;
out.write_all(b"\n                </p>\n                ")?;
i18n!(sign_in.catalog, "really-long-platform-description").to_html(out)?;
out.write_all(b"\n            </div>\n            <div class=\"column\">\n                <h1 class=\"title\">\n                    ")?;
i18n!(sign_in.catalog, "Login").to_html(out)?;
out.write_all(b"\n                </h1>\n                <p class=\"subtitle\">\n                ")?;
i18n!(sign_in.catalog, "Welcome back!").to_html(out)?;
out.write_all(b"\n                </p>\n                ")?;
if let Some(ref a) = sign_in.alert {
out.write_all(b"\n                    ")?;
alert(out, a)?;
out.write_all(b"\n                ")?;
}
out.write_all(b"\n                <form method=\"POST\" action=\"/auth/sign_in\">\n                    <input type=\"hidden\" name=\"csrf_token\" value=\"")?;
sign_in.csrf.to_html(out)?;
out.write_all(b"\">\n                    ")?;
email_input(out, &sign_in.email)?;
out.write_all(b"\n                    ")?;
password_input(out, &sign_in.password)?;
out.write_all(b"\n                    <button>")?;
i18n!(sign_in.catalog, "Log In").to_html(out)?;
out.write_all(b"</button>\n                </form>\n            </div>\n        </div>\n    </div>\n</section>\n")?;

Ok(())
}
)?;
out.write_all(b"\n")?;
Ok(())
}

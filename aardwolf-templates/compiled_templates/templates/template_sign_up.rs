use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
<<<<<<< HEAD
use rocket_i18n::i18n;
use crate::{SignUp, templates::{base, widgets::{alert, icon, password_input}, email::{email_input}}};
=======
use gettext_macros::i18n;
use crate::{SignUp, templates::{base, widgets::{alert, email_input, password_input}}};
>>>>>>> 4171ff0286ed26c81a011663cd6f23b09128cf9f

pub fn sign_up(out: &mut Write, sign_up: &SignUp) -> io::Result<()> {
base(out, sign_up.catalog, "Aardwolf | Sign Up", |out| {
out.write_all(b"\n<header>\n    <h2 class=\"title\">")?;
i18n!(sign_up.catalog, "Aardwolf Instance").to_html(out)?;
out.write_all(b"</h2>\n    <div style=\"text-align: right\">\n        <a href=\"sign_in\">")?;
i18n!(sign_up.catalog, "Have an Account? - Login").to_html(out)?;
out.write_all(b"</a>\n    </div>\n</header>\n<!-- End Page Header -->\n<section class=\"section\">\n    <div class=\"container\">\n\n        <div class=\"columns is-mobile\">\n            <div class=\"column\">\n                <h1 class=\"title\">\n                    ")?;
i18n!(sign_up.catalog, "About Aardwolf").to_html(out)?;
out.write_all(b"\n                </h1>\n                <p class=\"subtitle\">\n                ")?;
i18n!(sign_up.catalog, "This is who we are!").to_html(out)?;
out.write_all(b"\n                </p>\n                ")?;
i18n!(sign_up.catalog, "really-long-platform-description").to_html(out)?;
out.write_all(b"\n            </div>\n            <div class=\"column\">\n                <h1 class=\"title\">\n                    ")?;
i18n!(sign_up.catalog, "Create an Account").to_html(out)?;
out.write_all(b"\n                </h1>\n                <p class=\"subtitle\">\n                ")?;
i18n!(sign_up.catalog, "Feel free to sign up!").to_html(out)?;
out.write_all(b"\n                </p>\n                <form method=\"POST\" action=\"/auth/sign_up\">\n                    <input type=\"hidden\" name=\"csrf_token\" value=\"")?;
sign_up.csrf.to_html(out)?;
out.write_all(b"\">\n                    ")?;
if let Some(ref a) = sign_up.alert {
out.write_all(b"\n                        ")?;
alert(out, a)?;
out.write_all(b"\n                    ")?;
}
out.write_all(b"\n\n                    ")?;
email_input(out, &sign_up.email)?;
out.write_all(b"\n                    ")?;
password_input(out, &sign_up.password)?;
out.write_all(b"\n                    ")?;
password_input(out, &sign_up.password_confirmation)?;
out.write_all(b"\n                    <button>")?;
i18n!(sign_up.catalog, "Sign Up").to_html(out)?;
out.write_all(b"</button>\n                </form>\n            </div>\n        </div>\n    </div>\n</section>\n")?;

Ok(())
}
)?;
out.write_all(b"\n")?;
Ok(())
}

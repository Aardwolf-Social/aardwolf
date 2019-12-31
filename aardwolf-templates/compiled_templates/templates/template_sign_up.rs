use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use gettext_macros::i18n;
use crate::{SignUp, templates::{base, elements::{alert, input_email, input_password}}};

pub fn sign_up<W>(mut out: &mut W, sign_up: &SignUp) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
base(&mut out, sign_up.catalog, "Aardwolf | Sign Up", |mut out| {
out.write_all(b"\n<header>\n    <h2 class=\"title\">")?;
i18n!(sign_up.catalog, "Aardwolf Instance").to_html(&mut out)?;
out.write_all(b"</h2>\n    <div style=\"text-align: right\">\n        <a href=\"sign_in\">")?;
i18n!(sign_up.catalog, "Have an Account? - Login").to_html(&mut out)?;
out.write_all(b"</a>\n    </div>\n</header>\n<!-- End Page Header -->\n<section class=\"section\">\n    <div class=\"container\">\n\n        <div class=\"columns is-mobile\">\n            <div class=\"column\">\n                <h1 class=\"title\">\n                    ")?;
i18n!(sign_up.catalog, "About Aardwolf").to_html(&mut out)?;
out.write_all(b"\n                </h1>\n                <p class=\"subtitle\">\n                ")?;
i18n!(sign_up.catalog, "This is who we are!").to_html(&mut out)?;
out.write_all(b"\n                </p>\n                ")?;
i18n!(sign_up.catalog, "really-long-platform-description").to_html(&mut out)?;
out.write_all(b"\n            </div>\n            <div class=\"column\">\n                <h1 class=\"title\">\n                    ")?;
i18n!(sign_up.catalog, "Create an Account").to_html(&mut out)?;
out.write_all(b"\n                </h1>\n                <p class=\"subtitle\">\n                ")?;
i18n!(sign_up.catalog, "Feel free to sign up!").to_html(&mut out)?;
out.write_all(b"\n                </p>\n                <form method=\"POST\" action=\"/auth/sign_up\">\n                    <input type=\"hidden\" name=\"csrf_token\" value=\"")?;
sign_up.csrf.to_html(&mut out)?;
out.write_all(b"\">\n                    ")?;
if let Some(ref a) = sign_up.alert {
out.write_all(b"\n                        ")?;
alert(&mut out, a)?;
out.write_all(b"\n                    ")?;
}
out.write_all(b"\n\n                    ")?;
input_email(&mut out, &sign_up.email)?;
out.write_all(b"\n                    ")?;
input_password(&mut out, &sign_up.password)?;
out.write_all(b"\n                    ")?;
input_password(&mut out, &sign_up.password_confirmation)?;
out.write_all(b"\n                    <button>")?;
i18n!(sign_up.catalog, "Sign Up").to_html(&mut out)?;
out.write_all(b"</button>\n                </form>\n            </div>\n        </div>\n    </div>\n</section>\n")?;

Ok(())
}
)?;
out.write_all(b"\n")?;
Ok(())
}

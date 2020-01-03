use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use gettext_macros::i18n;
use crate::{SignUp, templates::{base, elements::{alert, input_email, input_password}}};

pub fn sign_up_html<W>(mut out: &mut W, sign_up: &SignUp) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
base(&mut out, sign_up.catalog, "Aardwolf | Sign Up", |mut out| {
out.write_all(b"\r\n<header>\r\n    <h2 class=\"title\">")?;
i18n!(sign_up.catalog, "Aardwolf Instance").to_html(&mut out)?;
out.write_all(b"</h2>\r\n    <div style=\"text-align: right\">\r\n        <a href=\"sign_in\">")?;
i18n!(sign_up.catalog, "Have an Account? - Login").to_html(&mut out)?;
out.write_all(b"</a>\r\n    </div>\r\n</header>\r\n<!-- End Page Header -->\r\n<section class=\"section\">\r\n    <div class=\"container\">\r\n\r\n        <div class=\"columns is-mobile\">\r\n            <div class=\"column\">\r\n                <h1 class=\"title\">\r\n                    ")?;
i18n!(sign_up.catalog, "About Aardwolf").to_html(&mut out)?;
out.write_all(b"\r\n                </h1>\r\n                <p class=\"subtitle\">\r\n                ")?;
i18n!(sign_up.catalog, "This is who we are!").to_html(&mut out)?;
out.write_all(b"\r\n                </p>\r\n                ")?;
i18n!(sign_up.catalog, "really-long-platform-description").to_html(&mut out)?;
out.write_all(b"\r\n            </div>\r\n            <div class=\"column\">\r\n                <h1 class=\"title\">\r\n                    ")?;
i18n!(sign_up.catalog, "Create an Account").to_html(&mut out)?;
out.write_all(b"\r\n                </h1>\r\n                <p class=\"subtitle\">\r\n                ")?;
i18n!(sign_up.catalog, "Feel free to sign up!").to_html(&mut out)?;
out.write_all(b"\r\n                </p>\r\n                <form method=\"POST\" action=\"/auth/sign_up\">\r\n                    <input type=\"hidden\" name=\"csrf_token\" value=\"")?;
sign_up.csrf.to_html(&mut out)?;
out.write_all(b"\">\r\n                    ")?;
if let Some(ref a) = sign_up.alert {
out.write_all(b"\r\n                        ")?;
alert(&mut out, a)?;
out.write_all(b"\r\n                    ")?;
}
out.write_all(b"\r\n\r\n                    ")?;
input_email(&mut out, &sign_up.email)?;
out.write_all(b"\r\n                    ")?;
input_password(&mut out, &sign_up.password)?;
out.write_all(b"\r\n                    ")?;
input_password(&mut out, &sign_up.password_confirmation)?;
out.write_all(b"\r\n                    <button>")?;
i18n!(sign_up.catalog, "Sign Up").to_html(&mut out)?;
out.write_all(b"</button>\r\n                </form>\r\n            </div>\r\n        </div>\r\n    </div>\r\n</section>\r\n")?;

Ok(())
}
)?;
out.write_all(b"\r\n")?;
Ok(())
}

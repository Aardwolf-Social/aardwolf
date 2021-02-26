use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use gettext_macros::i18n;
use crate::{SignIn, templates::{base, elements::{alert, input_email, input_password}}};

pub fn sign_in_html<W>(mut out: &mut W, sign_in: &SignIn) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
base(&mut out, sign_in.catalog, "Aardwolf | Sign In", |mut out| {
out.write_all(b"\r\n<header>\r\n    <h2 class=\"title\">")?;
i18n!(sign_in.catalog, "Aardwolf Instance").to_html(&mut out)?;
out.write_all(b"</h2>\r\n    <div style=\"text-align: right\">\r\n        <a href=\"sign_up\">")?;
i18n!(sign_in.catalog, "Need an Account? - Create one!").to_html(&mut out)?;
out.write_all(b"</a>\r\n    </div>\r\n</header>\r\n<!-- End Page Header -->\r\n<section class=\"section\">\r\n    <div class=\"container\">\r\n\t\r\n            <div class=\"columns is-centered\">\r\n\r\n\t\t\t\t<!-- ******************************* -->\r\n\t\t\t\t<!-- Begin Left-Hand Column Contents -->\r\n\t\t\t\t<!-- ******************************* -->\r\n                <div class=\"column is-mobile\">\r\n            <div class=\"column\">\r\n                <h1 class=\"title\">\r\n                    ")?;
i18n!(sign_in.catalog, "About Aardwolf").to_html(&mut out)?;
out.write_all(b"\r\n                </h1>\r\n                <p class=\"subtitle\">\r\n                ")?;
i18n!(sign_in.catalog, "This is who we are!").to_html(&mut out)?;
out.write_all(b"\r\n                </p>\r\n                ")?;
i18n!(sign_in.catalog, "really-long-platform-description").to_html(&mut out)?;
out.write_all(b"\r\n            </div>\r\n            <div class=\"column\">\r\n                <h1 class=\"title\">\r\n                    ")?;
i18n!(sign_in.catalog, "Login").to_html(&mut out)?;
out.write_all(b"\r\n                </h1>\r\n                <p class=\"subtitle\">\r\n                ")?;
i18n!(sign_in.catalog, "Welcome back!").to_html(&mut out)?;
out.write_all(b"\r\n                </p>\r\n                ")?;
if let Some(ref a) = sign_in.alert {
out.write_all(b"\r\n                    ")?;
alert(&mut out, a)?;
out.write_all(b"\r\n                ")?;
}
out.write_all(b"\r\n                <form method=\"POST\" action=\"/auth/sign_in\">\r\n                    <input type=\"hidden\" name=\"csrf_token\" value=\"")?;
sign_in.csrf.to_html(&mut out)?;
out.write_all(b"\">\r\n                    ")?;
input_email(&mut out, &sign_in.email)?;
out.write_all(b"\r\n                    ")?;
input_password(&mut out, &sign_in.password)?;
out.write_all(b"\r\n                    <button>")?;
i18n!(sign_in.catalog, "Log In").to_html(&mut out)?;
out.write_all(b"</button>\r\n\t\t\t\t</form>\r\n\t\t\t</div>\r\n\t\t\t<!-- End Left-Hand Column -->\r\n\t\t</div>\r\n\t</div>\r\n</section>\r\n")?;

Ok(())
}
)?;
out.write_all(b"\r\n\r\n\r\n")?;
Ok(())
}

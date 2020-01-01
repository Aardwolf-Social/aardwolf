use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use gettext_macros::i18n;
use crate::{SignIn, templates::{base, elements::{alert, input_email, input_password}}};

pub fn sign_in_html<W>(mut out: &mut W, sign_in: &SignIn) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
base(&mut out, sign_in.catalog, "Aardwolf | Sign In", |mut out| {
out.write_all(b"\n<header>\n    <h2 class=\"title\">")?;
i18n!(sign_in.catalog, "Aardwolf Instance").to_html(&mut out)?;
out.write_all(b"</h2>\n    <div style=\"text-align: right\">\n        <a href=\"sign_up\">")?;
i18n!(sign_in.catalog, "Need an Account? - Create one!").to_html(&mut out)?;
out.write_all(b"</a>\n    </div>\n</header>\n<!-- End Page Header -->\n<section class=\"section\">\n    <div class=\"container\">\n\t\n            <div class=\"columns is-centered\">\n\n\t\t\t\t<!-- ******************************* -->\n\t\t\t\t<!-- Begin Left-Hand Column Contents -->\n\t\t\t\t<!-- ******************************* -->\n                <div class=\"column is-mobile\">\n            <div class=\"column\">\n                <h1 class=\"title\">\n                    ")?;
i18n!(sign_in.catalog, "About Aardwolf").to_html(&mut out)?;
out.write_all(b"\n                </h1>\n                <p class=\"subtitle\">\n                ")?;
i18n!(sign_in.catalog, "This is who we are!").to_html(&mut out)?;
out.write_all(b"\n                </p>\n                ")?;
i18n!(sign_in.catalog, "really-long-platform-description").to_html(&mut out)?;
out.write_all(b"\n            </div>\n            <div class=\"column\">\n                <h1 class=\"title\">\n                    ")?;
i18n!(sign_in.catalog, "Login").to_html(&mut out)?;
out.write_all(b"\n                </h1>\n                <p class=\"subtitle\">\n                ")?;
i18n!(sign_in.catalog, "Welcome back!").to_html(&mut out)?;
out.write_all(b"\n                </p>\n                ")?;
if let Some(ref a) = sign_in.alert {
out.write_all(b"\n                    ")?;
alert(&mut out, a)?;
out.write_all(b"\n                ")?;
}
out.write_all(b"\n                <form method=\"POST\" action=\"/auth/sign_in\">\n                    <input type=\"hidden\" name=\"csrf_token\" value=\"")?;
sign_in.csrf.to_html(&mut out)?;
out.write_all(b"\">\n                    ")?;
input_email(&mut out, &sign_in.email)?;
out.write_all(b"\n                    ")?;
input_password(&mut out, &sign_in.password)?;
out.write_all(b"\n                    <button>")?;
i18n!(sign_in.catalog, "Log In").to_html(&mut out)?;
out.write_all(b"</button>\n\t\t\t\t</form>\n\t\t\t</div>\n\t\t\t<!-- End Left-Hand Column -->\n\t\t</div>\n\t</div>\n</section>\n")?;

Ok(())
}
)?;
out.write_all(b"\n\n\n")?;
Ok(())
}

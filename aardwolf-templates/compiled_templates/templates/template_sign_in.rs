use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use gettext_macros::i18n;
use crate::{SignIn, templates::{base, ui::{alert, email_input, password_input}}};

pub fn sign_in<W>(mut out: &mut W, sign_in: &SignIn) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
base(&mut out, sign_in.catalog, "Aardwolf | Sign In", |mut out| {
out.write_all(b"\n<header>\n    <h2 class=\"title\">")?;
i18n!(sign_in.catalog, "Aardwolf Instance").to_html(&mut out)?;
out.write_all(b"</h2>\n    <div style=\"text-align: right\">\n        <a href=\"sign_up\">")?;
i18n!(sign_in.catalog, "Need an Account? - Create one!").to_html(&mut out)?;
out.write_all(b"</a>\n    </div>\n</header>\n<!-- End Page Header -->\n<section class=\"section\">\n    <div class=\"container\">\n\n        <div class=\"columns is-mobile\">\n            <div class=\"column\">\n                <h1 class=\"title\">\n                    ")?;
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
email_input(&mut out, &sign_in.email)?;
out.write_all(b"\n                    ")?;
password_input(&mut out, &sign_in.password)?;
out.write_all(b"\n                    <button>")?;
i18n!(sign_in.catalog, "Log In").to_html(&mut out)?;
out.write_all(b"</button>\n                </form>\n            </div>\n        </div>\n    </div>\n</section>\n")?;

Ok(())
}
)?;
out.write_all(b"\n")?;
Ok(())
}

use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use gettext_macros::i18n;
use crate::{FirstLogin, templates::{base, elements::{alert, input_text, input_select, input_checkbox}}};

pub fn first_login_html<W>(mut out: &mut W, first_login: &FirstLogin) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
base(&mut out, first_login.catalog, "Aardwolf | Get Posting", |mut out| {
out.write_all(b"\r\n<header>\r\n    <h2 class=\"title\">")?;
i18n!(first_login.catalog, "Get posting!").to_html(&mut out)?;
out.write_all(b"</h2>\r\n    <div style=\"text-align: right\">\r\n        ")?;
i18n!(first_login.catalog, "Fill out your profile information").to_html(&mut out)?;
out.write_all(b"\r\n    </div>\r\n</header>\r\n<section>\r\n    <div class=\"container\">\r\n        <div class=\"columns is-mobile\">\r\n            <form method=\"POST\" action=\"/personas/create\">\r\n                ")?;
if let Some(ref a) = first_login.alert {
out.write_all(b"\r\n                    ")?;
alert(&mut out, a)?;
out.write_all(b"\r\n                ")?;
}
out.write_all(b"\r\n                <input type=\"hidden\" name=\"csrf_token\" value=\"")?;
first_login.csrf.to_html(&mut out)?;
out.write_all(b"\">\r\n                ")?;
input_text(&mut out, &first_login.display_name)?;
out.write_all(b"\r\n                ")?;
input_text(&mut out, &first_login.shortname)?;
out.write_all(b"\r\n                ")?;
input_select(&mut out, &first_login.follow_policy)?;
out.write_all(b"\r\n                ")?;
input_select(&mut out, &first_login.default_visibility)?;
out.write_all(b"\r\n                ")?;
input_checkbox(&mut out, &first_login.is_searchable)?;
out.write_all(b"\r\n                <button>")?;
i18n!(first_login.catalog, "Create Persona").to_html(&mut out)?;
out.write_all(b"</button>\r\n            </form>\r\n        </div>\r\n    </div>\r\n</section>\r\n")?;

Ok(())
}
)?;
out.write_all(b";\r\n")?;
Ok(())
}

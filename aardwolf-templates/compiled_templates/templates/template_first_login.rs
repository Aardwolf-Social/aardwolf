use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use gettext_macros::i18n;
use crate::{FirstLogin, templates::{base, ui::{alert, text_input, select_input, checkbox_input}}};

pub fn first_login<W>(mut out: &mut W, first_login: &FirstLogin) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
base(&mut out, first_login.catalog, "Aardwolf | Get Posting", |mut out| {
out.write_all(b"\n<header>\n    <h2 class=\"title\">")?;
i18n!(first_login.catalog, "Get posting!").to_html(&mut out)?;
out.write_all(b"</h2>\n    <div style=\"text-align: right\">\n        ")?;
i18n!(first_login.catalog, "Fill out your profile information").to_html(&mut out)?;
out.write_all(b"\n    </div>\n</header>\n<section>\n    <div class=\"container\">\n        <div class=\"columns is-mobile\">\n            <form method=\"POST\" action=\"/personas/create\">\n                ")?;
if let Some(ref a) = first_login.alert {
out.write_all(b"\n                    ")?;
alert(&mut out, a)?;
out.write_all(b"\n                ")?;
}
out.write_all(b"\n                <input type=\"hidden\" name=\"csrf_token\" value=\"")?;
first_login.csrf.to_html(&mut out)?;
out.write_all(b"\">\n                ")?;
text_input(&mut out, &first_login.display_name)?;
out.write_all(b"\n                ")?;
text_input(&mut out, &first_login.shortname)?;
out.write_all(b"\n                ")?;
select_input(&mut out, &first_login.follow_policy)?;
out.write_all(b"\n                ")?;
select_input(&mut out, &first_login.default_visibility)?;
out.write_all(b"\n                ")?;
checkbox_input(&mut out, &first_login.is_searchable)?;
out.write_all(b"\n                <button>")?;
i18n!(first_login.catalog, "Create Persona").to_html(&mut out)?;
out.write_all(b"</button>\n            </form>\n        </div>\n    </div>\n</section>\n")?;

Ok(())
}
)?;
out.write_all(b";\n")?;
Ok(())
}

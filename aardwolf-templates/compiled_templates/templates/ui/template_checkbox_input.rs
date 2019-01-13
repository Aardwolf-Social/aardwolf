use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use rocket_i18n::i18n;
use crate::{CheckboxInput, templates::ui::icon};

pub fn checkbox_input(out: &mut Write, input: &CheckboxInput) -> io::Result<()> {
out.write_all(b"<div class=\"aardwolf-input-wrapper\">\n    <label for=\"")?;
input.name.to_html(out)?;
out.write_all(b"\">\n        ")?;
if let Some(ref i) = input.icon {
out.write_all(b"\n            ")?;
icon(out, i)?;
out.write_all(b"\n        ")?;
}
out.write_all(b"\n        ")?;
i18n!(input.catalog, input.label).to_html(out)?;
out.write_all(b"\n    </label>\n    <div class=\"aardwolf-input aardwolf-checkbox-input\">\n        <input type=\"checkbox\" name=\"")?;
input.name.to_html(out)?;
out.write_all(b"\" value=\"true\" checked=\"")?;
input.checked.to_html(out)?;
out.write_all(b"\" />\n    </div>\n    <div class=\"aardwolf-input-meta\">\n        ")?;
if let Some(error) = input.error {
out.write_all(b"\n            <span class=\"aardwolf-input-error\">")?;
i18n!(input.catalog, error).to_html(out)?;
out.write_all(b"</span>\n        ")?;
}
out.write_all(b"\n    </div>\n</div>\n")?;
Ok(())
}

use std::io::{self, Write};
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use ::templates::{Html,ToHtml};
use rocket_i18n::i18n;
use crate::{Input, templates::ui::icon};

pub fn input(out: &mut Write, input: Input)
-> io::Result<()> {
write!(out, "<div class=\"aardwolf-input-wrapper\">\n    <div class=\"aardwolf-input aardwolf-")?;
input.kind.to_html(out)?;
write!(out, "-input\">\n        ")?;
if let Some(i) = input.icon {
write!(out, "\n            ")?;
icon(out, i)?;
write!(out, "\n        ")?;
}
write!(out, "\n        ")?;
if let Some(placeholder) = input.placeholder {
write!(out, "\n            <input type=\"")?;
input.kind.to_html(out)?;
write!(out, "\" name=\"")?;
input.name.to_html(out)?;
write!(out, "\" placeholder=\"")?;
i18n!(input.catalog, placeholder).to_html(out)?;
write!(out, "\" value=\"")?;
input.value.to_html(out)?;
write!(out, "\" />\n        ")?;
} else {
write!(out, "\n            <input type=\"")?;
input.kind.to_html(out)?;
write!(out, "\" name=\"")?;
input.name.to_html(out)?;
write!(out, "\" value=\"")?;
input.value.to_html(out)?;
write!(out, "\" />\n        ")?;
}
write!(out, "\n    </div>\n    <div class=\"aardwolf-input-meta\">\n        ")?;
if let Some(error) = input.error {
write!(out, "\n            <span class=\"aardwolf-input-error\">")?;
i18n!(input.catalog, error).to_html(out)?;
write!(out, "</span>\n        ")?;
}
write!(out, "\n    </div>\n</div>\n")?;
Ok(())
}

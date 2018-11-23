use std::io::{self, Write};
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use ::templates::{Html,ToHtml};
use gettext::Catalog;
use rocket_i18n::i18n;

pub fn input(out: &mut Write, catalog: &Catalog, kind: &str, name: &str, icon: Option<&str>, placeholder: Option<&str>, value: &str, error: Option<String>)
-> io::Result<()> {
write!(out, "<div class=\"aardwolf-input-wrapper\">\n    <div class=\"aardwolf-input aardwolf-")?;
kind.to_html(out)?;
write!(out, "-input\">\n        ")?;
if let Some(icon) = icon {
write!(out, "\n            <span class=\"icon icon-")?;
icon.to_html(out)?;
write!(out, "\"></span>\n        ")?;
}
write!(out, "\n        ")?;
if let Some(placeholder) = placeholder {
write!(out, "\n            <input type=\"")?;
kind.to_html(out)?;
write!(out, "\" name=\"")?;
name.to_html(out)?;
write!(out, "\" placeholder=\"")?;
i18n!(catalog, placeholder).to_html(out)?;
write!(out, "\" value=\"")?;
value.to_html(out)?;
write!(out, "\" />\n        ")?;
} else {
write!(out, "\n            <input type=\"")?;
kind.to_html(out)?;
write!(out, "\" name=\"")?;
name.to_html(out)?;
write!(out, "\" value=\"")?;
value.to_html(out)?;
write!(out, "\" />\n        ")?;
}
write!(out, "\n    </div>\n    <div class=\"aardwolf-input-meta\">\n        ")?;
if let Some(error) = error {
write!(out, "\n            <span class=\"aardwolf-input-error\">")?;
error.to_html(out)?;
write!(out, "</span>\n        ")?;
}
write!(out, "\n    </div>\n</div>\n")?;
Ok(())
}

use std::io::{self, Write};
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use ::templates::{Html,ToHtml};
use gettext::Catalog;
use rocket_i18n::i18n;

pub fn alert(out: &mut Write, catalog: &Catalog, kind: &str, message: &str)
-> io::Result<()> {
write!(out, "<div class=\"aardwolf-alert\">\n    <div class=\"aardwolf-alert-meta\">\n        ")?;
if kind == "error" {
write!(out, "\n            ")?;
i18n!(catalog, "Error").to_html(out)?;
write!(out, "\n        ")?;
} else {
write!(out, "\n            ")?;
if kind == "warning" {
write!(out, "\n                ")?;
i18n!(catalog, "Warning").to_html(out)?;
write!(out, "\n            ")?;
} else {
write!(out, "\n                ")?;
if kind == "info" {
write!(out, "\n                    ")?;
i18n!(catalog, "Info").to_html(out)?;
write!(out, "\n                ")?;
}
write!(out, "\n            ")?;
}
write!(out, "\n        ")?;
}
write!(out, "\n    </div>\n    ")?;
i18n!(catalog, message).to_html(out)?;
write!(out, "\n</div>\n")?;
Ok(())
}

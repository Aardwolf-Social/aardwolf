use std::io::{self, Write};
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use ::templates::{Html,ToHtml};
use rocket_i18n::i18n;
use crate::{Alert, AlertKind};

pub fn alert(out: &mut Write, alert: Alert)
-> io::Result<()> {
write!(out, "<div class=\"aardwolf-alert\">\n    <div class=\"aardwolf-alert-meta\">\n        ")?;
if alert.kind == AlertKind::Error {
write!(out, "\n            ")?;
i18n!(alert.catalog, "Error").to_html(out)?;
write!(out, "\n        ")?;
} else {
write!(out, "\n            ")?;
if alert.kind == AlertKind::Warning {
write!(out, "\n                ")?;
i18n!(alert.catalog, "Warning").to_html(out)?;
write!(out, "\n            ")?;
} else {
write!(out, "\n                ")?;
if alert.kind == AlertKind::Info {
write!(out, "\n                    ")?;
i18n!(alert.catalog, "Info").to_html(out)?;
write!(out, "\n                ")?;
}
write!(out, "\n            ")?;
}
write!(out, "\n        ")?;
}
write!(out, "\n    </div>\n    ")?;
i18n!(alert.catalog, alert.message).to_html(out)?;
write!(out, "\n</div>\n")?;
Ok(())
}

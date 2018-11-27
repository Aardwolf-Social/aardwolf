use std::io::{self, Write};
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use rocket_i18n::i18n;
use crate::{Alert, templates::ui::icon};

pub fn alert(out: &mut Write, alert: &Alert) -> io::Result<()> {
out.write_all(b"<div class=\"aardwolf-alert aardwolf-alert-")?;
alert.kind.to_html(out)?;
out.write_all(b"\">\n    <div class=\"aardwolf-alert-meta\">\n        ")?;
icon(out, "warning")?;
out.write_all(b"\n    </div>\n    <div class=\"aardwolf-alert-message\">\n        ")?;
i18n!(alert.catalog, alert.message).to_html(out)?;
out.write_all(b"\n    </div>\n</div>\n")?;
Ok(())
}

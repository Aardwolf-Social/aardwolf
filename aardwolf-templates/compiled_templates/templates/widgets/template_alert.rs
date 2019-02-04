use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use crate::{Alert, templates::widgets::icon};

pub fn alert(out: &mut Write, alert: &Alert) -> io::Result<()> {
out.write_all(b"<div class=\"aardwolf-alert aardwolf-alert-")?;
alert.kind.to_html(out)?;
out.write_all(b"\">\n    <div class=\"aardwolf-alert-meta\">\n        ")?;
icon(out, "warning")?;
out.write_all(b"\n    </div>\n    <div class=\"aardwolf-alert-message\">\n        ")?;
alert.message.to_html(out)?;
out.write_all(b"\n    </div>\n</div>\n")?;
Ok(())
}

use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use crate::{Alert, templates::elements::icon};

pub fn alert_html<W>(mut out: &mut W, alert: &Alert) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
out.write_all(b"<div class=\"aardwolf-alert aardwolf-alert-")?;
alert.kind.to_html(&mut out)?;
out.write_all(b"\">\r\n    <div class=\"aardwolf-alert-meta\">\r\n        ")?;
icon(&mut out, "warning")?;
out.write_all(b"\r\n    </div>\r\n    <div class=\"aardwolf-alert-message\">\r\n        ")?;
alert.message.to_html(&mut out)?;
out.write_all(b"\r\n    </div>\r\n</div>\r\n")?;
Ok(())
}

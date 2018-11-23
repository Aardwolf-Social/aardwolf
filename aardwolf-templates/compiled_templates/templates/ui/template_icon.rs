use std::io::{self, Write};
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use ::templates::{Html,ToHtml};

pub fn icon(out: &mut Write, icon: &str)
-> io::Result<()> {
write!(out, "<i class=\"fa fa-")?;
icon.to_html(out)?;
write!(out, "\"></i>\n")?;
Ok(())
}

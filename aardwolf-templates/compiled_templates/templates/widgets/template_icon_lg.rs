use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};

pub fn icon_lg(out: &mut Write, icon: &str) -> io::Result<()> {
out.write_all(b"<span class=\"fa fa-")?;
icon.to_html(out)?;
out.write_all(b" fa-lg\" aria-hidden=\"true\"></span>\n")?;
Ok(())
}

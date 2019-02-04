use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use rocket_i18n::i18n;

pub fn icon(out: &mut Write, icon: &Icon) -> io::Result<()> {
(icon: &str).to_html(out)?;
out.write_all(b"\n<span class=\"fa fa-")?;
icon.to_html(out)?;
out.write_all(b"\"></span>\n\n")?;
(icon_lg: &str).to_html(out)?;
out.write_all(b"\n<span class=\"fa fa")?;
icon.to_html(out)?;
out.write_all(b" fa-lg\"></span>\n\n")?;
(icon_2x: &str).to_html(out)?;
out.write_all(b"\n<span class=\"fa fa")?;
icon.to_html(out)?;
out.write_all(b" fa-2x\"></span>\n\n")?;
(icon_3x: &str).to_html(out)?;
out.write_all(b"\n<span class=\"fa fa")?;
icon.to_html(out)?;
out.write_all(b" fa-3x\"></span>\n\n")?;
(icon_4x: &str).to_html(out)?;
out.write_all(b"\n<span class=\"fa fa")?;
icon.to_html(out)?;
out.write_all(b" fa-4x\"></span>\n\n")?;
(icon_5x: &str).to_html(out)?;
out.write_all(b"\n<span class=\"fa fa")?;
icon.to_html(out)?;
out.write_all(b" fa-5x\"></span>\n")?;
Ok(())
}

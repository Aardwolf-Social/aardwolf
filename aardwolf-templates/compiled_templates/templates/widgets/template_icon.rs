use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use rocket_i18n::i18n;

<<<<<<< HEAD
pub fn icon(out: &mut Write, icon: &Icon) -> io::Result<()> {
(icon: &str).to_html(out)?;
out.write_all(b"\n<span class=\"fa fa-")?;
=======
pub fn icon(out: &mut Write, icon: &str) -> io::Result<()> {
out.write_all(b"<i class=\"fa fa-")?;
>>>>>>> 4171ff0286ed26c81a011663cd6f23b09128cf9f
icon.to_html(out)?;
out.write_all(b"\"></i>\n")?;
Ok(())
}

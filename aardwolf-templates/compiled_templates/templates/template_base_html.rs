use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use gettext::Catalog;
use crate::templates::{footer, html_head};

pub fn base_html<W>(mut out: &mut W, catalog: &Catalog, title: &str, content: impl FnOnce(&mut W) -> io::Result<()>) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
out.write_all(b"<!DOCTYPE html>\r\n<html lang=\"en\">\r\n    ")?;
head(&mut out, title)?;
out.write_all(b"\r\n    <body>\r\n        ")?;
content(&mut out)?;
out.write_all(b"\r\n    </body>\r\n    ")?;
footer(&mut out, catalog)?;
out.write_all(b"\r\n</html>\r\n")?;
Ok(())
}

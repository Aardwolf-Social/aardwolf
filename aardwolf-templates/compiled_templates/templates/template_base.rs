use std::io::{self, Write};
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use gettext::Catalog;
use crate::templates::{footer, head};

pub fn base(out: &mut Write, catalog: &Catalog, title: &str, content: impl FnOnce(&mut Write) -> io::Result<()>) -> io::Result<()> {
out.write_all(b"<!DOCTYPE html>\n<html lang=\"en\">\n    ")?;
head(out, catalog, title)?;
out.write_all(b"\n    <body>\n        ")?;
content(out)?;
out.write_all(b"\n        ")?;
footer(out, catalog)?;
out.write_all(b"\n    </body>\n</html>\n")?;
Ok(())
}

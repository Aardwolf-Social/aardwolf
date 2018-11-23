use std::io::{self, Write};
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use ::templates::{Html,ToHtml};
use gettext::Catalog;
use crate::templates::{footer, head};

pub fn base<Content>(out: &mut Write, catalog: &Catalog, title: &str, content: Content)
-> io::Result<()> 
where Content: FnOnce(&mut Write) -> io::Result<()>{
write!(out, "<!DOCTYPE html>\n<html lang=\"en\">\n    ")?;
head(out, catalog, title)?;
write!(out, "\n    <body>\n        ")?;
content(out)?;
write!(out, "\n        ")?;
footer(out, catalog)?;
write!(out, "\n    </body>\n</html>\n")?;
Ok(())
}

use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};

pub fn head<W: Write>(mut out: W, title: &str) -> io::Result<()> {
out.write_all(b"<head>\n    <meta charset=\"utf-8\" />\n    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />\n    <title>")?;
title.to_html(&mut out)?;
out.write_all(b"</title>\n    <link rel=\"stylesheet\" href=\"/web/app.css\" />\n    <link rel=\"stylesheet\" type=\"text/css\" href=\"/stylesheets/scratchpad.css\" />\n</head>\n")?;
Ok(())
}

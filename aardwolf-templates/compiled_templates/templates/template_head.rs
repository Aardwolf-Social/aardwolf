use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};

<<<<<<< HEAD:aardwolf-templates/compiled_templates/templates/template_head.rs
pub fn head(out: &mut Write, catalog: &Catalog, title: &str) -> io::Result<()> {
=======
pub fn html_head(out: &mut Write, title: &str) -> io::Result<()> {
>>>>>>> 4171ff0286ed26c81a011663cd6f23b09128cf9f:aardwolf-templates/compiled_templates/templates/template_html_head.rs
out.write_all(b"<head>\n    <meta charset=\"utf-8\" />\n    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />\n    <title>")?;
title.to_html(out)?;
out.write_all(b"</title>\n    <link rel=\"stylesheet\" href=\"/web/app.css\" />\n    <link rel=\"stylesheet\" type=\"text/css\" href=\"/stylesheets/scratchpad.css\" />\n</head>\n")?;
Ok(())
}

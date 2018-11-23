use std::io::{self, Write};
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use ::templates::{Html,ToHtml};
use gettext::Catalog;
use crate::templates::ui::input;

pub fn text_input(out: &mut Write, catalog: &Catalog, name: &str, icon: Option<&str>, placeholder: Option<&str>, value: &str, error: Option<String>)
-> io::Result<()> {
input(out, catalog, "text", name, icon, placeholder, value, error)?;
write!(out, "\n")?;
Ok(())
}

use std::io::{self, Write};
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use ::templates::{Html,ToHtml};
use gettext::Catalog;
use crate::templates::ui::input;

pub fn email_input(out: &mut Write, catalog: &Catalog, name: &str, placeholder: &str, value: &str, error: Option<String>)
-> io::Result<()> {
input(out, catalog, "email", name, Some("envelope"), Some(placeholder), value, error)?;
write!(out, "\n")?;
Ok(())
}

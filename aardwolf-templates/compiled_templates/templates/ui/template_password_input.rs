use std::io::{self, Write};
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use ::templates::{Html,ToHtml};
use gettext::Catalog;
use crate::templates::ui::input;

pub fn password_input(out: &mut Write, catalog: &Catalog, name: &str, placeholder: &str, error: Option<String>)
-> io::Result<()> {
input(out, catalog, "password", name, Some("lock"), Some(placeholder), "", error)?;
write!(out, "\n")?;
Ok(())
}

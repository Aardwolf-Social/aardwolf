use std::io::{self, Write};
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use ::templates::{Html,ToHtml};
use crate::{TextInput, templates::ui::input};

pub fn text_input(out: &mut Write, text_input: TextInput)
-> io::Result<()> {
input(out, text_input.into())?;
write!(out, "\n")?;
Ok(())
}

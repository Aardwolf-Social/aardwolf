use std::io::{self, Write};
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use ::templates::{Html,ToHtml};
use crate::{PasswordInput, templates::ui::input};

pub fn password_input(out: &mut Write, password_input: PasswordInput)
-> io::Result<()> {
input(out, password_input.into())?;
write!(out, "\n")?;
Ok(())
}

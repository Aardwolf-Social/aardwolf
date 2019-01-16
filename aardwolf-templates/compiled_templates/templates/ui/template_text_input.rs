use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use crate::{TextInput, templates::ui::input};

pub fn text_input(out: &mut Write, text_input: &TextInput) -> io::Result<()> {
input(out, &text_input.into())?;
out.write_all(b"\n")?;
Ok(())
}

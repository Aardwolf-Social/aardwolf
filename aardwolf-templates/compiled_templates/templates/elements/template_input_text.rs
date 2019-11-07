use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use crate::{TextInput, templates::elements::input};

pub fn input_text<W>(mut out: &mut W, input_text: &TextInput) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
input(&mut out, &input_text.into())?;
out.write_all(b"\n")?;
Ok(())
}

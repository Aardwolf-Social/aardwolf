use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use crate::{InputText, templates::elements::input};

pub fn input_text_html<W>(mut out: &mut W, input_text: &InputText) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
input(&mut out, &input_text.into())?;
out.write_all(b"\r\n")?;
Ok(())
}

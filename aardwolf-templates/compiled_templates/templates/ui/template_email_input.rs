use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use crate::{EmailInput, templates::ui::input};

pub fn email_input<W>(mut out: &mut W, email_input: &EmailInput) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
input(&mut out, &email_input.into())?;
out.write_all(b"\n")?;
Ok(())
}

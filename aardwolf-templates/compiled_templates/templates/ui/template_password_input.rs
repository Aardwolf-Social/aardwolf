use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use crate::{PasswordInput, templates::ui::input};

pub fn password_input(out: &mut Write, password_input: &PasswordInput) -> io::Result<()> {
input(out, &password_input.into())?;
out.write_all(b"\n")?;
Ok(())
}

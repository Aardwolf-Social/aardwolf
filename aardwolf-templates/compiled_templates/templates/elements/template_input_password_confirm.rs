use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use crate::{InputPasswordConfirm, templates::elements::input};

pub fn input_password_confirm<W>(mut out: &mut W, input_password_confirm: &InputPasswordConfirm) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
input(&mut out, &input_password_confirm.into())?;
out.write_all(b"\n\n<!-- Reusable Password Confirm Input -->\n<div class=\"field\">\n\t<label class=\"label\">")?;
i18n!(catalog, "Confirm Password").to_html(&mut out)?;
out.write_all(b"</label>\n\t<div class=\"control has-icons-left\">\n\t\t<input class=\"input\" type=\"password\" placeholder=\"*************\" required>\n\t\t<span class=icon is-small is-left>\n\t\t\t<span class=\"fa fa-lock\"></span>\n\t\t</span>\n\t</div>\n</div>\n<!-- End Reusable Password Confirm Input -->\n")?;
Ok(())
}

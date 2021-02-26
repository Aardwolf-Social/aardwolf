use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use crate::{InputPassword, templates::elements::input};

pub fn input_password_html<W>(mut out: &mut W, input_password: &InputPassword) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
input(&mut out, &input_password.into())?;
out.write_all(b"\r\n\r\n\r\n<!-- Reusable Password Input -->\r\n<div class=\"field\">\r\n\t<label class=\"label\">")?;
i18n!(catalog, "Password").to_html(&mut out)?;
out.write_all(b"</label>\r\n\t<div class=\"control has-icons-left\">\r\n\t\t<input class=\"input\" type=\"password\" placeholder=\"*************\" required>\r\n\t\t<span class=icon is-small is-left>\r\n\t\t\t<span class=\"fa fa-lock\"></span>\r\n\t\t</span>\r\n\t</div>\r\n</div>\r\n<!-- End Reusable Password Input -->\r\n")?;
Ok(())
}

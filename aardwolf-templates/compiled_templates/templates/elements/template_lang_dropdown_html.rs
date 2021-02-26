use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use gettext_macros::i18n;
use crate::templates::{base, elements::{alert,lang_dropdown}};

pub fn lang_dropdown_html<W>(mut out: &mut W, lang_dropdown: &LangDropdown) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
out.write_all(b"<div class=\"field\">\r\n  <p class=\"control is-expanded has-icons-left has-icons-right\">\r\n    <div class=\"select is-narrow\">\r\n      <select>\r\n        <option value=\"english\">English</option>\r\n        <option value=\"awoo\">Awoo</option>\r\n        <option value=\"japanese\">Japanese</option>\r\n        <option value=\"parcel_tongue\">Parcel Tongue</option>\r\n        <option value=\"polish\">Polish</option>\r\n        <option value=\"redneck\">Redneck</option>\r\n        <option value=\"urdu\">urdu</option>\r\n      </select>\r\n    </div>\r\n  </p>\r\n</div>\r\n")?;
Ok(())
}

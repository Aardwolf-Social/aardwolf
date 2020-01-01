use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use gettext_macros::i18n;
use crate::{LangDropdown, templates::{base, elements::{alert}}};

pub fn lang_dropdown_html<W>(mut out: &mut W, lang_dropdown: &LangDropdown) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
out.write_all(b"<div class=\"field\">\n  <p class=\"control is-expanded has-icons-left has-icons-right\">\n    <div class=\"select is-narrow\">\n      <select>\n        <option value=\"english\">English</option>\n        <option value=\"awoo\">Awoo</option>\n        <option value=\"japanese\">Japanese</option>\n        <option value=\"parcel_tongue\">Parcel Tongue</option>\n        <option value=\"polish\">Polish</option>\n        <option value=\"redneck\">Redneck</option>\n        <option value=\"urdu\">urdu</option>\n      </select>\n    </div>\n  </p>\n</div>\n")?;
Ok(())
}

use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use crate::{SearchBar, templates::{base}};

pub fn search_bar_html<W>(mut out: &mut W, search_bar: &SearchBar) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
out.write_all(b"<div class=\"search-bar field\">\r\n   <p class=\"control has-icons-left has-icons-right\">\r\n      <input class=\"input\" type=\"text\" placeholder=\"Search\">\r\n      <span class=\"icon is-small is-left\">\r\n      \t<i class=\"fa fa-search\" aria-hidden=\"true\"></i>\r\n      </span>\r\n   </p>\r\n</div>\r\n")?;
Ok(())
}

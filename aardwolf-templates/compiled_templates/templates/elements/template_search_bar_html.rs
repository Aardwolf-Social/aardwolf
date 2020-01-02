use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use crate::{SearchBar, templates::{base}};

pub fn search_bar_html<W>(mut out: &mut W, search_bar: &SearchBar) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
out.write_all(b"<div class=\"search-bar field\">\n   <p class=\"control has-icons-left has-icons-right\">\n      <input class=\"input\" type=\"text\" placeholder=\"Search\">\n      <span class=\"icon is-small is-left\">\n      \t<i class=\"fa fa-search\" aria-hidden=\"true\"></i>\n      </span>\n   </p>\n</div>\n")?;
Ok(())
}

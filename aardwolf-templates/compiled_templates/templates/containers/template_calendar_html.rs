use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use gettext::Catalog;
use gettext_macros::i18n;

pub fn calendar_html<W>(mut out: &mut W, catalog: &Catalog) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
out.write_all(b"<section class=\"section\"><!-- Right-column Section -->\r\n\r\n\t<p>Hey check it out!  There is a Calendar with date/time pickers on <a href=\"https://creativebulma.net\">Creative Bulma</a>!!<br />\r\n\t<a href=\"https://creativebulma.net/product/calendar\">https://creativebulma.net/product/calendar</a>\r\n\r\n</section><!-- /Right-column Section -->\r\n")?;
Ok(())
}

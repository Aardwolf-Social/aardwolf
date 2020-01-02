use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use gettext::Catalog;
use crate::templates::{html_head, footer, home::{home_nav_top, home}};

pub fn home_base<W>(mut out: &mut W, catalog: &Catalog, title: &str, content: impl FnOnce(&mut W) -> io::Result<()>) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
out.write_all(b"<!DOCTYPE html>\n<html>\n\n<!-- *********** -->\n<!-- HTML Header -->\n<!-- *********** -->\n\n<body>\n\n<!-- ********************** -->\n<!-- Logged in Top Nav Menu -->\n<!-- ********************** -->\n\n<div class=\"columns is-mobile\"><!-- Column Content Container -->\n  <div class=\"left-column columns is-3\"><!-- Left-column (Nav) -->\n\n\t<!-- ****************** -->\n\t<!-- Left-hand Nav Menu -->\n\t<!-- ****************** -->\n\n  </div><!-- /Left-column (Nav)-->\n  <div class=\"right-column columns\"><!-- Right-column (Main content) -->\n    <section class=\"section\"><!-- Right-column Section -->\n\n\t<!-- *********************** -->\n\t<!-- Right-hand Body Content -->\n\t<!-- *********************** -->\n\n    </section><!-- /Right-column Section -->\n  </div><!-- /Right-column -->\n</div><!-- /Column Content Container -->\n\n</body>\n\n\t<!-- ************** -->\n\t<!-- Footer Content -->\n\t<!-- ************** -->\n\n</html>\n")?;
Ok(())
}

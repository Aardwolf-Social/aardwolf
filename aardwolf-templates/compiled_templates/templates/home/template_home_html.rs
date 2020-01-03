use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use crate::{Home, templates::{base, home::{feed, nav_top}, posts::{new}, asides::{shortcuts}}};

pub fn home_html<W>(mut out: &mut W, home: &Home) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
base(&mut out, home.catalog, "Aardwolf | Home", |mut out| {
out.write_all(b"\r\n    ")?;
nav(&mut out, home.catalog)?;
out.write_all(b"\r\n\t<div class=\"columns is-mobile\"><!-- Column Content Container -->\r\n\t  <div class=\"left-column columns is-3\"><!-- Left-column (Nav) -->\r\n\r\n\t\t<!-- Left-hand Nav Menu -->\r\n\t\t")?;
shortcuts(&mut out, &home.shortcuts)?;
out.write_all(b"\r\n\r\n\t  </div><!-- /Left-column (Nav)-->\r\n\t  <div class=\"right-column columns\"><!-- Right-column (Main content) -->\r\n\t\t<section class=\"section\"><!-- Right-column Section -->\r\n\r\n\t\t<!-- Right-hand Body Content -->\r\n\t\t")?;
new_post(&mut out, home.catalog, &home.shortcuts.username, &home.new_post)?;
out.write_all(b"\r\n\t\t")?;
feed(&mut out, home.catalog)?;
out.write_all(b"\r\n\r\n\t\t</section><!-- /Right-column Section -->\r\n\t  </div><!-- /Right-column -->\r\n\t</div><!-- /Column Content Container -->\r\n")?;

Ok(())
}
)?;
out.write_all(b"\r\n\r\n\r\n\r\n")?;
Ok(())
}

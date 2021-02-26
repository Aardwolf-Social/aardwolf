use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use gettext::Catalog;
use gettext_macros::i18n;
use crate::templates::elements::icon;

pub fn footer_html<W>(mut out: &mut W, catalog: &Catalog) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
out.write_all(b"<footer class=\"footer\">\r\n  <div class=\"container\">\r\n    <div class=\"content has-text-centered\">\r\n        <a href=\"termsofservice.html\" class=\"footer_box\">")?;
i18n!(catalog, "Terms of Service").to_html(&mut out)?;
out.write_all(b"</a>\r\n        <span class=\"vertical_line\"/>\r\n        <span class=\"footer_box\">")?;
i18n!(catalog, "Copyright 2018").to_html(&mut out)?;
out.write_all(b"</span>\r\n        <span class=\"vertical_line\" />\r\n        <a href=\"https://github.com/BanjoFox/aardwolf\" class=\"footer_box\">")?;
i18n!(catalog, "Check us out on GitHub!").to_html(&mut out)?;
out.write_all(b" ")?;
icon(&mut out, "github")?;
out.write_all(b"</a>\r\n        <span class=\"vertical_line\" />\r\n        <a href=\"https://www.patreon.com/banjofox\" class=\"footer_box\">")?;
i18n!(catalog, "Buy the team a coffee").to_html(&mut out)?;
out.write_all(b" ")?;
icon(&mut out, "coffee")?;
out.write_all(b"</a>\t\t\r\n    </div>\r\n  </div>\r\n</footer>\r\n")?;
Ok(())
}

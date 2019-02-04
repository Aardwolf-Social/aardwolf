use std::io::{self, Write};
<<<<<<< HEAD
=======
#[allow(renamed_and_removed_lints)]
>>>>>>> 4171ff0286ed26c81a011663cd6f23b09128cf9f
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use gettext::Catalog;
<<<<<<< HEAD
use rocket_i18n::i18n;
=======
use gettext_macros::i18n;
>>>>>>> 4171ff0286ed26c81a011663cd6f23b09128cf9f
use crate::templates::widgets::icon;

pub fn footer(out: &mut Write, catalog: &Catalog) -> io::Result<()> {
out.write_all(b"<footer class=\"footer\">\n  <div class=\"container\">\n    <div class=\"content has-text-centered\">\n        <a href=\"termsofservice.html\" class=\"footer_box\">")?;
i18n!(catalog, "Terms of Service").to_html(out)?;
<<<<<<< HEAD
out.write_all(b"</a>\n        <span class=\"vertical-line\"/>\n        <span class=\"footer-box\">")?;
i18n!(catalog, "Copyright 2018").to_html(out)?;
out.write_all(b"</span>\n        <span class=\"vertical-line\" />\n        <a href=\"https://github.com/BanjoFox/aardwolf\" class=\"footer-box\">")?;
i18n!(catalog, "Check us out on GitHub!").to_html(out)?;
out.write_all(b" ")?;
icon(out, "github")?;
out.write_all(b"</a>\n        <span class=\"vertical-line\" />\n        <a href=\"https://www.patreon.com/banjofox\" class=\"footer-box\">")?;
=======
out.write_all(b"</a>\n        <span class=\"vertical_line\"/>\n        <span class=\"footer_box\">")?;
i18n!(catalog, "Copyright 2018").to_html(out)?;
out.write_all(b"</span>\n        <span class=\"vertical_line\" />\n        <a href=\"https://github.com/BanjoFox/aardwolf\" class=\"footer_box\">")?;
i18n!(catalog, "Check us out on GitHub!").to_html(out)?;
out.write_all(b" ")?;
icon(out, "github")?;
out.write_all(b"</a>\n        <span class=\"vertical_line\" />\n        <a href=\"https://www.patreon.com/banjofox\" class=\"footer_box\">")?;
>>>>>>> 4171ff0286ed26c81a011663cd6f23b09128cf9f
i18n!(catalog, "Buy the team a coffee").to_html(out)?;
out.write_all(b" ")?;
icon(out, "coffee")?;
out.write_all(b"</a>\t\t\n    </div>\n  </div>\n</footer>\n")?;
Ok(())
}

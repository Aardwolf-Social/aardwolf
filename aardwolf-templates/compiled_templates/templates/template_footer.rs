use std::io::{self, Write};
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use ::templates::{Html,ToHtml};
use gettext::Catalog;
use rocket_i18n::i18n;

pub fn footer(out: &mut Write, catalog: &Catalog)
-> io::Result<()> {
write!(out, "<footer class=\"footer\">\n  <div class=\"container\">\n    <div class=\"content has-text-centered\">\n        <a href=\"termsofservice.html\" class=\"footer_box\">")?;
i18n!(catalog, "Terms of Service").to_html(out)?;
write!(out, "</a>\n        <span class=\"vertical_line\"/>\n        <span class=\"footer_box\">")?;
i18n!(catalog, "Copyright 2018").to_html(out)?;
write!(out, "</span>\n        <span class=\"vertical_line\" />\n        <a href=\"https://github.com/BanjoFox/aardwolf\" class=\"footer_box\">")?;
i18n!(catalog, "Check us out on GitHub!").to_html(out)?;
write!(out, "<i class=\"fab fa-github\"></i></a>\n        <span class=\"vertical_line\" />\n        <a href=\"https://www.patreon.com/banjofox\" class=\"footer_box\">")?;
i18n!(catalog, "Buy the team a coffee").to_html(out)?;
write!(out, "<span class=\"fas fa-beer\"aria-hidden=\"true\"></span></a>\t\t\n    </div>\n  </div>\n</footer>\n")?;
Ok(())
}

use std::io::{self, Write};
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use ::templates::{Html,ToHtml};
use gettext::Catalog;
use rocket_i18n::i18n;

pub fn shortcuts(out: &mut Write, catalog: Catalog, profile_link: &str, username: &str)
-> io::Result<()> {
write!(out, "<aside class=\"menu\">\n    <a href=\"")?;
profile_link.to_html(out)?;
write!(out, "\">")?;
username.to_html(out)?;
write!(out, "</a>\n    <ul class=\"menu-list\">\n        <li><a class=\"fa fa-newspaper-o\" aria-hidden=\"true\">")?;
i18n!(catalog, "News feed").to_html(out)?;
write!(out, "</a></li>\n        <li><a class=\"fa fa-envelope\" aria-hidden=\"true\">")?;
i18n!(catalog, "Messages").to_html(out)?;
write!(out, "</a></li>\n    </ul>\n    <p class=\"menu-label\">\n    <span class=\"fa fa-star\" aria-hidden=\"true\">")?;
i18n!(catalog, "Shortcuts").to_html(out)?;
write!(out, "</span>\n    </p>\n    <ul class=\"menu-list\">\n        <li><a class=\"fa fa-calendar\" aria-hidden=\"true\">")?;
i18n!(catalog, "Calendar").to_html(out)?;
write!(out, "</a></li>\n        <li><a class=\"fa fa-users\" aria-hidden=\"true\">")?;
i18n!(catalog, "Groups").to_html(out)?;
write!(out, "</a></li>\n        <li><a class=\"fa fa-list\" aria-hidden=\"true\">")?;
i18n!(catalog, "Lists").to_html(out)?;
write!(out, "</a></li>\n        <li><a class=\"fa fa-picture-o\" aria-hidden=\"true\">")?;
i18n!(catalog, "Photos").to_html(out)?;
write!(out, "</a></li>\n        <li><a class=\"fa fa-bookmark\" aria-hidden=\"true\">")?;
i18n!(catalog, "Favorites").to_html(out)?;
write!(out, "</a></li>\n        <li><a class=\"fa fa-cloud\" aria-hidden=\"true\">")?;
i18n!(catalog, "Weather").to_html(out)?;
write!(out, "</a></li>     \n    </ul>\n    <p class=\"menu-label\">\n    <span class=\"fa fa-lightbulb-o\" aria-hidden=\"true\">")?;
i18n!(catalog, "Create").to_html(out)?;
write!(out, "</span>\n    </p>\n    <ul class=\"menu-list\">\n        <li><a class=\"fa fa-calendar-plus-o\" aria-hidden=\"true\">")?;
i18n!(catalog, "New Event").to_html(out)?;
write!(out, "</a></li>\n        <li><a class=\"fa fa-users\" aria-hidden=\"true\">")?;
i18n!(catalog, "New Group").to_html(out)?;
write!(out, "</a></li>\n    </ul>\n</aside>\n")?;
Ok(())
}

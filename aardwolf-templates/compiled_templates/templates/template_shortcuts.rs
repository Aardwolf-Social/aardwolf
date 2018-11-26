use std::io::{self, Write};
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use ::templates::{Html,ToHtml};
use rocket_i18n::i18n;
use crate::{Shortcuts, templates::ui::icon};

pub fn shortcuts(out: &mut Write, shortcuts: &Shortcuts)
-> io::Result<()> {
write!(out, "<aside class=\"menu\">\n    <a href=\"")?;
shortcuts.profile_link.to_html(out)?;
write!(out, "\">")?;
shortcuts.username.to_html(out)?;
write!(out, "</a>\n    <ul class=\"menu-list\">\n        <li><a aria-hidden=\"true\">")?;
icon(out, "newspaper-o")?;
write!(out, " ")?;
i18n!(shortcuts.catalog, "News feed").to_html(out)?;
write!(out, "</a></li>\n        <li><a aria-hidden=\"true\">")?;
icon(out, "envelope")?;
write!(out, " ")?;
i18n!(shortcuts.catalog, "Messages").to_html(out)?;
write!(out, "</a></li>\n    </ul>\n    <p class=\"menu-label\">\n    <span class=\"fa fa-star\" aria-hidden=\"true\">")?;
icon(out, "star")?;
write!(out, " ")?;
i18n!(shortcuts.catalog, "Shortcuts").to_html(out)?;
write!(out, "</span>\n    </p>\n    <ul class=\"menu-list\">\n        <li><a aria-hidden=\"true\">")?;
icon(out, "calendar")?;
write!(out, " ")?;
i18n!(shortcuts.catalog, "Calendar").to_html(out)?;
write!(out, "</a></li>\n        <li><a aria-hidden=\"true\">")?;
icon(out, "users")?;
write!(out, " ")?;
i18n!(shortcuts.catalog, "Groups").to_html(out)?;
write!(out, "</a></li>\n        <li><a aria-hidden=\"true\">")?;
icon(out, "list")?;
write!(out, " ")?;
i18n!(shortcuts.catalog, "Lists").to_html(out)?;
write!(out, "</a></li>\n        <li><a aria-hidden=\"true\">")?;
icon(out, "picture-o")?;
write!(out, " ")?;
i18n!(shortcuts.catalog, "Photos").to_html(out)?;
write!(out, "</a></li>\n        <li><a aria-hidden=\"true\">")?;
icon(out, "bookmark")?;
write!(out, " ")?;
i18n!(shortcuts.catalog, "Favorites").to_html(out)?;
write!(out, "</a></li>\n        <li><a aria-hidden=\"true\">")?;
icon(out, "cloud")?;
write!(out, " ")?;
i18n!(shortcuts.catalog, "Weather").to_html(out)?;
write!(out, "</a></li>     \n    </ul>\n    <p class=\"menu-label\">\n    <span class=\"fa fa-lightbulb-o\" aria-hidden=\"true\">")?;
icon(out, "lightbulb-o")?;
write!(out, " ")?;
i18n!(shortcuts.catalog, "Create").to_html(out)?;
write!(out, "</span>\n    </p>\n    <ul class=\"menu-list\">\n        <li><a aria-hidden=\"true\">")?;
icon(out, "calendar-plus-o")?;
write!(out, " ")?;
i18n!(shortcuts.catalog, "New Event").to_html(out)?;
write!(out, "</a></li>\n        <li><a aria-hidden=\"true\">")?;
icon(out, "users")?;
write!(out, " ")?;
i18n!(shortcuts.catalog, "New Group").to_html(out)?;
write!(out, "</a></li>\n    </ul>\n</aside>\n")?;
Ok(())
}

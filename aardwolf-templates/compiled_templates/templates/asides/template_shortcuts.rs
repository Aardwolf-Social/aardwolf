use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use gettext_macros::i18n;
use crate::{Shortcuts, templates::widgets::icon};

pub fn shortcuts(out: &mut Write, shortcuts: &Shortcuts) -> io::Result<()> {
out.write_all(b"<aside class=\"menu\">\n    <a href=\"")?;
shortcuts.profile_link.to_html(out)?;
out.write_all(b"\">")?;
shortcuts.username.to_html(out)?;
out.write_all(b"</a>\n    <ul class=\"menu-list\">\n        <li><a aria-hidden=\"true\">")?;
icon(out, "newspaper-o")?;
out.write_all(b" ")?;
i18n!(shortcuts.catalog, "News feed").to_html(out)?;
out.write_all(b"</a></li>\n        <li><a aria-hidden=\"true\">")?;
icon(out, "envelope")?;
out.write_all(b" ")?;
i18n!(shortcuts.catalog, "Messages").to_html(out)?;
out.write_all(b"</a></li>\n    </ul>\n    <p class=\"menu-label\">\n    <span class=\"fa fa-star\" aria-hidden=\"true\">")?;
icon(out, "star")?;
out.write_all(b" ")?;
i18n!(shortcuts.catalog, "Shortcuts").to_html(out)?;
out.write_all(b"</span>\n    </p>\n    <ul class=\"menu-list\">\n        <li><a aria-hidden=\"true\">")?;
icon(out, "calendar")?;
out.write_all(b" ")?;
i18n!(shortcuts.catalog, "Calendar").to_html(out)?;
out.write_all(b"</a></li>\n        <li><a aria-hidden=\"true\">")?;
icon(out, "users")?;
out.write_all(b" ")?;
i18n!(shortcuts.catalog, "Groups").to_html(out)?;
out.write_all(b"</a></li>\n        <li><a aria-hidden=\"true\">")?;
icon(out, "list")?;
out.write_all(b" ")?;
i18n!(shortcuts.catalog, "Lists").to_html(out)?;
out.write_all(b"</a></li>\n        <li><a aria-hidden=\"true\">")?;
icon(out, "picture-o")?;
out.write_all(b" ")?;
i18n!(shortcuts.catalog, "Photos").to_html(out)?;
out.write_all(b"</a></li>\n        <li><a aria-hidden=\"true\">")?;
icon(out, "bookmark")?;
out.write_all(b" ")?;
i18n!(shortcuts.catalog, "Favorites").to_html(out)?;
out.write_all(b"</a></li>\n        <li><a aria-hidden=\"true\">")?;
icon(out, "cloud")?;
out.write_all(b" ")?;
i18n!(shortcuts.catalog, "Weather").to_html(out)?;
out.write_all(b"</a></li>     \n    </ul>\n    <p class=\"menu-label\">\n    <span class=\"fa fa-lightbulb-o\" aria-hidden=\"true\">")?;
icon(out, "lightbulb-o")?;
out.write_all(b" ")?;
i18n!(shortcuts.catalog, "Create").to_html(out)?;
out.write_all(b"</span>\n    </p>\n    <ul class=\"menu-list\">\n        <li><a aria-hidden=\"true\">")?;
icon(out, "calendar-plus-o")?;
out.write_all(b" ")?;
i18n!(shortcuts.catalog, "New Event").to_html(out)?;
out.write_all(b"</a></li>\n        <li><a aria-hidden=\"true\">")?;
icon(out, "users")?;
out.write_all(b" ")?;
i18n!(shortcuts.catalog, "New Group").to_html(out)?;
out.write_all(b"</a></li>\n    </ul>\n</aside>\n")?;
Ok(())
}

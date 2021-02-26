use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use gettext_macros::i18n;
use crate::{Shortcuts, templates::elements::icon};

pub fn aside_shortcuts<W>(mut out: &mut W, shortcuts: &Shortcuts) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
out.write_all(b"<aside class=\"menu\">\n    <a href=\"")?;
shortcuts.profile_link.to_html(&mut out)?;
out.write_all(b"\">")?;
shortcuts.username.to_html(&mut out)?;
out.write_all(b"</a>\n    <ul class=\"menu-list\">\n        <li><a aria-hidden=\"true\">")?;
icon(&mut out, "newspaper-o")?;
out.write_all(b" ")?;
i18n!(shortcuts.catalog, "News feed").to_html(&mut out)?;
out.write_all(b"</a></li>\n        <li><a aria-hidden=\"true\">")?;
icon(&mut out, "envelope")?;
out.write_all(b" ")?;
i18n!(shortcuts.catalog, "Messages").to_html(&mut out)?;
out.write_all(b"</a></li>\n    </ul>\n    <p class=\"menu-label\">\n    <span class=\"fa fa-star\" aria-hidden=\"true\">")?;
icon(&mut out, "star")?;
out.write_all(b" ")?;
i18n!(shortcuts.catalog, "Shortcuts").to_html(&mut out)?;
out.write_all(b"</span>\n    </p>\n    <ul class=\"menu-list\">\n        <li><a aria-hidden=\"true\">")?;
icon(&mut out, "calendar")?;
out.write_all(b" ")?;
i18n!(shortcuts.catalog, "Calendar").to_html(&mut out)?;
out.write_all(b"</a></li>\n        <li><a aria-hidden=\"true\">")?;
icon(&mut out, "users")?;
out.write_all(b" ")?;
i18n!(shortcuts.catalog, "Groups").to_html(&mut out)?;
out.write_all(b"</a></li>\n        <li><a aria-hidden=\"true\">")?;
icon(&mut out, "list")?;
out.write_all(b" ")?;
i18n!(shortcuts.catalog, "Lists").to_html(&mut out)?;
out.write_all(b"</a></li>\n        <li><a aria-hidden=\"true\">")?;
icon(&mut out, "picture-o")?;
out.write_all(b" ")?;
i18n!(shortcuts.catalog, "Photos").to_html(&mut out)?;
out.write_all(b"</a></li>\n        <li><a aria-hidden=\"true\">")?;
icon(&mut out, "bookmark")?;
out.write_all(b" ")?;
i18n!(shortcuts.catalog, "Favorites").to_html(&mut out)?;
out.write_all(b"</a></li>\n        <li><a aria-hidden=\"true\">")?;
icon(&mut out, "cloud")?;
out.write_all(b" ")?;
i18n!(shortcuts.catalog, "Weather").to_html(&mut out)?;
out.write_all(b"</a></li>     \n    </ul>\n    <p class=\"menu-label\">\n    <span class=\"fa fa-lightbulb-o\" aria-hidden=\"true\">")?;
icon(&mut out, "lightbulb-o")?;
out.write_all(b" ")?;
i18n!(shortcuts.catalog, "Create").to_html(&mut out)?;
out.write_all(b"</span>\n    </p>\n    <ul class=\"menu-list\">\n        <li><a aria-hidden=\"true\">")?;
icon(&mut out, "calendar-plus-o")?;
out.write_all(b" ")?;
i18n!(shortcuts.catalog, "New Event").to_html(&mut out)?;
out.write_all(b"</a></li>\n        <li><a aria-hidden=\"true\">")?;
icon(&mut out, "users")?;
out.write_all(b" ")?;
i18n!(shortcuts.catalog, "New Group").to_html(&mut out)?;
out.write_all(b"</a></li>\n    </ul>\n</aside>\n")?;
Ok(())
}

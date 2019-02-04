use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use gettext::Catalog;
use gettext_macros::i18n;

pub fn settings(out: &mut Write, catalog: &Catalog) -> io::Result<()> {
out.write_all(b"<aside class=\"menu\">\n  <p class=\"menu-label\">\n    <a>")?;
icon(out, "chevron-left")?;
i18n!(catalog, "Go back").to_html(out)?;
out.write_all(b"</a>\n  </p>\n  <p class=\"menu-label\">\n    <span>")?;
icon(out, "gears")?;
i18n!(catalog, "Settings").to_html(out)?;
out.write_all(b"\n  </p>\n  <ul class=\"menu-list\">\n    <li><a>")?;
icon(out, "user")?;
i18n!(catalog, "Edit Profile").to_html(out)?;
out.write_all(b"</a></li>\n    <li><a>")?;
icon(out, "sliders")?;
i18n!(catalog, "Preferences").to_html(out)?;
out.write_all(b"</a></li>\n    <li><a>")?;
icon(out, "volume-off")?;
i18n!(catalog, "Muted Keywords").to_html(out)?;
out.write_all(b"</a></li>\n    <li><a>")?;
icon(out, "volume-off")?;
i18n!(catalog, "Muted Users").to_html(out)?;
out.write_all(b"</a></li>\n    <li><a>")?;
icon(out, "bell")?;
i18n!(catalog, "Notifications").to_html(out)?;
out.write_all(b"</a></li>\n    <li><a>")?;
icon(out, "lok")?;
i18n!(catalog, "Security").to_html(out)?;
out.write_all(b"</a></li>\n    <li><a>")?;
icon(out, "cloud-upload")?;
i18n!(catalog, "Data Import").to_html(out)?;
out.write_all(b"</a></li>\n    <li><a>")?;
icon(out, "cloud-download")?;
i18n!(catalog, "Data Export").to_html(out)?;
out.write_all(b"</a></li>\n    <li><a>")?;
icon(out, "list")?;
i18n!(catalog, "Authorized Apps").to_html(out)?;
out.write_all(b"</a></li>\n    <li><a>")?;
icon(out, "users")?;
i18n!(catalog, "Authorized Followers").to_html(out)?;
out.write_all(b"</a></li>\n  </ul>\n  <p class=\"menu-label\">\n    ")?;
icon(out, "logout")?;
out.write_all(b"<a>")?;
i18n!(catalog, "Invite People").to_html(out)?;
out.write_all(b"</a>\n  </p>\n  <p class=\"menu-label\">\n    ")?;
icon(out, "logout")?;
out.write_all(b"<a>")?;
i18n!(catalog, "Logout").to_html(out)?;
out.write_all(b"</a>\n  </p>\n</aside>\n")?;
Ok(())
}

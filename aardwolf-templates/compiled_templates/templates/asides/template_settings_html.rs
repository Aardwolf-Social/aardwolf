use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use gettext::Catalog;
use gettext_macros::i18n;

pub fn settings_html<W>(mut out: &mut W, catalog: &Catalog) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
out.write_all(b"<aside class=\"menu\">\r\n  <p class=\"menu-label\">\r\n    <a class=\"fa fa-chevron-left\">")?;
i18n!(catalog, "Go back").to_html(&mut out)?;
out.write_all(b"</a>\r\n  </p>\r\n  <p class=\"menu-label\">\r\n    <span class=\"fa fa-gears\">")?;
i18n!(catalog, "Settings").to_html(&mut out)?;
out.write_all(b"</span>\r\n  </p>\r\n  <ul class=\"menu-list\">\r\n    <li><a class=\"fa fa-user\">")?;
i18n!(catalog, "Edit Profile").to_html(&mut out)?;
out.write_all(b"</a></li>\r\n    <li><a class=\"fa fa-sliders\">")?;
i18n!(catalog, "Preferences").to_html(&mut out)?;
out.write_all(b"</a></li>\r\n    <li><a class=\"fa fa-volume-off\">")?;
i18n!(catalog, "Muted Keywords").to_html(&mut out)?;
out.write_all(b"</a></li>\r\n    <li><a class=\"fa fa-volume-off\">")?;
i18n!(catalog, "Muted Users").to_html(&mut out)?;
out.write_all(b"</a></li>\r\n    <li><a class=\"fa fa-bell\">")?;
i18n!(catalog, "Notifications").to_html(&mut out)?;
out.write_all(b"</a></li>\r\n    <li><a class=\"fa fa-lok\">")?;
i18n!(catalog, "Security").to_html(&mut out)?;
out.write_all(b"</a></li>\r\n    <li><a class=\"fa fa-cloud-upload\">")?;
i18n!(catalog, "Data Import").to_html(&mut out)?;
out.write_all(b"</a></li>\r\n    <li><a class=\"fa fa-cloud-download\">")?;
i18n!(catalog, "Data Export").to_html(&mut out)?;
out.write_all(b"</a></li>\r\n    <li><a class=\"fa fa-list\">")?;
i18n!(catalog, "Authorized Apps").to_html(&mut out)?;
out.write_all(b"</a></li>\r\n    <li><a class=\"fa fa-users\">")?;
i18n!(catalog, "Authorized Followers").to_html(&mut out)?;
out.write_all(b"</a></li>\r\n  </ul>\r\n  <p class=\"menu-label\">\r\n    <a class=\"fa fa-user-plus\">")?;
i18n!(catalog, "Invite People").to_html(&mut out)?;
out.write_all(b"</a>\r\n  </p>\r\n  <p class=\"menu-label\">\r\n    <a class=\"fa fa-logout\">")?;
i18n!(catalog, "Logout").to_html(&mut out)?;
out.write_all(b"</a>\r\n  </p>\r\n</aside>\r\n")?;
Ok(())
}

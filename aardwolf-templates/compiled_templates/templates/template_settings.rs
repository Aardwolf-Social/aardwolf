use std::io::{self, Write};
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use ::templates::{Html,ToHtml};
use gettext::Catalog;
use rocket_i18n::i18n;

pub fn settings(out: &mut Write, catalog: &Catalog)
-> io::Result<()> {
write!(out, "<aside class=\"menu\">\n  <p class=\"menu-label\">\n    <a class=\"fa fa-chevron-left\">")?;
i18n!(catalog, "Go back").to_html(out)?;
write!(out, "</a>\n  </p>\n  <p class=\"menu-label\">\n    <span class=\"fa fa-gears\">")?;
i18n!(catalog, "Settings").to_html(out)?;
write!(out, "</span>\n  </p>\n  <ul class=\"menu-list\">\n    <li><a class=\"fa fa-user\">")?;
i18n!(catalog, "Edit Profile").to_html(out)?;
write!(out, "</a></li>\n    <li><a class=\"fa fa-sliders\">")?;
i18n!(catalog, "Preferences").to_html(out)?;
write!(out, "</a></li>\n    <li><a class=\"fa fa-volume-off\">")?;
i18n!(catalog, "Muted Keywords").to_html(out)?;
write!(out, "</a></li>\n    <li><a class=\"fa fa-volume-off\">")?;
i18n!(catalog, "Muted Users").to_html(out)?;
write!(out, "</a></li>\n    <li><a class=\"fa fa-bell\">")?;
i18n!(catalog, "Notifications").to_html(out)?;
write!(out, "</a></li>\n    <li><a class=\"fa fa-lok\">")?;
i18n!(catalog, "Security").to_html(out)?;
write!(out, "</a></li>\n    <li><a class=\"fa fa-cloud-upload\">")?;
i18n!(catalog, "Data Import").to_html(out)?;
write!(out, "</a></li>\n    <li><a class=\"fa fa-cloud-download\">")?;
i18n!(catalog, "Data Export").to_html(out)?;
write!(out, "</a></li>\n    <li><a class=\"fa fa-list\">")?;
i18n!(catalog, "Authorized Apps").to_html(out)?;
write!(out, "</a></li>\n    <li><a class=\"fa fa-users\">")?;
i18n!(catalog, "Authorized Followers").to_html(out)?;
write!(out, "</a></li>\n  </ul>\n  <p class=\"menu-label\">\n    <a class=\"fa fa-user-plus\">")?;
i18n!(catalog, "Invite People").to_html(out)?;
write!(out, "</a>\n  </p>\n  <p class=\"menu-label\">\n    <a class=\"fa fa-logout\">")?;
i18n!(catalog, "Logout").to_html(out)?;
write!(out, "</a>\n  </p>\n</aside>\n")?;
Ok(())
}

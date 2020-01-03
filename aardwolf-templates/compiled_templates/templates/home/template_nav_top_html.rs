use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use gettext::Catalog;
use gettext_macros::i18n;

pub fn nav_top_html<W>(mut out: &mut W, catalog: &Catalog) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
out.write_all(b"<nav class=\"navbar\" role=\"navigation\" aria-label=\"main navigation\">\r\n    <div class=\"navbar-brand\">\r\n      <a class=\"navbar-item\">\r\n            <strong>INSTANCE TITLE</strong>\r\n      </a>  \r\n      <a class=\"navbar-item\" href=\"https://aardwolf.social\">\r\n             <img src=\"/web/images/aardwolf-logo.png\" height=\"100\" alt=\"Aardwolf\">\r\n      </a>\r\n      <a role=\"button\" class=\"navbar-burger burger\" aria-label=\"menu\" aria-expanded=\"false\" data-target=\"navbarBasicExample\">\r\n        <span aria-hidden=\"true\"></span>\r\n        <span aria-hidden=\"true\"></span>\r\n        <span aria-hidden=\"true\"></span>\r\n      </a>\r\n      <a class=\"navbar-item\">\r\n            <strong>USERNAME</strong>\r\n      </a>  \t\r\n    </div><!-- /navbar-brand -->\r\n  \r\n    <div id=\"navbarBasicExample\" class=\"navbar-menu\">\r\n      <div class=\"navbar-end\">\r\n          <!-- Nav Right-hand Menu -->\t\r\n          <a class=\"navbar-item\">\r\n            ")?;
i18n!(catalog, "Home").to_html(&mut out)?;
out.write_all(b"\r\n          </a>\r\n          <a ")?;
click.to_html(&mut out)?;
out.write_all(b"=\"toggleNotifCompView\" class=\"navbar-item\" href=\"#\">\r\n            ")?;
i18n!(catalog, "Notifications").to_html(&mut out)?;
out.write_all(b"\r\n          </a>\r\n          <notification-box v-if=\"showNotificationBox\"></notification-box>\r\n        <!-- Nav Right-hand Dropdown -->\r\n        <div class=\"navbar-item has-dropdown is-hoverable\">\r\n          <a class=\"navbar-link\">\r\n            More\r\n          </a>\r\n            <div class=\"navbar-dropdown\">\r\n              <a class=\"navbar-item\">\r\n                About\r\n              </a>\r\n              <a class=\"navbar-item\" href=\"/templates/asides/aside_settings.html\">\r\n                Settings\r\n              </a>\r\n              <a class=\"navbar-item\">\r\n                Contact\r\n              </a>\r\n              <hr class=\"navbar-divider\">\r\n              <a class=\"navbar-item\">\r\n                Report an issue\r\n              </a>\r\n            </div>\r\n        </div>\r\n        <!-- /Nav Dropdown -->\r\n        <!-- Nav Login/Logout Buttons -->\r\n        <div class=\"navbar-item\">\r\n          <div class=\"buttons\">\r\n            <a class=\"button is-light\">\r\n              ")?;
i18n!(catalog, "Logout").to_html(&mut out)?;
out.write_all(b"\r\n            </a>\r\n          </div>\r\n        </div>\r\n        <!-- /Nav Login/Logout Buttons -->\t  \r\n      </div><!-- /navbar-end -->\r\n    </div><!-- /navbarBasicExample -->\r\n  </nav>\r\n")?;
Ok(())
}

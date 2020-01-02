use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use gettext::Catalog;
use gettext_macros::i18n;

pub fn home_nav_top_html<W>(mut out: &mut W, catalog: &Catalog) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
out.write_all(b"<nav class=\"navbar\" role=\"navigation\" aria-label=\"main navigation\">\n    <div class=\"navbar-brand\">\n      <a class=\"navbar-item\">\n            <strong>INSTANCE TITLE</strong>\n      </a>  \n      <a class=\"navbar-item\" href=\"https://aardwolf.social\">\n             <img src=\"/web/images/aardwolf-logo.png\" height=\"100\" alt=\"Aardwolf\">\n      </a>\n      <a role=\"button\" class=\"navbar-burger burger\" aria-label=\"menu\" aria-expanded=\"false\" data-target=\"navbarBasicExample\">\n        <span aria-hidden=\"true\"></span>\n        <span aria-hidden=\"true\"></span>\n        <span aria-hidden=\"true\"></span>\n      </a>\n      <a class=\"navbar-item\">\n            <strong>USERNAME</strong>\n      </a>  \t\n    </div><!-- /navbar-brand -->\n  \n    <div id=\"navbarBasicExample\" class=\"navbar-menu\">\n      <div class=\"navbar-end\">\n          <!-- Nav Right-hand Menu -->\t\n          <a class=\"navbar-item\">\n            ")?;
i18n!(catalog, "Home").to_html(&mut out)?;
out.write_all(b"\n          </a>\n          <a ")?;
click.to_html(&mut out)?;
out.write_all(b"=\"toggleNotifCompView\" class=\"navbar-item\" href=\"#\">\n            ")?;
i18n!(catalog, "Notifications").to_html(&mut out)?;
out.write_all(b"\n          </a>\n          <notification-box v-if=\"showNotificationBox\"></notification-box>\n        <!-- Nav Right-hand Dropdown -->\n        <div class=\"navbar-item has-dropdown is-hoverable\">\n          <a class=\"navbar-link\">\n            More\n          </a>\n            <div class=\"navbar-dropdown\">\n              <a class=\"navbar-item\">\n                About\n              </a>\n              <a class=\"navbar-item\" href=\"/templates/asides/aside_settings.html\">\n                Settings\n              </a>\n              <a class=\"navbar-item\">\n                Contact\n              </a>\n              <hr class=\"navbar-divider\">\n              <a class=\"navbar-item\">\n                Report an issue\n              </a>\n            </div>\n        </div>\n        <!-- /Nav Dropdown -->\n        <!-- Nav Login/Logout Buttons -->\n        <div class=\"navbar-item\">\n          <div class=\"buttons\">\n            <a class=\"button is-light\">\n              ")?;
i18n!(catalog, "Logout").to_html(&mut out)?;
out.write_all(b"\n            </a>\n          </div>\n        </div>\n        <!-- /Nav Login/Logout Buttons -->\t  \n      </div><!-- /navbar-end -->\n    </div><!-- /navbarBasicExample -->\n  </nav>\n")?;
Ok(())
}

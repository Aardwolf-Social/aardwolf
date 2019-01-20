use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use gettext::Catalog;
use gettext_macros::i18n;

pub fn nav(out: &mut Write, catalog: &Catalog) -> io::Result<()> {
out.write_all(b"<nav class=\"navbar\">\n    <div class=\"container\">\n        <div class=\"navbar-brand\">\n            <a class=\"navbar-item\">\n                <img src=\"/images/aardwolf-logo.png\" alt=\"Aardwolf\">\n            </a>\n            <span class=\"navbar-burger burger\" data-target=\"navbar_menu_hero_a\">\n                <span></span>\n                <span></span>\n                <span></span>\n            </span>\n        </div>\n        <div id=\"navbar_menu_hero_a\" class=\"navbar-menu\">\n            <div class=\"navbar-end\">\n                <a class=\"navbar-item is-active\">\n                    ")?;
i18n!(catalog, "Home").to_html(out)?;
out.write_all(b"\n                </a>\n                <a class=\"navbar-item\">\n                    ")?;
i18n!(catalog, "Profile").to_html(out)?;
out.write_all(b"\n                </a>\n                <a class=\"navbar-item\">\n                    ")?;
i18n!(catalog, "Messages").to_html(out)?;
out.write_all(b"\n                </a>\n                <span class=\"navbar-item\">\n                    <a class=\"button is-primary is-inverted\">\n                        <span class=\"icon\">\n                            <i class=\"fa fa-github\"></i>\n                        </span>\n                        <span>")?;
i18n!(catalog, "Download").to_html(out)?;
out.write_all(b"</span>\n                    </a>\n                </span>\n                <span class=\"navbar-item\">\n                    <a class=\"button is-dark\" href=\"/auth/sign_out\">\n                        <span class=\"icon\">\n                            <i class=\"fas fa-logout\"></i>\n                        </span>\n                        ")?;
i18n!(catalog, "Logout").to_html(out)?;
out.write_all(b"\n                    </a>\n                </span>\n            </div>\n        </div>\n    </div>\n</nav>\n")?;
Ok(())
}

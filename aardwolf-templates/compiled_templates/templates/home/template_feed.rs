use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use gettext::Catalog;
use gettext_macros::i18n;

pub fn feed(out: &mut Write, catalog: &Catalog) -> io::Result<()> {
out.write_all(b"<article class=\"media\">\n    <!-- Begin sample conversation -->\n    <figure class=\"media-left\">\n        <p class=\"image is-64x64\">\n        <img src=\"https://bulma.io/images/placeholders/128x128.png\" alt=\"")?;
i18n!(catalog, "Username").to_html(out)?;
out.write_all(b"\">\n        </p>\n    </figure>\n    <div class=\"media-content\">\n        <div class=\"content\">\n            <p>\n            <strong>Barbara Middleton</strong>\n            <br> Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis porta eros lacus, nec ultricies elit blandit non. Suspendisse pellentesque mauris sit amet dolor blandit rutrum. Nunc in tempus turpis.\n            <br>\n            <small><a>")?;
i18n!(catalog, "Like").to_html(out)?;
out.write_all("</a> · <a>".as_bytes())?;
i18n!(catalog, "Reply").to_html(out)?;
out.write_all("</a> · <a>".as_bytes())?;
i18n!(catalog, "Boost").to_html(out)?;
out.write_all("</a> · ".as_bytes())?;
icon(out, "ellipsis-h")?;
out.write_all("</i> · 3 hrs</small>\n\n            </p>\n        </div>\n\n        <article class=\"media\">\n            <figure class=\"media-left\">\n                <p class=\"image is-48x48\">\n                <img src=\"https://bulma.io/images/placeholders/96x96.png\" alt=\"".as_bytes())?;
i18n!(catalog, "Username").to_html(out)?;
out.write_all(b"\">\n                </p>\n            </figure>\n            <div class=\"media-content\">\n                <div class=\"content\">\n                    <p>\n                    <strong>Sean Brown</strong>\n                    <br> Donec sollicitudin urna eget eros malesuada sagittis. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Aliquam blandit nisl a nulla sagittis, a lobortis leo feugiat.\n                    <br>\n                    <small><a>")?;
i18n!(catalog, "Like").to_html(out)?;
out.write_all("</a> · <a>".as_bytes())?;
i18n!(catalog, "Reply").to_html(out)?;
out.write_all("</a> · <a>".as_bytes())?;
i18n!(catalog, "Boost").to_html(out)?;
out.write_all("</a> · ".as_bytes())?;
icon(out, "ellipsis-h")?;
out.write_all("</i> · 2 hrs</small>\n                    </p>\n                </div>\n\n                <article class=\"media\">\n                    Vivamus quis semper metus, non tincidunt dolor. Vivamus in mi eu lorem cursus ullamcorper sit amet nec massa.\n                </article>\n\n                <article class=\"media\">\n                    Morbi vitae diam et purus tincidunt porttitor vel vitae augue. Praesent malesuada metus sed pharetra euismod. Cras tellus odio, tincidunt iaculis diam non, porta aliquet tortor.\n                </article>\n            </div>\n        </article>\n\n        <article class=\"media\">\n            <figure class=\"media-left\">\n                <p class=\"image is-48x48\">\n                <img src=\"https://bulma.io/images/placeholders/96x96.png\" alt=\"".as_bytes())?;
i18n!(catalog, "Username").to_html(out)?;
out.write_all(b"\">\n                </p>\n            </figure>\n            <div class=\"media-content\">\n                <div class=\"content\">\n                    <p>\n                    <strong>Kayli Eunice </strong>\n                    <br> Sed convallis scelerisque mauris, non pulvinar nunc mattis vel. Maecenas varius felis sit amet magna vestibulum euismod malesuada cursus libero. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia\n                    Curae; Phasellus lacinia non nisl id feugiat.\n                    <br>\n                    <small><a>")?;
i18n!(catalog, "Like").to_html(out)?;
out.write_all("</a> · <a>".as_bytes())?;
i18n!(catalog, "Reply").to_html(out)?;
out.write_all("</a> · <a>".as_bytes())?;
i18n!(catalog, "Boost").to_html(out)?;
out.write_all("</a> · ".as_bytes())?;
icon(out, "ellipsis-h")?;
out.write_all("</i> · 2 hrs</small>\n                    </p>\n                </div>\n            </div>\n        </article>\n    </div>\n</article>\n<!-- End sample conversation -->  \n".as_bytes())?;
Ok(())
}

use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use gettext::Catalog;
use gettext_macros::i18n;

pub fn feed_html<W>(mut out: &mut W, catalog: &Catalog) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
out.write_all(b"<article class=\"media\">\r\n    <!-- Begin sample conversation -->\r\n    <figure class=\"media-left\">\r\n        <p class=\"image is-64x64\">\r\n        <img src=\"https://bulma.io/images/placeholders/128x128.png\" alt=\"")?;
i18n!(catalog, "Username").to_html(&mut out)?;
out.write_all(b"\">\r\n        </p>\r\n    </figure>\r\n    <div class=\"media-content\">\r\n        <div class=\"content\">\r\n            <p>\r\n            <strong>Barbara Middleton</strong>\r\n            <br> Lorem ipsum dolor sit amet, consectetur adipiscing elit. Duis porta eros lacus, nec ultricies elit blandit non. Suspendisse pellentesque mauris sit amet dolor blandit rutrum. Nunc in tempus turpis.\r\n            <br>\r\n            <small><a>")?;
i18n!(catalog, "Like").to_html(&mut out)?;
out.write_all("</a> · <a>".as_bytes())?;
i18n!(catalog, "Reply").to_html(&mut out)?;
out.write_all("</a> · <a>".as_bytes())?;
i18n!(catalog, "Boost").to_html(&mut out)?;
out.write_all("</a> · <span class=\"fa fa-ellipsis-h\" aria-hidden=\"true\"></i> · 3 hrs</small>\r\n            </p>\r\n        </div>\r\n\r\n        <article class=\"media\">\r\n            <figure class=\"media-left\">\r\n                <p class=\"image is-48x48\">\r\n                <img src=\"https://bulma.io/images/placeholders/96x96.png\" alt=\"".as_bytes())?;
i18n!(catalog, "Username").to_html(&mut out)?;
out.write_all(b"\">\r\n                </p>\r\n            </figure>\r\n            <div class=\"media-content\">\r\n                <div class=\"content\">\r\n                    <p>\r\n                    <strong>Sean Brown</strong>\r\n                    <br> Donec sollicitudin urna eget eros malesuada sagittis. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Aliquam blandit nisl a nulla sagittis, a lobortis leo feugiat.\r\n                    <br>\r\n                    <small><a>")?;
i18n!(catalog, "Like").to_html(&mut out)?;
out.write_all("</a> · <a>".as_bytes())?;
i18n!(catalog, "Reply").to_html(&mut out)?;
out.write_all("</a> · <a>".as_bytes())?;
i18n!(catalog, "Boost").to_html(&mut out)?;
out.write_all("</a> · <span class=\"fa fa-ellipsis-h\" aria-hidden=\"true\"></i> · 2 hrs</small>\r\n                    </p>\r\n                </div>\r\n\r\n                <article class=\"media\">\r\n                    Vivamus quis semper metus, non tincidunt dolor. Vivamus in mi eu lorem cursus ullamcorper sit amet nec massa.\r\n                </article>\r\n\r\n                <article class=\"media\">\r\n                    Morbi vitae diam et purus tincidunt porttitor vel vitae augue. Praesent malesuada metus sed pharetra euismod. Cras tellus odio, tincidunt iaculis diam non, porta aliquet tortor.\r\n                </article>\r\n            </div>\r\n        </article>\r\n\r\n        <article class=\"media\">\r\n            <figure class=\"media-left\">\r\n                <p class=\"image is-48x48\">\r\n                <img src=\"https://bulma.io/images/placeholders/96x96.png\" alt=\"".as_bytes())?;
i18n!(catalog, "Username").to_html(&mut out)?;
out.write_all(b"\">\r\n                </p>\r\n            </figure>\r\n            <div class=\"media-content\">\r\n                <div class=\"content\">\r\n                    <p>\r\n                    <strong>Kayli Eunice </strong>\r\n                    <br> Sed convallis scelerisque mauris, non pulvinar nunc mattis vel. Maecenas varius felis sit amet magna vestibulum euismod malesuada cursus libero. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia\r\n                    Curae; Phasellus lacinia non nisl id feugiat.\r\n                    <br>\r\n                    <small><a>")?;
i18n!(catalog, "Like").to_html(&mut out)?;
out.write_all("</a> · <a>".as_bytes())?;
i18n!(catalog, "Reply").to_html(&mut out)?;
out.write_all("</a> · <a>".as_bytes())?;
i18n!(catalog, "Boost").to_html(&mut out)?;
out.write_all("</a> · <span class=\"fa fa-ellipsis-h\" aria-hidden=\"true\"></i> · 2 hrs</small>\r\n                    </p>\r\n                </div>\r\n            </div>\r\n        </article>\r\n    </div>\r\n</article>\r\n<!-- End sample conversation -->  \r\n".as_bytes())?;
Ok(())
}

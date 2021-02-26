use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use gettext::Catalog;
use gettext_macros::i18n;

pub fn profile_edit_html<W>(mut out: &mut W, catalog: &Catalog) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
out.write_all(b"<!-- Begin Profile Section -->\r\n<section class=\"section\">\r\n    <div class=\"container\">\r\n\r\n        <title>")?;
i18n!(catalog, "profile title").to_html(&mut out)?;
out.write_all(b"</title>\r\n\r\n        <!-- section title -->\r\n        <h1 class=\"title\">")?;
i18n!(catalog, "profile title").to_html(&mut out)?;
out.write_all(b"</h1>\r\n        <h2 class=\"subtitle\">")?;
i18n!(catalog, "profile sub title").to_html(&mut out)?;
out.write_all(b"</h2>\r\n\r\n        <!-- Profile display name -->\r\n\t\t<div class=\"field box\">\r\n\t\t  <label class=\"label\">")?;
i18n!(catalog, "profile display name").to_html(&mut out)?;
out.write_all(b"</label>\r\n\t\t  <div class=\"control\">\r\n\t\t\t<input class=\"input\" type=\"text\" placeholder=\"")?;
i18n!(catalog, "profile display name").to_html(&mut out)?;
out.write_all(b"\">\r\n\t\t  </div>\r\n\t\t  <p>30 ")?;
i18n!(catalog, "profile char remaining").to_html(&mut out)?;
out.write_all(b"</p><!-- temporary placeholder -->\r\n\t\t</div>\r\n\r\n        <!-- Profile bio -->\r\n\t\t<div class=\"field box\">\r\n\t\t  <label class=\"label\">")?;
i18n!(catalog, "profile bio").to_html(&mut out)?;
out.write_all(b"</label>\r\n\t\t  <div class=\"control\">\r\n\t\t\t<textarea class=\"textarea\" placeholder=\"")?;
i18n!(catalog, "profile bio").to_html(&mut out)?;
out.write_all(b"\"></textarea>\r\n\t\t  </div>\r\n\t\t  <p>500 ")?;
i18n!(catalog, "profile char remaining").to_html(&mut out)?;
out.write_all(b"</p><!-- temporary placeholder -->\r\n\t\t</div>\r\n\r\n        <!-- Profile Avatar -->\r\n\t\t<div class=\"box\">\r\n\t\t  <label class=\"label\">")?;
i18n!(catalog, "profile avatar upload").to_html(&mut out)?;
out.write_all(b"</label>\r\n\t\t<figure class=\"image is-128x128\">\r\n\t\t  <img src=\"https://bulma.io/images/placeholders/128x128.png\">\r\n\t\t</figure>\r\n\t\t<br />\r\n\r\n\t\t<!-- Avatar upload controls -->\r\n\t\t<div class=\"file has-name\">\r\n\t\t  <label class=\"file-label\">\r\n\t\t\t<input class=\"file-input\" type=\"file\" name=\"resume\">\r\n\t\t\t<span class=\"file-cta\">\r\n\t\t\t  <span class=\"file-icon\">\r\n\t\t\t\t<i class=\"fa fa-upload\"></i>\r\n\t\t\t  </span>\r\n\t\t\t  <span class=\"file-label\">\r\n\t\t\t\t")?;
i18n!(catalog, "profile avatar upload label").to_html(&mut out)?;
out.write_all(b"\r\n\t\t\t  </span>\r\n\t\t\t</span>\r\n\t\t\t<span class=\"file-name\">\r\n\t\t\t  ")?;
i18n!(catalog, "profile avatar file name").to_html(&mut out)?;
out.write_all(b"\r\n\t\t\t</span>\r\n\t\t  </label>\r\n\t\t</div>\r\n\t\t<p>")?;
i18n!(catalog, "profile avatar sub text").to_html(&mut out)?;
out.write_all(b"</p>\r\n\t\t</div>\r\n\r\n        <!-- Profile Header -->\r\n\t\t<div class=\"box\">\r\n\t\t  <label class=\"label\">")?;
i18n!(catalog, "profile header upload").to_html(&mut out)?;
out.write_all(b"</label>\r\n\t\t<figure class=\"image is-128x128\">\r\n\t\t  <img src=\"https://bulma.io/images/placeholders/128x128.png\">\r\n\t\t</figure>\r\n\t\t<br />\r\n\r\n\t\t<!-- Header upload controls -->\r\n\t\t<div class=\"file has-name\">\r\n\t\t  <label class=\"file-label\">\r\n\t\t\t<input class=\"file-input\" type=\"file\" name=\"resume\">\r\n\t\t\t<span class=\"file-cta\">\r\n\t\t\t  <span class=\"file-icon\">\r\n\t\t\t\t<i class=\"fa fa-upload\"></i>\r\n\t\t\t  </span>\r\n\t\t\t  <span class=\"file-label\">\r\n\t\t\t\t")?;
i18n!(catalog, "profile avatar header label").to_html(&mut out)?;
out.write_all(b"\r\n\t\t\t  </span>\r\n\t\t\t</span>\r\n\t\t\t<span class=\"file-name\">\r\n\t\t\t  ")?;
i18n!(catalog, "profile header file name").to_html(&mut out)?;
out.write_all(b"\r\n\t\t\t</span>\r\n\t\t  </label>\r\n\t\t</div>\r\n\t\t<p>")?;
i18n!(catalog, "profile header sub text").to_html(&mut out)?;
out.write_all(b"</p><!-- temporary placeholder -->\r\n\t\t</div>\r\n\r\n    </div>\r\n</section>\r\n<!-- End Profile Section -->\r\n\r\n")?;
Ok(())
}

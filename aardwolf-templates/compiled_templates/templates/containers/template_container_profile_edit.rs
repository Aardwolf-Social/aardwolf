use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use gettext::Catalog;
use gettext_macros::i18n;

pub fn container_profile_edit<W>(mut out: &mut W, catalog: &Catalog) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
out.write_all(b"<!-- Begin Profile Section -->\n<section class=\"section\">\n    <div class=\"container\">\n\n        <title>")?;
i18n!(catalog, "profile title").to_html(&mut out)?;
out.write_all(b"</title>\n\n        <!-- section title -->\n        <h1 class=\"title\">")?;
i18n!(catalog, "profile title").to_html(&mut out)?;
out.write_all(b"</h1>\n        <h2 class=\"subtitle\">")?;
i18n!(catalog, "profile sub title").to_html(&mut out)?;
out.write_all(b"</h2>\n\n        <!-- Profile display name -->\n\t\t<div class=\"field box\">\n\t\t  <label class=\"label\">")?;
i18n!(catalog, "profile display name").to_html(&mut out)?;
out.write_all(b"</label>\n\t\t  <div class=\"control\">\n\t\t\t<input class=\"input\" type=\"text\" placeholder=\"")?;
i18n!(catalog, "profile display name").to_html(&mut out)?;
out.write_all(b"\">\n\t\t  </div>\n\t\t  <p>30 ")?;
i18n!(catalog, "profile char remaining").to_html(&mut out)?;
out.write_all(b"</p><!-- temporary placeholder -->\n\t\t</div>\n\n        <!-- Profile bio -->\n\t\t<div class=\"field box\">\n\t\t  <label class=\"label\">")?;
i18n!(catalog, "profile bio").to_html(&mut out)?;
out.write_all(b"</label>\n\t\t  <div class=\"control\">\n\t\t\t<textarea class=\"textarea\" placeholder=\"")?;
i18n!(catalog, "profile bio").to_html(&mut out)?;
out.write_all(b"\"></textarea>\n\t\t  </div>\n\t\t  <p>500 ")?;
i18n!(catalog, "profile char remaining").to_html(&mut out)?;
out.write_all(b"</p><!-- temporary placeholder -->\n\t\t</div>\n\n        <!-- Profile Avatar -->\n\t\t<div class=\"box\">\n\t\t  <label class=\"label\">")?;
i18n!(catalog, "profile avatar upload").to_html(&mut out)?;
out.write_all(b"</label>\n\t\t<figure class=\"image is-128x128\">\n\t\t  <img src=\"https://bulma.io/images/placeholders/128x128.png\">\n\t\t</figure>\n\t\t<br />\n\n\t\t<!-- Avatar upload controls -->\n\t\t<div class=\"file has-name\">\n\t\t  <label class=\"file-label\">\n\t\t\t<input class=\"file-input\" type=\"file\" name=\"resume\">\n\t\t\t<span class=\"file-cta\">\n\t\t\t  <span class=\"file-icon\">\n\t\t\t\t<i class=\"fa fa-upload\"></i>\n\t\t\t  </span>\n\t\t\t  <span class=\"file-label\">\n\t\t\t\t")?;
i18n!(catalog, "profile avatar upload label").to_html(&mut out)?;
out.write_all(b"\n\t\t\t  </span>\n\t\t\t</span>\n\t\t\t<span class=\"file-name\">\n\t\t\t  ")?;
i18n!(catalog, "profile avatar file name").to_html(&mut out)?;
out.write_all(b"\n\t\t\t</span>\n\t\t  </label>\n\t\t</div>\n\t\t<p>")?;
i18n!(catalog, "profile avatar sub text").to_html(&mut out)?;
out.write_all(b"</p>\n\t\t</div>\n\n        <!-- Profile Header -->\n\t\t<div class=\"box\">\n\t\t  <label class=\"label\">")?;
i18n!(catalog, "profile header upload").to_html(&mut out)?;
out.write_all(b"</label>\n\t\t<figure class=\"image is-128x128\">\n\t\t  <img src=\"https://bulma.io/images/placeholders/128x128.png\">\n\t\t</figure>\n\t\t<br />\n\n\t\t<!-- Header upload controls -->\n\t\t<div class=\"file has-name\">\n\t\t  <label class=\"file-label\">\n\t\t\t<input class=\"file-input\" type=\"file\" name=\"resume\">\n\t\t\t<span class=\"file-cta\">\n\t\t\t  <span class=\"file-icon\">\n\t\t\t\t<i class=\"fa fa-upload\"></i>\n\t\t\t  </span>\n\t\t\t  <span class=\"file-label\">\n\t\t\t\t")?;
i18n!(catalog, "profile avatar header label").to_html(&mut out)?;
out.write_all(b"\n\t\t\t  </span>\n\t\t\t</span>\n\t\t\t<span class=\"file-name\">\n\t\t\t  ")?;
i18n!(catalog, "profile header file name").to_html(&mut out)?;
out.write_all(b"\n\t\t\t</span>\n\t\t  </label>\n\t\t</div>\n\t\t<p>")?;
i18n!(catalog, "profile header sub text").to_html(&mut out)?;
out.write_all(b"</p><!-- temporary placeholder -->\n\t\t</div>\n\n    </div>\n</section>\n<!-- End Profile Section -->\n\n")?;
Ok(())
}

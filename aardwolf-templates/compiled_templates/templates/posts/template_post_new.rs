use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use gettext::Catalog;
use gettext_macros::i18n;
use crate::{PostNew, templates::elements::{alert, input_select, input_text, input_textarea}};

pub fn post_new<W>(mut out: &mut W, catalog: &Catalog, username: &str, post_new: &PostNew) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
out.write_all(b"<article class=\"media\"><!-- Begin new post -->\n    <figure class=\"media-left\">\n        <p class=\"image is-64x64\">\n        <img src=\"https://bulma.io/images/placeholders/128x128.png\" alt=\"")?;
username.to_html(&mut out)?;
out.write_all(b"\">\n        </p>\n    </figure>\n    <div class=\"media-content\">\n        <form method=\"POST\" action=\"/posts/create\">\n            ")?;
if let Some(ref a) = post_new.alert {
out.write_all(b"\n                ")?;
alert(&mut out, a)?;
out.write_all(b"\n            ")?;
}
out.write_all(b"\n            <input type=\"hidden\" name=\"csrf_token\" value=\"")?;
new_post.csrf.to_html(&mut out)?;
out.write_all(b"\">\n            ")?;
input_textarea(&mut out, &post_new.source)?;
out.write_all(b"\n            ")?;
input_select(&mut out, &post_new.visibility)?;
out.write_all(b"\n            ")?;
input_text(&mut out, &post_new.name)?;
out.write_all(b"\n            <button>")?;
i18n!(catalog, "Awoo!").to_html(&mut out)?;
out.write_all(b"</button>\n        </form>\n    </div>\n</article><!-- End of new post -->\n")?;
Ok(())
}

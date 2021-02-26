use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use gettext::Catalog;
use gettext_macros::i18n;
use crate::{PostNew, templates::elements::{alert, input_select, input_text, input_textarea}};

pub fn new_html<W>(mut out: &mut W, catalog: &Catalog, username: &str, post_new: &PostNew) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
out.write_all(b"<article class=\"media\"><!-- Begin new post -->\r\n    <figure class=\"media-left\">\r\n        <p class=\"image is-64x64\">\r\n        <img src=\"https://bulma.io/images/placeholders/128x128.png\" alt=\"")?;
username.to_html(&mut out)?;
out.write_all(b"\">\r\n        </p>\r\n    </figure>\r\n    <div class=\"media-content\">\r\n        <form method=\"POST\" action=\"/posts/create\">\r\n            ")?;
if let Some(ref a) = post_new.alert {
out.write_all(b"\r\n                ")?;
alert(&mut out, a)?;
out.write_all(b"\r\n            ")?;
}
out.write_all(b"\r\n            <input type=\"hidden\" name=\"csrf_token\" value=\"")?;
new_post.csrf.to_html(&mut out)?;
out.write_all(b"\">\r\n            ")?;
input_textarea(&mut out, &post_new.source)?;
out.write_all(b"\r\n            ")?;
input_select(&mut out, &post_new.visibility)?;
out.write_all(b"\r\n            ")?;
input_text(&mut out, &post_new.name)?;
out.write_all(b"\r\n            <button>")?;
i18n!(catalog, "Awoo!").to_html(&mut out)?;
out.write_all(b"</button>\r\n        </form>\r\n    </div>\r\n</article><!-- End of new post -->\r\n")?;
Ok(())
}

use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use gettext::Catalog;
use gettext_macros::i18n;
use crate::templates::{posts::{reply_post}, elements::{alert, input_select, input_text, input_textarea}};

pub fn reply_post_html<W>(mut out: &mut W, catalog: &Catalog, username: &str, reply_post: &PostNew) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
out.write_all(b"<article class=\"media\"><!-- Begin reply box -->\r\n  <figure class=\"media-left\">\r\n\t<p class=\"image is-64x64\">\r\n\t  <img src=\"https://bulma.io/images/placeholders/128x128.png\" alt=\"")?;
user_name.to_html(&mut out)?;
out.write_all(b"\">\r\n\t</p>\r\n  </figure>\r\n        <form method=\"POST\" action=\"/posts/create\">\r\n            ")?;
if let Some(ref a) = reply_post.alert {
out.write_all(b"\r\n                ")?;
alert(&mut out, a)?;
out.write_all(b"\r\n            ")?;
}
out.write_all(b"\r\n            <input type=\"hidden\" name=\"csrf_token\" value=\"")?;
new_post.csrf.to_html(&mut out)?;
out.write_all(b"\">\r\n            ")?;
input_textarea(&mut out, &reply_post.source)?;
out.write_all(b"\r\n            ")?;
input_select(&mut out, &reply_post.visibility)?;
out.write_all(b"\r\n            ")?;
input_text(&mut out, &reply_post.name)?;
out.write_all(b"\r\n            <button>")?;
i18n!(catalog, "Wooa!").to_html(&mut out)?;
out.write_all(b"</button>\r\n        </form>\r\n  </div>\r\n</article><!-- End reply box -->\r\n")?;
Ok(())
}

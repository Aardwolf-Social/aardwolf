use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use gettext::Catalog;
use gettext_macros::i18n;
use crate::{NewPost, templates::ui::{alert, select_input, text_input, textarea_input}};

pub fn new_post<W>(mut out: &mut W, catalog: &Catalog, username: &str, new_post: &NewPost) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
out.write_all(b"<article class=\"media\"><!-- Begin new post -->\n    <figure class=\"media-left\">\n        <p class=\"image is-64x64\">\n        <img src=\"https://bulma.io/images/placeholders/128x128.png\" alt=\"")?;
username.to_html(&mut out)?;
out.write_all(b"\">\n        </p>\n    </figure>\n    <div class=\"media-content\">\n        <form method=\"POST\" action=\"/posts/create\">\n            ")?;
if let Some(ref a) = new_post.alert {
out.write_all(b"\n                ")?;
alert(&mut out, a)?;
out.write_all(b"\n            ")?;
}
out.write_all(b"\n            <input type=\"hidden\" name=\"csrf_token\" value=\"")?;
new_post.csrf.to_html(&mut out)?;
out.write_all(b"\">\n            ")?;
textarea_input(&mut out, &new_post.source)?;
out.write_all(b"\n            ")?;
select_input(&mut out, &new_post.visibility)?;
out.write_all(b"\n            ")?;
text_input(&mut out, &new_post.name)?;
out.write_all(b"\n            <button>")?;
i18n!(catalog, "Awoo!").to_html(&mut out)?;
out.write_all(b"</button>\n        </form>\n    </div>\n</article><!-- End of new post -->\n")?;
Ok(())
}

use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use gettext_macros::i18n;
use crate::templates::{base, elements::{alert, notification_content}};

pub fn notification_content_html<W>(mut out: &mut W, notification_content: &NotificationContent) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
out.write_all(b"<div class=\"box\">\r\n  <article class=\"media\">\r\n\t<div class=\"media-left\">\r\n\t  <figure>\r\n\t\t<img src=\"../web/images/default_avatar.png\" alt=\"Profile Image\" aria-hidden=\"true\">\r\n\t  </figure>\r\n\t</div>\r\n\t<div class=\"media-content\">\r\n\t  <div class=\"content\">\r\n\t\t  <!-- Begin Sample Contents -->\r\n\t\t  <strong>John Smith</strong> <small>")?;
aardwolf.social.to_html(&mut out)?;
out.write_all(b"</small>\r\n\t\t  <br />\r\n\t\t  ")?;
i18n!(catalog, "Favorited your post").to_html(&mut out)?;
out.write_all(b"\r\n\t\t   <br />\r\n\t\t   <small><time datetime=\"2016-1-1\">11:09 PM - 1 Jan 2016</time></small>\r\n\t\t   <!-- End Sample Contents -->\r\n\t  </div>\r\n\t</div>\r\n  </article>\r\n</div>\r\n")?;
Ok(())
}

use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use gettext_macros::i18n;
use crate::{NotificationContent, templates::{base, elements::{alert}}};

pub fn notification_content<W>(mut out: &mut W, notification_content: &NotificationContent) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
out.write_all(b"<div class=\"box\">\n  <article class=\"media\">\n\t<div class=\"media-left\">\n\t  <figure>\n\t\t<img src=\"../web/images/default_avatar.png\" alt=\"Profile Image\" aria-hidden=\"true\">\n\t  </figure>\n\t</div>\n\t<div class=\"media-content\">\n\t  <div class=\"content\">\n\t\t  <!-- Begin Sample Contents -->\n\t\t  <strong>John Smith</strong> <small>")?;
aardwolf.social.to_html(&mut out)?;
out.write_all(b"</small>\n\t\t  <br />\n\t\t  ")?;
i18n!(catalog, "Favorited your post").to_html(&mut out)?;
out.write_all(b"\n\t\t   <br />\n\t\t   <small><time datetime=\"2016-1-1\">11:09 PM - 1 Jan 2016</time></small>\n\t\t   <!-- End Sample Contents -->\n\t  </div>\n\t</div>\n  </article>\n</div>\n")?;
Ok(())
}

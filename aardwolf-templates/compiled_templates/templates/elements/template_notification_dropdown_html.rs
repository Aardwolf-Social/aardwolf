use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use gettext_macros::i18n;
use crate::templates::{base, elements::{alert, notification_dropdown}};

pub fn notification_dropdown_html<W>(mut out: &mut W, notification_dropdown: &NotificationDropdown) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
out.write_all(b"<!-- Top Nav Notifications Dropdown -->\r\n<div class=\"navbar-item has-dropdown is-hoverable\">\r\n\t<a class=\"navbar-link\">")?;
i18n!(catalog, "Notifications").to_html(&mut out)?;
out.write_all(b"</a>\r\n\t<div class=\"navbar-dropdown notifications-dropdown\">\r\n\r\n\t\t<!-- ************************************************* -->\r\n\t\t<!-- A little element to signify no new notifications. -->\r\n\t\t<!-- Commented out to meet wireframe criteria.         -->\r\n\t\t<!-- ************************************************* -->\r\n\t\t<!--\r\n\t\t<div class=\"box no-recent-activity\">\r\n\t\t\tNo recent activity\r\n\t\t</div>\r\n\t\t-->\r\n\r\n\t\t<!-- Begin Sample notifications -->\r\n\t\t<div class=\"box\">\r\n\t\t\t<article class=\"media\">\r\n\t\t\t\t<div class=\"media-left\">\r\n\t\t\t\t\t<figure class=\"image is-64x64\">\r\n\t\t\t\t\t\t<img src=\"https://bulma.io/images/placeholders/128x128.png\" alt=\"Image\">\r\n\t\t\t\t\t</figure>\r\n\t\t\t\t</div>\r\n\t\t\t\t<div class=\"media-content\">\r\n\t\t\t\t\t<div class=\"content\">\r\n\t\t\t\t\t\t<p><strong>Juan Quixote</strong> <a href=\"#link-to-user-account\">")?;
the_don.to_html(&mut out)?;
out.write_all(b"</a></p>\r\n\t\t\t\t\t\t<p class=\"action\">")?;
i18n!(catalog, "Accepted your Friend Request").to_html(&mut out)?;
out.write_all(b"</p>\r\n\t\t\t\t\t\t<hr/>\r\n\t\t\t\t\t\t<p class=\"notification-date\"><small><time datetime=\"2016-1-1\">13:11 PM - 2 Jan 2016</time></small></p>\r\n\t\t\t\t\t</div>\r\n\t\t\t\t</div>\r\n\t\t\t</article>\r\n\t\t</div>\r\n\r\n\t\t<div class=\"box\">\r\n\t\t\t<article class=\"media\">\r\n\t\t\t\t<div class=\"media-left\">\r\n\t\t\t\t\t<figure class=\"image is-64x64\">\r\n\t\t\t\t\t\t<img src=\"https://bulma.io/images/placeholders/128x128.png\" alt=\"Image\">\r\n\t\t\t\t\t</figure>\r\n\t\t\t\t</div>\r\n\t\t\t\t<div class=\"media-content\">\r\n\t\t\t\t\t<div class=\"content\">\r\n\t\t\t\t\t\t<p><strong>John Smith</strong> <a href=\"#link-to-user-account\">")?;
mrsmith.to_html(&mut out)?;
out.write_all(b"</a></p>\r\n\t\t\t\t\t\t<p class=\"action\">Favorited <a href=\"#your-post\">your post</a>!</p>\r\n\t\t\t\t\t\t<hr/>\r\n\t\t\t\t\t\t<p class=\"notification-date\"><small><time datetime=\"2016-1-1\">11:09 PM - 1 Jan 2016</time></small></p>\r\n\t\t\t\t\t</div>\r\n\t\t\t\t</div>\r\n\t\t\t</article>\r\n\t\t</div>\r\n\r\n\t\t<div class=\"box\">\r\n\t\t\t<article class=\"media\">\r\n\t\t\t\t<div class=\"media-left\">\r\n\t\t\t\t\t<figure class=\"image is-64x64\">\r\n\t\t\t\t\t\t<img src=\"https://bulma.io/images/placeholders/128x128.png\" alt=\"Image\">\r\n\t\t\t\t\t</figure>\r\n\t\t\t\t</div>\r\n\t\t\t\t<div class=\"media-content\">\r\n\t\t\t\t\t<div class=\"content\">\r\n\t\t\t\t\t\t<p><strong>Alice Marquee</strong> <a href=\"#link-to-user-account\">")?;
amarquis.to_html(&mut out)?;
out.write_all(b"</a></p>\r\n\t\t\t\t\t\t<p class=\"action\">Accepted your Friend Request!</p>\r\n\t\t\t\t\t\t<hr/>\r\n\t\t\t\t\t\t<p class=\"notification-date\"><small><time datetime=\"2016-1-1\">12:34 PM - 1 Jan 2016</time></small></p>\r\n\t\t\t\t\t</div>\r\n\t\t\t\t</div>\r\n\t\t\t</article>\r\n\t\t</div>\r\n\r\n\t\t<div class=\"box\">\r\n\t\t\t<article class=\"media\">\r\n\t\t\t\t<div class=\"media-left\">\r\n\t\t\t\t\t<figure class=\"image is-64x64\">\r\n\t\t\t\t\t\t<img src=\"https://bulma.io/images/placeholders/128x128.png\" alt=\"Image\">\r\n\t\t\t\t\t</figure>\r\n\t\t\t\t</div>\r\n\t\t\t\t<div class=\"media-content\">\r\n\t\t\t\t\t<div class=\"content\">\r\n\t\t\t\t\t\t<p><strong>Bob Staffers</strong> <a href=\"#link-to-user-account\">")?;
bobsyouruncle.to_html(&mut out)?;
out.write_all(b"</a></p>\r\n\t\t\t\t\t\t<p class=\"action\">Wants to be your friend.</p>\r\n\t\t\t\t\t\t<hr/>\r\n\t\t\t\t\t\t<p class=\"notification-date\"><small><time datetime=\"2016-1-1\">08:40 PM - 27 Dec 2015</time></small></p>\r\n\t\t\t\t\t</div>\r\n\t\t\t\t</div>\r\n\t\t\t</article>\r\n\t\t</div>\r\n\t\t<!-- End Sample notifications -->\r\n\t</div>\r\n</div>\r\n")?;
Ok(())
}

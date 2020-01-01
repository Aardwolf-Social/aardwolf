use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use gettext_macros::i18n;
use crate::{NotificationContent, templates::{base, elements::{alert, notification_content}}};

pub fn notification_dropdown_html<W>(mut out: &mut W, notification_dropdown: &NotificationDropdown) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
out.write_all(b"<!-- Top Nav Notifications Dropdown -->\n<div class=\"navbar-item has-dropdown is-hoverable\">\n\t<a class=\"navbar-link\">")?;
i18n!(catalog, "Notifications").to_html(&mut out)?;
out.write_all(b"</a>\n\t<div class=\"navbar-dropdown notifications-dropdown\">\n\n\t\t<!-- ************************************************* -->\n\t\t<!-- A little element to signify no new notifications. -->\n\t\t<!-- Commented out to meet wireframe criteria.         -->\n\t\t<!-- ************************************************* -->\n\t\t<!--\n\t\t<div class=\"box no-recent-activity\">\n\t\t\tNo recent activity\n\t\t</div>\n\t\t-->\n\n\t\t<!-- Begin Sample notifications -->\n\t\t<div class=\"box\">\n\t\t\t<article class=\"media\">\n\t\t\t\t<div class=\"media-left\">\n\t\t\t\t\t<figure class=\"image is-64x64\">\n\t\t\t\t\t\t<img src=\"https://bulma.io/images/placeholders/128x128.png\" alt=\"Image\">\n\t\t\t\t\t</figure>\n\t\t\t\t</div>\n\t\t\t\t<div class=\"media-content\">\n\t\t\t\t\t<div class=\"content\">\n\t\t\t\t\t\t<p><strong>Juan Quixote</strong> <a href=\"#link-to-user-account\">")?;
the_don.to_html(&mut out)?;
out.write_all(b"</a></p>\n\t\t\t\t\t\t<p class=\"action\">")?;
i18n!(catalog, "Accepted your Friend Request").to_html(&mut out)?;
out.write_all(b"</p>\n\t\t\t\t\t\t<hr/>\n\t\t\t\t\t\t<p class=\"notification-date\"><small><time datetime=\"2016-1-1\">13:11 PM - 2 Jan 2016</time></small></p>\n\t\t\t\t\t</div>\n\t\t\t\t</div>\n\t\t\t</article>\n\t\t</div>\n\n\t\t<div class=\"box\">\n\t\t\t<article class=\"media\">\n\t\t\t\t<div class=\"media-left\">\n\t\t\t\t\t<figure class=\"image is-64x64\">\n\t\t\t\t\t\t<img src=\"https://bulma.io/images/placeholders/128x128.png\" alt=\"Image\">\n\t\t\t\t\t</figure>\n\t\t\t\t</div>\n\t\t\t\t<div class=\"media-content\">\n\t\t\t\t\t<div class=\"content\">\n\t\t\t\t\t\t<p><strong>John Smith</strong> <a href=\"#link-to-user-account\">")?;
mrsmith.to_html(&mut out)?;
out.write_all(b"</a></p>\n\t\t\t\t\t\t<p class=\"action\">Favorited <a href=\"#your-post\">your post</a>!</p>\n\t\t\t\t\t\t<hr/>\n\t\t\t\t\t\t<p class=\"notification-date\"><small><time datetime=\"2016-1-1\">11:09 PM - 1 Jan 2016</time></small></p>\n\t\t\t\t\t</div>\n\t\t\t\t</div>\n\t\t\t</article>\n\t\t</div>\n\n\t\t<div class=\"box\">\n\t\t\t<article class=\"media\">\n\t\t\t\t<div class=\"media-left\">\n\t\t\t\t\t<figure class=\"image is-64x64\">\n\t\t\t\t\t\t<img src=\"https://bulma.io/images/placeholders/128x128.png\" alt=\"Image\">\n\t\t\t\t\t</figure>\n\t\t\t\t</div>\n\t\t\t\t<div class=\"media-content\">\n\t\t\t\t\t<div class=\"content\">\n\t\t\t\t\t\t<p><strong>Alice Marquee</strong> <a href=\"#link-to-user-account\">")?;
amarquis.to_html(&mut out)?;
out.write_all(b"</a></p>\n\t\t\t\t\t\t<p class=\"action\">Accepted your Friend Request!</p>\n\t\t\t\t\t\t<hr/>\n\t\t\t\t\t\t<p class=\"notification-date\"><small><time datetime=\"2016-1-1\">12:34 PM - 1 Jan 2016</time></small></p>\n\t\t\t\t\t</div>\n\t\t\t\t</div>\n\t\t\t</article>\n\t\t</div>\n\n\t\t<div class=\"box\">\n\t\t\t<article class=\"media\">\n\t\t\t\t<div class=\"media-left\">\n\t\t\t\t\t<figure class=\"image is-64x64\">\n\t\t\t\t\t\t<img src=\"https://bulma.io/images/placeholders/128x128.png\" alt=\"Image\">\n\t\t\t\t\t</figure>\n\t\t\t\t</div>\n\t\t\t\t<div class=\"media-content\">\n\t\t\t\t\t<div class=\"content\">\n\t\t\t\t\t\t<p><strong>Bob Staffers</strong> <a href=\"#link-to-user-account\">")?;
bobsyouruncle.to_html(&mut out)?;
out.write_all(b"</a></p>\n\t\t\t\t\t\t<p class=\"action\">Wants to be your friend.</p>\n\t\t\t\t\t\t<hr/>\n\t\t\t\t\t\t<p class=\"notification-date\"><small><time datetime=\"2016-1-1\">08:40 PM - 27 Dec 2015</time></small></p>\n\t\t\t\t\t</div>\n\t\t\t\t</div>\n\t\t\t</article>\n\t\t</div>\n\t\t<!-- End Sample notifications -->\n\t</div>\n</div>\n")?;
Ok(())
}

use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};

pub fn html_head<W>(mut out: &mut W, title: &str) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
out.write_all(b"<script src=\"/web/javascript/vue.2.5.20.min.js\"></script>\n<script src=\"/web/javascript/components/notificationBox.js\"></script>\n<script src=\"/web/javascript/components/showNotifcations.js\"></script>\n\n<head>\n  <meta charset=\"utf-8\">\n  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n  <title>")?;
title.to_html(&mut out)?;
out.write_all(b"</title>\n  <link rel=\"stylesheet\" href=\"/web/css/fork-awesome-1.1.7/css/fork-awesome.min.css\">\n  <link rel=\"stylesheet\" href=\"/web/css/bulma-0.7.5/css/bulma.min.css\">\n  <link rel=\"stylesheet\" href=\"/web/css/base.css\">\n  <link rel=\"stylesheet\" href=\"/web/css/scratchpad.css\">\n  <!-- Component styles -->\n  <link rel=\"stylesheet\" href=\"/web/css/components/notificationBox.css\">\n</head>\n")?;
Ok(())
}

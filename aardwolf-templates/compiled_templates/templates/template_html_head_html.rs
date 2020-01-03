use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};

pub fn html_head_html<W>(mut out: &mut W, title: &str) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
out.write_all(b"<script src=\"/web/javascript/vue.2.5.20.min.js\"></script>\r\n<script src=\"/web/javascript/components/notificationBox.js\"></script>\r\n<script src=\"/web/javascript/components/showNotifcations.js\"></script>\r\n\r\n<head>\r\n  <meta charset=\"utf-8\">\r\n  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\r\n  <title>")?;
title.to_html(&mut out)?;
out.write_all(b"</title>\r\n  <link rel=\"stylesheet\" href=\"/web/css/fork-awesome-1.1.7/css/fork-awesome.min.css\">\r\n  <link rel=\"stylesheet\" href=\"/web/css/bulma-0.7.5/css/bulma.min.css\">\r\n  <link rel=\"stylesheet\" href=\"/web/css/base.css\">\r\n  <link rel=\"stylesheet\" href=\"/web/css/scratchpad.css\">\r\n  <!-- Component styles -->\r\n  <link rel=\"stylesheet\" href=\"/web/css/components/notificationBox.css\">\r\n</head>\r\n")?;
Ok(())
}

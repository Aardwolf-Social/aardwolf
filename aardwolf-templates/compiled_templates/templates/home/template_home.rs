use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use crate::{Home, templates::{base, home::{feed, home_nav_top}, new_post, aside_shortcuts}};

pub fn home<W>(mut out: &mut W, home: &Home) -> io::Result<()> where W: ?Sized, for<'a> &'a mut W: Write {
base(&mut out, home.catalog, "Aardwolf | Home", |mut out| {
out.write_all(b"\n    ")?;
nav(&mut out, home.catalog)?;
out.write_all(b"\n    <section class=\"section\">\n        ")?;
shortcuts(&mut out, &home.shortcuts)?;
out.write_all(b"\n    </section>\n    <section class=\"section\">\n        <div class=\"container\">\n            ")?;
new_post(&mut out, home.catalog, &home.shortcuts.username, &home.new_post)?;
out.write_all(b"\n            ")?;
feed(&mut out, home.catalog)?;
out.write_all(b"\n        </div>\n    </section>\n")?;

Ok(())
}
)?;
out.write_all(b"\n")?;
Ok(())
}

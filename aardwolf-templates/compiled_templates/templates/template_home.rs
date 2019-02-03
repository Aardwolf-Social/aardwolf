use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use crate::{Home, templates::{base, home::{feed, nav}, new_post, shortcuts}};

pub fn home(out: &mut Write, home: &Home) -> io::Result<()> {
base(out, home.catalog, "Aardwolf | Home", |out| {
out.write_all(b"\n    ")?;
nav(out, home.catalog)?;
out.write_all(b"\n    <section class=\"section\">\n        ")?;
shortcuts(out, &home.shortcuts)?;
out.write_all(b"\n    </section>\n    <section class=\"section\">\n        <div class=\"container\">\n            ")?;
new_post(out, home.catalog, &home.shortcuts.username, &home.new_post)?;
out.write_all(b"\n            ")?;
feed(out, home.catalog)?;
out.write_all(b"\n        </div>\n    </section>\n")?;

Ok(())
}
)?;
out.write_all(b"\n")?;
Ok(())
}

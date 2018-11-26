use std::io::{self, Write};
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use ::templates::{Html,ToHtml};
use crate::{Home, templates::{base, home::{feed, nav}, new_post, shortcuts}};

pub fn home(out: &mut Write, home: Home)
-> io::Result<()> {
base(out, home.catalog, "Aardwolf | Home", |out| {
write!(out, "\n    ")?;
nav(out, home.catalog)?;
write!(out, "\n    <section class=\"section\">\n        ")?;
shortcuts(out, home.shortcuts)?;
write!(out, "\n    </section>\n    <section class=\"section\">\n        <div class=\"container\">\n            ")?;
new_post(out, home.catalog)?;
write!(out, "\n            ")?;
feed(out, home.catalog)?;
write!(out, "\n        </div>\n    </section>\n")?;

Ok(())
}
)?;
write!(out, "\n")?;
Ok(())
}

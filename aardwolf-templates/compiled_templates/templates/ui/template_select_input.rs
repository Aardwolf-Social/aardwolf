use std::io::{self, Write};
#[allow(renamed_and_removed_lints)]
#[cfg_attr(feature="cargo-clippy", allow(useless_attribute))]
#[allow(unused)]
use super::{Html,ToHtml};
use rocket_i18n::i18n;
use crate::SelectInput;

pub fn select_input(out: &mut Write, input: &SelectInput) -> io::Result<()> {
out.write_all(b"<div class=\"aardwolf-input-wrapper\">\n    <label for=\"")?;
input.name.to_html(out)?;
out.write_all(b"\">\n        ")?;
i18n!(input.catalog, input.label).to_html(out)?;
out.write_all(b"\n    </label>\n    <div class=\"aardwolf-input aardwolf-select-input\">\n        <select name=\"")?;
input.name.to_html(out)?;
out.write_all(b"\">\n            ")?;
for option in input.options.iter() {
out.write_all(b"\n                ")?;
if option.value == input.selected {
out.write_all(b"\n                    <option value=\"")?;
option.value.to_html(out)?;
out.write_all(b"\" selected>\n                        ")?;
i18n!(input.catalog, option.display).to_html(out)?;
out.write_all(b"\n                    </option>\n                ")?;
} else {
out.write_all(b"\n                    <option value=\"")?;
option.value.to_html(out)?;
out.write_all(b"\">\n                        ")?;
i18n!(input.catalog, option.display).to_html(out)?;
out.write_all(b"\n                    </option>\n                ")?;
}
out.write_all(b"\n            ")?;
}
out.write_all(b"</select>\n    </div>\n    <div class=\"aardwolf-input-meta\">\n        ")?;
if let Some(error) = input.error {
out.write_all(b"\n            <span class=\"aardwolf-input-error\">")?;
i18n!(input.catalog, error).to_html(out)?;
out.write_all(b"</span>\n        ")?;
}
out.write_all(b"\n    </div>\n</div>\n")?;
Ok(())
}

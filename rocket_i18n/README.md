# Rocket I18N [![Build Status](https://travis-ci.org/BaptisteGelez/rocket_i18n.svg?branch=master)](https://travis-ci.org/BaptisteGelez/rocket_i18n)

A crate to help you internationalize your Rocket applications.

## Features

- Create `.po` files for locales listed in `po/LINGUAS`, from a POT file
- Update `.po` files from the POT file if needed
- Compile `.po` files into `.mo` ones
- Select the correct locale for each request
- Integrates with Tera templates

## Usage

First add it to your `Cargo.toml` (you have to use the git version, because we can't publish the latest version on [https://crates.io](crates.io) as it depends on the `master` branch of Rocket):

```toml
[dependencies.rocket_i18n]
git = "https://github.com/BaptisteGelez/rocket_i18n"
rev = "457b88c59ec31905a9193df43df58bee55b4b83d"
```

Then, in your `main.rs`:

```rust
extern crate rocket_i18n;

// ...

fn main() {
    rocket::ignite()
        // Register the fairing. The parameter is the domain you want to use (the name of your app most of the time)
        .attach(rocket_i18n::I18n::new("my_app"))
        // Eventually register the Tera filters (only works with the master branch of Rocket)
        .attach(rocket_contrib::Template::custom(|engines| {
            rocket_i18n::tera(&mut engines.tera);
        }))
        // Register routes, etc
}
```

### Using Tera filters

If you called `rocket_i18n::tera`, you'll be able to use two Tera filters to translate your interface.

The first one, `_`, corresponds to the `gettext` function of gettext. It takes a string as input and translate it. Any argument given to the filter can
be used in the translated string using the Tera syntax.

```jinja
<p>{{ "Hello, world" | _ }}</p>
<p>{{ "Your name is {{ name }}" | _(name=user.name) }}
```

The second one, `_n`, is equivalent to `ngettext`. It takes the plural form as input, and two required arguments in addition to those you may want to use for interpolation:

- `singular`, the singular form of this string
- `count`, the number of items, to determine how the string should be pluralized

```jinja
<p>{{ "{{ count }} new messages" | _n(singular="One new message", count=messages.unread_count) }}</p>
```

### In Rust code

You can also use all the gettext functions in your Rust code.

```rust
use rocket_i18n;

#[get("/")]
fn index() -> String {
    gettext("Hello, world!")
}

#[get("/<name>")]
fn hello(name: String) -> String {
    format!(gettext("Hello, {}!"), name)
}
```

### Editing the POT

For those strings to be translatable you should also add them to the `po/YOUR_DOMAIN.pot` file. To add a simple message, just do:

```po
msgid "Hello, world" # The string you used with your filter
msgstr "" # Always empty
```

For plural forms, the syntax is a bit different:

```po
msgid "You have one new notification" # The singular form
msgid_plural "You have {{ count }} new notifications" # The plural one
msgstr[0] ""
msgstr[1] ""
```

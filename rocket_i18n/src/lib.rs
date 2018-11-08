//! # Rocket I18N
//! 
//! A crate to help you internationalize your Rocket applications.
//! 
//! ## Features
//! 
//! - Create `.po` files for locales listed in `po/LINGUAS`, from a POT file
//! - Update `.po` files from the POT file if needed
//! - Compile `.po` files into `.mo` ones
//! - Select the correct locale for each request
//! - Integrates with Tera templates
//! 
//! ## Usage
//! 
//! First add it to your `Cargo.toml` (you have to use the git version, because we can't publish the latest version on [https://crates.io](crates.io) as it depends on the `master` branch of Rocket):
//! 
//! ```toml
//! [dependencies.rocket_i18n]
//! git = "https://github.com/BaptisteGelez/rocket_i18n"
//! rev = "457b88c59ec31905a9193df43df58bee55b4b83d"
//! ```
//! 
//! Then, in your `main.rs`:
//! 
//! ```rust,no_run
//! # //can't be run because 'po/LINGUAS' don't exist
//! # extern crate rocket;
//! # extern crate rocket_contrib;
//! extern crate rocket_i18n;
//! 
//! // ...
//! 
//! fn main() {
//!     rocket::ignite()
//!         // Register the fairing. The parameter is the domain you want to use (the name of your app most of the time)
//!         .attach(rocket_i18n::I18n::new("my_app"))
//!         // Eventually register the Tera filters (only works with the master branch of Rocket)
//!         .attach(rocket_contrib::Template::custom(|engines| {
//!             rocket_i18n::tera(&mut engines.tera);
//!         }));
//!         // Register routes, etc
//! }
//! ```
//! 
//! ### Using Tera filters
//! 
//! If you called `rocket_i18n::tera`, you'll be able to use two Tera filters to translate your interface.
//! 
//! The first one, `_`, corresponds to the `gettext` function of gettext. It takes a string as input and translate it. Any argument given to the filter can
//! be used in the translated string using the Tera syntax.
//! 
//! ```jinja
//! <p>{{ "Hello, world" | _ }}</p>
//! <p>{{ "Your name is {{ name }}" | _(name=user.name) }}
//! ```
//! 
//! The second one, `_n`, is equivalent to `ngettext`. It takes the plural form as input, and two required arguments in addition to those you may want to use for interpolation:
//! 
//! - `singular`, the singular form of this string
//! - `count`, the number of items, to determine how the string should be pluralized
//! 
//! ```jinja
//! <p>{{ "{{ count }} new messages" | _n(singular="One new message", count=messages.unread_count) }}</p>
//! ```
//! 
//! ### In Rust code
//! 
//! You can also use all the gettext functions in your Rust code.
//! 
//! ```rust
//! # #![feature(custom_attribute)]
//! use rocket_i18n::gettext;
//! 
//! #[get("/")]
//! fn index() -> String {
//!     gettext("Hello, world!")
//! }
//! ```
//! 
//! ### Editing the POT
//! 
//! For those strings to be translatable you should also add them to the `po/YOUR_DOMAIN.pot` file. To add a simple message, just do:
//! 
//! ```po
//! msgid "Hello, world" # The string you used with your filter
//! msgstr "" # Always empty
//! ```
//! 
//! For plural forms, the syntax is a bit different:
//! 
//! ```po
//! msgid "You have one new notification" # The singular form
//! msgid_plural "You have {{ count }} new notifications" # The plural one
//! msgstr[0] ""
//! msgstr[1] ""
//! ```
//! 

extern crate gettextrs;
extern crate rocket;
extern crate serde_json;
extern crate tera;

pub use gettextrs::*;
use rocket::{Data, Request, Rocket, fairing::{Fairing, Info, Kind}};
use std::{
    collections::HashMap,
    env,
    fs,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    process::Command
};
use tera::{Tera, Error as TeraError};

const ACCEPT_LANG: &'static str = "Accept-Language";

/// This is the main struct of this crate. You can register it on your Rocket instance as a
/// fairing.
/// 
/// ```rust,no_run
/// # //can't be run because 'po/LINGUAS' don't exist
/// # extern crate rocket;
/// # extern crate rocket_i18n;
/// rocket::ignite()
///     .attach(rocket_i18n::I18n::new("app"));
/// ```
/// 
/// The parameter you give to [`I18n::new`] is the gettext domain to use. It doesn't really matter what you choose,
/// but it is usually the name of your app.
/// 
/// Once this fairing is registered, it will update your .po files from the POT, compile them into .mo files, and select
/// the requested locale for each request using the `Accept-Language` HTTP header.
pub struct I18n {
    domain: &'static str
}

impl I18n {
    /// Creates a new I18n fairing for the given domain
    pub fn new(domain: &'static str) -> I18n {
        I18n {
            domain: domain
        }
    }
}

impl Fairing for I18n {
    fn info(&self) -> Info {
        Info {
            name: "Gettext I18n",
            kind: Kind::Attach | Kind::Request
        }
    }

    fn on_attach(&self, rocket: Rocket) -> Result<Rocket, Rocket> {
        update_po(self.domain);
        compile_po(self.domain);

        bindtextdomain(self.domain, fs::canonicalize(&PathBuf::from("./translations/")).unwrap().to_str().unwrap());
        textdomain(self.domain);
        Ok(rocket)
    }

    fn on_request(&self, request: &mut Request, _: &Data) {
        let lang = request
            .headers()
            .get_one(ACCEPT_LANG)
            .unwrap_or("en")
            .split(",")
            .filter_map(|lang| lang
                // Get the locale, not the country code
                .split(|c| c=='-'||c==';')
                .nth(0)
                )
            // Get the first requested locale we support
            .find(|lang|  get_locales().contains(&lang.to_string()))
            .unwrap_or("en");
        
        // We can't use setlocale(LocaleCategory::LcAll, lang), because it only accepts system-wide installed
        // locales (and most of the time there are only a few of them).
        // But, when we set the LANGUAGE environment variable, and an empty string as a second parameter to
        // setlocale, gettext will be smart enough to find a matching locale in the locally installed ones.
        env::set_var("LANGUAGE", lang);
        setlocale(LocaleCategory::LcAll, "");
    }
}

fn update_po(domain: &str) {
    let pot_path = Path::new("po").join(format!("{}.pot", domain));

    for lang in get_locales() {
        let po_path = Path::new("po").join(format!("{}.po", lang.clone()));
        if po_path.exists() && po_path.is_file() {
            println!("Updating {}", lang.clone());
            // Update it
            Command::new("msgmerge")
                .arg("-U")
                .arg(po_path.to_str().unwrap())
                .arg(pot_path.to_str().unwrap())
                .spawn()
                .expect("Couldn't update PO file");
        } else {
            println!("Creating {}", lang.clone());
            // Create it from the template
            Command::new("msginit")
                .arg(format!("--input={}", pot_path.to_str().unwrap()))
                .arg(format!("--output-file={}", po_path.to_str().unwrap()))
                .arg("-l")
                .arg(lang)
                .arg("--no-translator")
                .spawn()
                .expect("Couldn't init PO file");
        }
    }
}

fn compile_po(domain: &str) {
    for lang in get_locales() {
        let po_path = Path::new("po").join(format!("{}.po", lang.clone()));
        let mo_dir = Path::new("translations")
            .join(lang.clone())
            .join("LC_MESSAGES");
        fs::create_dir_all(mo_dir.clone()).expect("Couldn't create MO directory");
        let mo_path = mo_dir.join(format!("{}.mo", domain));

        Command::new("msgfmt")
            .arg(format!("--output-file={}", mo_path.to_str().unwrap()))
            .arg(po_path)
            .spawn()
            .expect("Couldn't compile translations");
    }
}

fn get_locales() -> Vec<String> {
    let linguas_file = fs::File::open(Path::new("po").join("LINGUAS")).expect("Couldn't find po/LINGUAS file");
    let linguas = BufReader::new(&linguas_file);
    linguas.lines().map(Result::unwrap).collect()
}

fn tera_gettext(msg: serde_json::Value, ctx: HashMap<String, serde_json::Value>) -> Result<serde_json::Value, TeraError> {
    let trans = gettext(msg.as_str().unwrap());
    Ok(serde_json::Value::String(Tera::one_off(trans.as_ref(), &ctx, false).unwrap_or(String::from(""))))
}

fn tera_ngettext(msg: serde_json::Value, ctx: HashMap<String, serde_json::Value>) -> Result<serde_json::Value, TeraError> {
    let trans = ngettext(
        ctx.get("singular").unwrap().as_str().unwrap(),
        msg.as_str().unwrap(),
        ctx.get("count").unwrap().as_u64().unwrap() as u32
    );
    Ok(serde_json::Value::String(Tera::one_off(trans.as_ref(), &ctx, false).unwrap_or(String::from(""))))
}

/// Register translation filters on your Tera instance
/// 
/// ```rust
/// # extern crate rocket;
/// # extern crate rocket_contrib;
/// # extern crate rocket_i18n;
/// rocket::ignite()
///     .attach(rocket_contrib::Template::custom(|engines| {
///         rocket_i18n::tera(&mut engines.tera);
///     }));
/// ```
/// 
/// The two registered filters are `_` and `_n`. For example use, see the crate documentation,
/// or the project's README.
pub fn tera(t: &mut Tera) {
    t.register_filter("_", tera_gettext);
    t.register_filter("_n", tera_ngettext);
}

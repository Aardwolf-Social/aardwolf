extern crate rocket_i18n;

use std::env;

fn main() {
    read_config();
    let config_changed = configure();
    if config_changed {
        write_config();
    }
    validate_features();
    build_translations();
}

/* Compile the translations
 */
fn build_translations() {
    rocket_i18n::compile_po("aardwolf", &["en".to_owned(), "pl".to_owned()]);
    rocket_i18n::update_po("aardwolf", &["en".to_owned(), "pl".to_owned()]);
}

/* Looks for a .config file. If found, reads it's values into the
 * environment for re-use. If a value is already set, it will skip setting
 * it and defer to the new value.
 */
fn read_config() {}

/* Goes through the build configuration values, tries to auto-detect
 * missing values, and checks that re-used values are still valid.
 * Returns: true if any values have changed from .config, false otherwise.
 */
fn configure() -> bool {
    let config_changed = false;

    config_changed
}

/* Writes out the configuration to .config for re-use next build.
 */
fn write_config() {}

/* Checks that incompatible features haven't been selected.
 * Panics if so.
 */
fn validate_features() {
    /* Exactly one logging implementation must be selected */
    {
        let mut numlog = 0;
        let log_implementations = vec!["simple-logging", "syslog", "systemd"];

        for imp in log_implementations {
            let imp = imp.to_uppercase();
            let imp = imp.replace("-", "_");
            let feature = "CARGO_FEATURE_".to_string() + &imp;
            if let Ok(_) = env::var(&feature) {
                numlog += 1;
                println!("Feature selected: {}", feature);
            }
        }

        if numlog != 1 {
            panic!("CONFIG ERROR: Exactly one logging implementation must be configured.");
        }
    }
}

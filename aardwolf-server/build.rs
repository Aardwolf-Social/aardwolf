use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
 read_config();
 let config_changed = configure();
 if config_changed {write_config();}
 validate_features();
}

/* Looks for a .config file. If found, reads it's values into the
 * environment for re-use. If a value is already set, it will skip setting
 * it and defer to the new value.
 */
fn read_config() {

}

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
fn write_config() {

}

/* Checks that incompatible features haven't been selected.
 * Panics if so.
 */
fn validate_features() {
    /* Only one logging implementation may be selected */
    let blah = env::var("CARGO_FEATURE_USE_SYSTEMD").unwrap();
    panic!("Assuming features failed validation! {}", blah);
}

use serde::{Deserialize, Serialize};
use yew::prelude::*;

// Pull in the template modules
pub mod templates;

// Start using the template
use templates::elements::main_title::PageTitle;
use templates::layout::footer::Footer;
use templates::pages::sign_up::SignUp;

// Lets make a struct for testing console logging
#[derive(Serialize, Deserialize)]
struct LogMe {
    username: String,
    favorite_pizza: String,
}

#[function_component(Aardwolf)]
pub fn aardwolf() -> Html {
    // Demo for logging to the browsers console
    log::info!("Hello browser console! :D");
    log::error!("Ohnoes! There was an OOPSIE!");

    let test_debug = "THAT THING";
    log::debug!("Hey dev, heres the {} you wanted!!", test_debug);

    // Give values to our LogMe struct
    let log_me = LogMe {
        username: "DemoUser".to_owned(),
        favorite_pizza: "Hawaiian".to_owned(),
    };

    // Use j as a variable for our serde_json string?
    let j = serde_json::to_string_pretty(&log_me);

    // Finally pretty print our serde'd struct data x'D!
    log::info!("{:#?}", j);

    // Start of the html! Yew macro
    html! {
        <>
            <PageTitle page_title="Aardwolf-Social | Sign Up!" />
            <SignUp />
            <Footer />
        </>
    }
} // End of pub fn aardwolf()

// Function to convert lists into HTML
pub fn list_to_html(list: Vec<&str>) -> Vec<Html> {
    // Iterate items onto a map, wrap them in <li> tags, and build the new list
    list.iter().map(|item| html! {<li>{item}</li>}).collect()
}


// Load I18n macro, for allow you use `t!` macro in anywhere.
#[macro_use]
extern crate rust_i18n;

// Config fallback missing translations to "en" locale.
// Use `fallback` option to set fallback locale.
i18n!("locales", fallback = "en-us");
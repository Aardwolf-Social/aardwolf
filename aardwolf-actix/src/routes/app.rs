use actix_web::{http::header::LOCATION, HttpResponse, State};
use collection_macros::hashmap;

use crate::{error::RenderResult, types::user::SignedInUserWithEmail, AppConfig};

pub(crate) fn index(
    (state, maybe_user): (State<AppConfig>, Option<SignedInUserWithEmail>),
) -> RenderResult {
    match maybe_user {
        Some(user) => logged_in_index((state, user)),
        None => Ok(logged_out_index()),
    }
}

fn logged_out_index() -> HttpResponse {
    HttpResponse::SeeOther()
        .header(LOCATION, "/auth/sign_in")
        .finish()
}

fn logged_in_index((state, user): (State<AppConfig>, SignedInUserWithEmail)) -> RenderResult {
    let map = hashmap!{
        "email" => user.1.to_verified()
            .map(|verified| verified.email().to_owned())
            .unwrap_or_else(|unverified| unverified.email().to_owned())
    };

    state.render("home", &map)
}

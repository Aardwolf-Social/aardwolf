use actix_web::{http::header::LOCATION, FromRequest, HttpRequest, HttpResponse, State};
use futures::Future;

use crate::{error::RenderResult, types::user::SignedInUserWithEmail, AppConfig};

pub(crate) fn index(
    (req, state): (HttpRequest<AppConfig>, State<AppConfig>),
) -> Box<dyn Future<Item = HttpResponse, Error = actix_web::Error>> {
    Box::new(SignedInUserWithEmail::from_request(&req, &()).then(|res| {
        match res {
            Ok(user) => logged_in_index((state, user)).map_err(From::from),
            _ => Ok(HttpResponse::SeeOther()
                .header(LOCATION, "/auth/sign_in")
                .finish()),
        }
    }))
}

fn logged_in_index((state, user): (State<AppConfig>, SignedInUserWithEmail)) -> RenderResult {
    let map = hashmap!{
        "email" => user.1.to_verified()
            .map(|verified| verified.email().to_owned())
            .unwrap_or_else(|unverified| unverified.email().to_owned())
    };

    state.render("home", &map)
}

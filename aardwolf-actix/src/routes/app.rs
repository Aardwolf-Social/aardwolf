use aardwolf_models::user::UserLike;
use aardwolf_templates::templates;
use actix_web::{http::header::LOCATION, HttpResponse, State};
use rocket_i18n::I18n;

use crate::{types::user::SignedInUserWithEmail, AppConfig};

pub(crate) fn index(
    (state, maybe_user, i18n): (State<AppConfig>, Option<SignedInUserWithEmail>, I18n),
) -> HttpResponse {
    match maybe_user {
        Some(user) => logged_in_index((state, user, i18n)),
        None => logged_out_index(),
    }
}

fn logged_out_index() -> HttpResponse {
    HttpResponse::SeeOther()
        .header(LOCATION, "/auth/sign_in")
        .finish()
}

fn logged_in_index(
    (state, user, i18n): (State<AppConfig>, SignedInUserWithEmail, I18n),
) -> HttpResponse {
    state.render(move |buf| {
        templates::home(
            buf,
            i18n.catalog.clone(),
            user.0.id().to_string().as_ref(),
            user.0.id().to_string().as_ref(),
        )
    })
}

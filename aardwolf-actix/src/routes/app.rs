use aardwolf_models::user::UserLike;
use actix_web::{http::header::LOCATION, HttpResponse};
use rocket_i18n::I18n;

use crate::{types::user::SignedInUser, WithRucte};

pub(crate) fn index((maybe_user, i18n): (Option<SignedInUser>, I18n)) -> HttpResponse {
    match maybe_user {
        Some(user) => logged_in_index((user, i18n)),
        None => logged_out_index(),
    }
}

fn logged_out_index() -> HttpResponse {
    HttpResponse::SeeOther()
        .header(LOCATION, "/auth/sign_in")
        .finish()
}

fn logged_in_index((user, i18n): (SignedInUser, I18n)) -> HttpResponse {
    HttpResponse::Ok().with_ructe(aardwolf_templates::Home::new(
        &i18n.catalog,
        user.0.id().to_string().as_ref(),
        user.0.id().to_string().as_ref(),
    ))
}

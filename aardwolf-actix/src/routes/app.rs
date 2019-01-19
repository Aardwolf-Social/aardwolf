use aardwolf_models::{sql_types::PostVisibility, user::UserLike};
use aardwolf_types::forms::posts::PostCreationFormState;
use actix_web::{http::header::LOCATION, middleware::session::Session, HttpResponse};
use rocket_i18n::I18n;

use crate::{types::user::SignedInUser, WithRucte};

pub(crate) fn index(
    (session, maybe_user, i18n): (Session, Option<SignedInUser>, I18n),
) -> HttpResponse {
    match maybe_user {
        Some(user) => logged_in_index((session, user, i18n)),
        None => logged_out_index(),
    }
}

fn logged_out_index() -> HttpResponse {
    HttpResponse::SeeOther()
        .header(LOCATION, "/auth/sign_in")
        .finish()
}

fn logged_in_index((session, user, i18n): (Session, SignedInUser, I18n)) -> HttpResponse {
    if session.get::<i32>("persona_id").unwrap_or(None).is_some()
        || user.0.primary_persona().is_some()
    {
        HttpResponse::Ok().with_ructe(aardwolf_templates::Home::new(
            &i18n.catalog,
            user.0.id().to_string().as_ref(),
            user.0.id().to_string().as_ref(),
            "csrf token",
            &PostCreationFormState {
                source: "".to_owned(),
                name: None,
                visibility: PostVisibility::Public, // TODO: this comes from the persona
            },
            None,
            false,
        ))
    } else {
        HttpResponse::SeeOther()
            .header(LOCATION, "/personas/create")
            .finish()
    }
}

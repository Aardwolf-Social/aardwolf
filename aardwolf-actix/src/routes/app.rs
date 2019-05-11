use aardwolf_types::forms::posts::PostCreationFormState;
use actix_i18n::I18n;
use actix_web::HttpResponse;

use crate::{
    types::{actor::CurrentActor, user::SignedInUser},
    WithRucte,
};

pub(crate) fn index((_user, actor, i18n): (SignedInUser, CurrentActor, I18n)) -> HttpResponse {
    HttpResponse::Ok().with_ructe(aardwolf_templates::Home::new(
        &i18n.catalog,
        &actor.1.shortname(),
        &actor.0.profile_url().0.to_string(),
        "csrf token",
        &PostCreationFormState {
            source: "".to_owned(),
            name: None,
            visibility: actor.1.default_visibility(),
        },
        None,
        false,
    ))
}

use aardwolf_templates::Home;
use aardwolf_types::forms::posts::PostCreationFormState;
use rocket_i18n::I18n;
use actix_web::HttpResponse;

use crate::{
    traits::RenderableExt,
    types::{actor::CurrentActor, user::SignedInUser},
};

pub(crate) fn index((_user, actor, i18n): (SignedInUser, CurrentActor, I18n)) -> HttpResponse {
    Home::new(
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
    )
    .ok()
}

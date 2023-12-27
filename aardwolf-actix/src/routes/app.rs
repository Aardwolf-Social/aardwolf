use aardwolf_templates::home::Home;
use aardwolf_types::forms::posts::PostCreationFormState;
use actix_web::{web::Data, HttpResponse};
use rocket_i18n::I18n;

use crate::{
    traits::RenderableExt,
    types::{actor::CurrentActor, user::SignedInUser},
};

pub(crate) async fn index(
    (_user, actor, i18n): (SignedInUser, CurrentActor, Data<I18n>),
) -> HttpResponse {
    Home::new(
        &i18n.catalog,
        actor.1.shortname(),
        actor.0.profile_url().0.as_ref(),
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

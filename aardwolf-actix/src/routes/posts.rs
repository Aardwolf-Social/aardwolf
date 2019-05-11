use aardwolf_types::{
    forms::posts::{PostCreationForm, ValidatePostCreationFail, ValidatePostCreationForm},
    operations::{
        check_create_post_permission::{CheckCreatePostPermission, CheckCreatePostPermissionFail},
        create_post::{CreatePost, CreatePostFail},
    },
};
use actix_i18n::I18n;
use actix_web::{http::header::LOCATION, web::{Form, Data}, HttpResponse};
use failure::Fail;
use futures::Future;

use crate::{
    db::DbActionError,
    types::{actor::CurrentActor, user::SignedInUser},
    AppConfig, WithRucte,
};

#[derive(Clone, Debug, Fail)]
pub enum PostCreateError {
    #[fail(display = "Error talking to db actor")]
    Mailbox,
    #[fail(display = "Error talking db")]
    Database,
    #[fail(display = "User does not have permission to create a persona")]
    Permission,
    #[fail(display = "Could not set cookie")]
    Cookie,
    #[fail(display = "Submitted form is invalid")]
    Form(#[cause] ValidatePostCreationFail),
}

impl From<ValidatePostCreationFail> for PostCreateError {
    fn from(e: ValidatePostCreationFail) -> Self {
        PostCreateError::Form(e)
    }
}

impl From<CreatePostFail> for PostCreateError {
    fn from(e: CreatePostFail) -> Self {
        match e {
            CreatePostFail::Database => PostCreateError::Database,
        }
    }
}

impl From<CheckCreatePostPermissionFail> for PostCreateError {
    fn from(e: CheckCreatePostPermissionFail) -> Self {
        match e {
            CheckCreatePostPermissionFail::Database => PostCreateError::Database,
            CheckCreatePostPermissionFail::Permission => PostCreateError::Permission,
        }
    }
}

impl From<DbActionError<CheckCreatePostPermissionFail>> for PostCreateError {
    fn from(e: DbActionError<CheckCreatePostPermissionFail>) -> Self {
        match e {
            DbActionError::Connection => PostCreateError::Database,
            DbActionError::Mailbox => PostCreateError::Mailbox,
            DbActionError::Action(e) => e.into(),
        }
    }
}

impl From<DbActionError<CreatePostFail>> for PostCreateError {
    fn from(e: DbActionError<CreatePostFail>) -> Self {
        match e {
            DbActionError::Connection => PostCreateError::Database,
            DbActionError::Mailbox => PostCreateError::Mailbox,
            DbActionError::Action(e) => e.into(),
        }
    }
}
pub(crate) fn create(
    (state, user, actor, form, i18n): (
        Data<AppConfig>,
        SignedInUser,
        CurrentActor,
        Form<PostCreationForm>,
        I18n,
    ),
) -> Box<dyn Future<Item = HttpResponse, Error = actix_web::error::Error>> {
    let form = form.into_inner();
    let form_state = form.as_state();
    let user = user.0;
    let CurrentActor(base_actor, persona) = actor;
    let base_actor2 = base_actor.clone();

    let res = perform!((*state).clone(), PostCreateError, [
        (form = ValidatePostCreationForm(form)),
        (creater = CheckCreatePostPermission(user.clone(), base_actor2)),
        (_ = CreatePost(creater, form, state.generator.clone())),
    ]);

    Box::new(
        res.map(move |_| HttpResponse::SeeOther().header(LOCATION, "/").finish())
            .or_else(move |e| {
                let (mut res, validation, system) = match e {
                    PostCreateError::Form(ref e) => (HttpResponse::BadRequest(), Some(e), false),
                    _ => (HttpResponse::InternalServerError(), None, true),
                };

                Ok(res.with_ructe(aardwolf_templates::Home::new(
                    &i18n.catalog,
                    "csrf",
                    persona.shortname(),
                    &base_actor.profile_url().0.to_string(),
                    &form_state,
                    validation,
                    system,
                )))
            }),
    )
}

use aardwolf_models::{base_actor::BaseActor, user::AuthenticatedUser};
use aardwolf_types::{
    forms::posts::{PostCreationForm, ValidatePostCreationFail, ValidatePostCreationForm},
    operations::{
        check_create_post_permission::{CheckCreatePostPermission, CheckCreatePostPermissionFail},
        create_post::{CreatePost, CreatePostFail},
    },
};
use actix_i18n::I18n;
use actix_web::{
    web::{Data, Form},
    HttpResponse,
};
use failure::Fail;

use crate::{
    action::{Impossible, Redirect},
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

impl From<Impossible> for PostCreateError {
    fn from(e: Impossible) -> Self {
        match e {}
    }
}

async fn create_inner(
    state: AppConfig,
    form: PostCreationForm,
    user: AuthenticatedUser,
    base_actor: BaseActor,
) -> Result<HttpResponse, PostCreateError> {
    Ok(perform!(state, [
        (form = ValidatePostCreationForm(form)),
        (creator = CheckCreatePostPermission(user, base_actor)),
        (_ = CreatePost(creator, form, state.generator.clone())),
        (_ = Redirect("/".to_owned())),
    ]))
}

pub(crate) async fn create(
    (state, user, actor, form, i18n): (
        Data<AppConfig>,
        SignedInUser,
        CurrentActor,
        Form<PostCreationForm>,
        I18n,
    ),
) -> Result<HttpResponse, actix_web::Error> {
    let form = form.into_inner();
    let form_state = form.as_state();
    let CurrentActor(base_actor, persona) = actor;

    let error = match create_inner((*state).clone(), form, user.0, base_actor.clone()).await {
        Ok(res) => return Ok(res),
        Err(e) => e,
    };

    let (mut res, validation, system) = match error {
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
}

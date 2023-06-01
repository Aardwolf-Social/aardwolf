use aardwolf_models::{
    base_actor::{persona::Persona, BaseActor},
    user::AuthenticatedUser,
};
use aardwolf_templates::Home;
use aardwolf_types::{
    forms::posts::{
        PostCreationForm, PostCreationFormState, ValidatePostCreationFail, ValidatePostCreationForm,
    },
    operations::{
        check_create_post_permission::{CheckCreatePostPermission, CheckCreatePostPermissionFail},
        create_post::{CreatePost, CreatePostFail},
    },
    traits::{DbAction, DbActionError, Validate},
};
use rocket_i18n::I18n;
use actix_web::{
    web::{Data, Form},
    HttpResponse, ResponseError,
};
use failure::Fail;
use std::fmt;

use crate::{
    action::redirect,
    types::{actor::CurrentActor, user::SignedInUser},
    AppConfig,
    traits::{WithRucte},
};

async fn create_inner(
    state: AppConfig,
    form: PostCreationForm,
    user: AuthenticatedUser,
    base_actor: BaseActor,
) -> Result<HttpResponse, PostCreateError> {
    let form = ValidatePostCreationForm(form).validate()?;
    let creator = CheckCreatePostPermission(user, base_actor)
        .run(state.pool.clone())
        .await?;
    CreatePost(creator, form, state.generator.clone())
        .run(state.pool.clone())
        .await?;
    Ok(redirect("/"))
}

pub(crate) async fn create(
    (state, user, actor, form, i18n): (
        Data<AppConfig>,
        SignedInUser,
        CurrentActor,
        Form<PostCreationForm>,
        I18n,
    ),
) -> Result<HttpResponse, PostCreateResponseError> {
    let form = form.into_inner();
    let form_state = form.as_state();
    let CurrentActor(base_actor, persona) = actor;

    create_inner((*state).clone(), form, user.0, base_actor.clone())
        .await
        .map_err(|error| PostCreateResponseError {
            i18n,
            csrf_token: "csrf token".to_string(),
            persona,
            base_actor,
            form_state,
            error,
        })
}

#[derive(Fail)]
#[fail(display = "Error")]
pub struct PostCreateResponseError {
    i18n: I18n,
    csrf_token: String,
    persona: Persona,
    base_actor: BaseActor,
    form_state: PostCreationFormState,
    error: PostCreateError,
}

impl fmt::Debug for PostCreateResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "...")
    }
}

impl ResponseError for PostCreateResponseError {
    fn error_response(&self) -> HttpResponse {
        let (mut res, validation, system) = match self.error {
            PostCreateError::Form(ref e) => (HttpResponse::BadRequest(), Some(e), false),
            _ => (HttpResponse::InternalServerError(), None, true),
        };

        res.ructe(Home::new(
            &self.i18n.catalog,
            &self.csrf_token,
            self.persona.shortname(),
            &self.base_actor.profile_url().0.to_string(),
            &self.form_state,
            validation,
            system,
        ))
    }
}

#[derive(Clone, Debug, Fail)]
pub enum PostCreateError {
    #[fail(display = "Error talking to db actor")]
    Canceled,
    #[fail(display = "Error talking db")]
    Database,
    #[fail(display = "User does not have permission to create a persona")]
    Permission,
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
            DbActionError::Pool(_) => PostCreateError::Database,
            DbActionError::Canceled => PostCreateError::Canceled,
            DbActionError::Error(e) => e.into(),
        }
    }
}

impl From<DbActionError<CreatePostFail>> for PostCreateError {
    fn from(e: DbActionError<CreatePostFail>) -> Self {
        match e {
            DbActionError::Pool(_) => PostCreateError::Database,
            DbActionError::Canceled => PostCreateError::Canceled,
            DbActionError::Error(e) => e.into(),
        }
    }
}

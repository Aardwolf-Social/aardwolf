use aardwolf_types::{
    forms::posts::{PostCreationForm, ValidatePostCreationFail, ValidatePostCreationForm},
    operations::{
        check_create_post_permission::{CheckCreatePostPermission, CheckCreatePostPermissionFail},
        create_post::{CreatePost, CreatePostFail},
    },
};
use failure::Fail;
use rocket::{http::Status, request::Form, response::Redirect, State};
use rocket_i18n::I18n;

use crate::{
    render_template, types::actor::CurrentActor, DbConn, ResponseOrRedirect, UrlGenerator,
};

#[derive(Clone, Debug, Fail)]
pub enum PostCreateError {
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

#[post("/create", data = "<form>")]
pub(crate) fn create(
    state: State<UrlGenerator>,
    actor: CurrentActor,
    form: Form<PostCreationForm>,
    i18n: I18n,
    db_conn: DbConn,
) -> ResponseOrRedirect {
    let form = form.into_inner();
    let form_state = form.as_state();
    let CurrentActor(user, base_actor, persona) = actor;
    let base_actor2 = base_actor.clone();

    let res = perform!(&db_conn, PostCreateError, [
        (form = ValidatePostCreationForm(form)),
        (creater = CheckCreatePostPermission(user.clone(), base_actor2)),
        (_ = CreatePost(creater, form, state.clone())),
    ]);

    match res {
        Ok(_) => Redirect::to("/").into(),
        Err(e) => {
            let (status, validation, system) = match e {
                PostCreateError::Form(ref e) => (Status::BadRequest, Some(e), false),
                _ => (Status::InternalServerError, None, true),
            };

            let mut response = render_template(&aardwolf_templates::Home::new(
                &i18n.catalog,
                "csrf",
                persona.shortname(),
                &base_actor.profile_url().0.to_string(),
                &form_state,
                validation,
                system,
            ));
            response.set_status(status);
            response.into()
        }
    }
}

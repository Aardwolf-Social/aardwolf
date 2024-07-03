use aardwolf_models::{
    base_actor::{persona::Persona, BaseActor},
    user::{AuthenticatedUser, UserLike},
};
use aardwolf_types::{
    operations::{
        fetch_authenticated_user::{FetchAuthenticatedUser, FetchAuthenticatedUserFail},
        fetch_base_actor::{FetchBaseActor, FetchBaseActorFail},
        fetch_persona::{FetchPersona, FetchPersonaFail},
    },
    traits::{DbAction, DbActionError},
};
use actix_http::Payload;
use actix_session::Session;
use actix_web::{error::ResponseError, FromRequest, HttpRequest, HttpResponse};
use futures::future::{FutureExt, LocalBoxFuture, TryFutureExt};
use thiserror::Error;

use crate::{error::redirect_error, from_session, AppConfig};

pub struct CurrentActor(pub BaseActor, pub Persona);

impl FromRequest for CurrentActor {
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request<'a>(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        extract(req.clone())
            .and_then(|(state, session)| from_request_inner(state, session))
            .boxed_local()
    }
}

fn extract(
    req: HttpRequest,
) -> LocalBoxFuture<'static, Result<(AppConfig, Session), actix_web::Error>> {
    let state = req.app_data::<AppConfig>();

    match state {
        Some(state) => {
            let state = state.clone();

            Session::extract(&req)
                .map_ok(move |session| (state, session))
                .map_err(|_: _| CurrentActorError::Cookie.into())
                .boxed_local()
        }
        None => futures::future::err(MissingState.into()).boxed_local(),
    }
}

async fn from_request_inner(
    state: AppConfig,
    session: Session,
) -> Result<CurrentActor, actix_web::Error> {
    let id: i32 = match from_session(&session, "persona_id", CurrentActorError::Cookie) {
        Ok(id) => id,
        Err(_) => {
            let user_id = from_session(&session, "user_id", CurrentActorError::Cookie)?;

            fetch_user(&state, user_id)
                .await?
                .primary_persona()
                .ok_or(CurrentActorError::Persona)?
        }
    };

    let actor = fetch_actor(&state, id).await?;

    Ok(actor)
}

async fn fetch_user(state: &AppConfig, id: i32) -> Result<AuthenticatedUser, CurrentActorError> {
    Ok(FetchAuthenticatedUser(id).run(state.pool.clone()).await?)
}

async fn fetch_actor(state: &AppConfig, id: i32) -> Result<CurrentActor, CurrentActorError> {
    let persona = FetchPersona(id).run(state.pool.clone()).await?;
    let base_actor = FetchBaseActor(persona.id()).run(state.pool.clone()).await?;
    Ok(CurrentActor(base_actor, persona))
}

#[derive(Clone, Debug, Error)]
pub enum CurrentActorError {
    #[error("Error talking to db actor")]
    Canceled,
    #[error("Error in database")]
    Database,
    #[error("User doesn't exist")]
    User,
    #[error("Base Actor doesn't exist")]
    Actor,
    #[error("Persona doesn't exist")]
    Persona,
    #[error("No user cookie present")]
    Cookie,
}

impl From<DbActionError<FetchAuthenticatedUserFail>> for CurrentActorError {
    fn from(e: DbActionError<FetchAuthenticatedUserFail>) -> Self {
        match e {
            DbActionError::Pool(_) => CurrentActorError::Database,
            DbActionError::Canceled => CurrentActorError::Canceled,
            DbActionError::Error(e) => match e {
                FetchAuthenticatedUserFail::Database => CurrentActorError::Database,
                FetchAuthenticatedUserFail::NotFound => CurrentActorError::User,
            },
        }
    }
}

impl From<DbActionError<FetchBaseActorFail>> for CurrentActorError {
    fn from(e: DbActionError<FetchBaseActorFail>) -> Self {
        match e {
            DbActionError::Pool(_) => CurrentActorError::Database,
            DbActionError::Canceled => CurrentActorError::Canceled,
            DbActionError::Error(e) => match e {
                FetchBaseActorFail::Database => CurrentActorError::Database,
                FetchBaseActorFail::NotFound => CurrentActorError::Actor,
            },
        }
    }
}

impl From<DbActionError<FetchPersonaFail>> for CurrentActorError {
    fn from(e: DbActionError<FetchPersonaFail>) -> Self {
        match e {
            DbActionError::Pool(_) => CurrentActorError::Database,
            DbActionError::Canceled => CurrentActorError::Canceled,
            DbActionError::Error(e) => match e {
                FetchPersonaFail::Database => CurrentActorError::Database,
                FetchPersonaFail::NotFound => CurrentActorError::Persona,
            },
        }
    }
}

impl ResponseError for CurrentActorError {
    fn error_response(&self) -> HttpResponse {
        redirect_error("/personas/create", Some(self.to_string()))
    }
}

#[derive(Clone, Debug, Error)]
#[error("State is missing")]
pub struct MissingState;

impl ResponseError for MissingState {
    // Defaults to InternalServerError
}

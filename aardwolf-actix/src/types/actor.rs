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
    wrapper::{ExportFail, ExportKind},
};
use actix_http::Payload;
use actix_session::Session;
use actix_web::{error::ResponseError, FromRequest, HttpRequest, HttpResponse};
use failure::Fail;
use futures::{
    future::{FutureExt, TryFutureExt},
};

use crate::{db::DbActionError, error::RedirectError, from_session, AppConfig};

#[derive(Clone, Debug, Fail)]
pub enum CurrentActorError {
    #[fail(display = "Error talking to db actor")]
    Mailbox,
    #[fail(display = "Error in database")]
    Database,
    #[fail(display = "User doesn't exist")]
    User,
    #[fail(display = "Base Actor doesn't exist")]
    Actor,
    #[fail(display = "Persona doesn't exist")]
    Persona,
    #[fail(display = "No user cookie present")]
    Cookie,
    #[fail(display = "Error exporting data")]
    Export,
}

impl From<DbActionError<FetchAuthenticatedUserFail>> for CurrentActorError {
    fn from(e: DbActionError<FetchAuthenticatedUserFail>) -> Self {
        match e {
            DbActionError::Connection => CurrentActorError::Database,
            DbActionError::Mailbox => CurrentActorError::Mailbox,
            DbActionError::Action(e) => match e {
                FetchAuthenticatedUserFail::Database => CurrentActorError::Database,
                FetchAuthenticatedUserFail::NotFound => CurrentActorError::User,
            },
        }
    }
}

impl From<DbActionError<FetchBaseActorFail>> for CurrentActorError {
    fn from(e: DbActionError<FetchBaseActorFail>) -> Self {
        match e {
            DbActionError::Connection => CurrentActorError::Database,
            DbActionError::Mailbox => CurrentActorError::Mailbox,
            DbActionError::Action(e) => match e {
                FetchBaseActorFail::Database => CurrentActorError::Database,
                FetchBaseActorFail::NotFound => CurrentActorError::Actor,
            },
        }
    }
}

impl From<DbActionError<FetchPersonaFail>> for CurrentActorError {
    fn from(e: DbActionError<FetchPersonaFail>) -> Self {
        match e {
            DbActionError::Connection => CurrentActorError::Database,
            DbActionError::Mailbox => CurrentActorError::Mailbox,
            DbActionError::Action(e) => match e {
                FetchPersonaFail::Database => CurrentActorError::Database,
                FetchPersonaFail::NotFound => CurrentActorError::Persona,
            },
        }
    }
}

impl From<ExportFail> for CurrentActorError {
    fn from(_: ExportFail) -> Self {
        CurrentActorError::Export
    }
}

impl ResponseError for CurrentActorError {
    fn error_response(&self) -> HttpResponse {
        RedirectError::new("/personas/create", &Some(self.to_string())).error_response()
    }
}

#[derive(Clone, Debug, Fail)]
#[fail(display = "State is missing")]
pub struct MissingState;

impl ResponseError for MissingState {
    // Defaults to InternalServerError
}

pub struct CurrentActor(pub BaseActor, pub Persona);

async fn fetch_user(state: AppConfig, id: i32) -> Result<AuthenticatedUser, CurrentActorError> {
    Ok(perform!(state, [(_ = FetchAuthenticatedUser(id)),]))
}

async fn fetch_actor(state: AppConfig, id: i32) -> Result<CurrentActor, CurrentActorError> {
    Ok(perform!(state, [
        (persona = FetchPersona(id)),
        (base_actor = FetchBaseActor(persona.id())),
        (_ = ExportKind(CurrentActor(base_actor, persona))),
    ]))
}

fn extract(req: &HttpRequest) -> Result<(AppConfig, Session), actix_web::Error> {
    let state = req
        .app_data::<AppConfig>()
        .ok_or(MissingState)
        .map(|s| s.clone())?;

    let session = Session::extract(req).map_err(|_| CurrentActorError::Cookie)?;

    Ok((state, session))
}

async fn from_request_inner(
    state: AppConfig,
    session: Session,
) -> Result<CurrentActor, actix_web::Error> {
    let id: i32 = match from_session(&session, "persona_id", CurrentActorError::Cookie) {
        Ok(id) => id,
        Err(_) => {
            let user_id = from_session(&session, "user_id", CurrentActorError::Cookie)?;

            fetch_user(state.clone(), user_id)
                .await?
                .primary_persona()
                .ok_or(CurrentActorError::Persona)?
        }
    };

    let actor = fetch_actor(state, id).await?;

    Ok(actor)
}

impl FromRequest for CurrentActor {
    type Config = ();
    type Error = actix_web::Error;
    type Future = Box<dyn futures_old::Future<Item = Self, Error = Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        use futures_old::future::{Future, IntoFuture};
        Box::new(
            extract(req)
                .into_future()
                .and_then(|(state, session)| from_request_inner(state, session).boxed_local().compat()),
        )
    }
}

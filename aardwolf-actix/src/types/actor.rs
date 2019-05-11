use aardwolf_models::{
    base_actor::{persona::Persona, BaseActor},
    user::UserLike,
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
use actix_web::{
    error::ResponseError, FromRequest, HttpRequest,
    HttpResponse,
};
use failure::Fail;
use futures::future::{Future, IntoFuture};

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

impl FromRequest for CurrentActor {
    type Config = ();
    type Error = actix_web::Error;
    type Future = Box<dyn Future<Item = Self, Error = Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let state = match req.app_data::<AppConfig>().ok_or(MissingState) {
            Ok(state) => state.clone(),
            Err(e) => return Box::new(futures::future::err(e.into())),
        };
        let state2 = state.clone();
        let state3 = state.clone();

        let session = match Session::extract(req) {
            Ok(session) => session,
            Err(_) => return Box::new(futures::future::err(CurrentActorError::Cookie.into())),
        };

        let user_id_res = from_session(&session, "user_id", CurrentActorError::Cookie);
        let persona_id_res = from_session(&session, "persona_id", CurrentActorError::Cookie);

        let fut: Box<dyn Future<Item = i32, Error = CurrentActorError>> = match persona_id_res {
            Ok(id) => Box::new(Ok(id).into_future()),
            Err(_) => {
                let fut = user_id_res
                    .into_future()
                    .and_then(move |id| {
                        perform!(state2.clone(), CurrentActorError, [
                            (_ = FetchAuthenticatedUser(id)),
                        ])
                    })
                    .and_then(move |user| user.primary_persona().ok_or(CurrentActorError::Persona));

                Box::new(fut)
            }
        };

        let res = fut
            .and_then(move |id| {
                perform!(state3, CurrentActorError, [
                    (persona = FetchPersona(id)),
                    (base_actor = FetchBaseActor(persona.id())),
                    (_ = ExportKind(CurrentActor(base_actor, persona))),
                ])
            })
            .map_err(From::from);

        Box::new(res)
    }
}

use aardwolf_models::user::AuthenticatedUser;
use aardwolf_types::{
    operations::fetch_authenticated_user::{FetchAuthenticatedUser, FetchAuthenticatedUserFail},
    traits::{DbAction, DbActionError},
};
use actix_http::Payload;
use actix_session::Session;
use actix_web::{error::ResponseError, FromRequest, HttpRequest, HttpResponse};
use failure::Fail;
use futures::future::{FutureExt, TryFutureExt};

use crate::{error::redirect_error, from_session, AppConfig};

pub struct SignedInUser(pub AuthenticatedUser);

impl FromRequest for SignedInUser {
    type Config = ();
    type Error = actix_web::Error;
    type Future = Box<dyn futures_old::Future<Item = Self, Error = Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        use futures_old::future::{Future, IntoFuture};
        Box::new(
            extract(req).into_future().and_then(|(state, session)| {
                from_request_inner(state, session).boxed_local().compat()
            }),
        )
    }
}

fn extract(req: &HttpRequest) -> Result<(AppConfig, Session), actix_web::Error> {
    let state = req
        .app_data::<AppConfig>()
        .ok_or(MissingState)
        .map(|s| s.clone())?;

    let session = Session::extract(req).map_err(|_| SignedInUserError::Cookie)?;

    Ok((state, session))
}

async fn from_request_inner(
    state: AppConfig,
    session: Session,
) -> Result<SignedInUser, actix_web::Error> {
    let id = from_session(&session, "user_id", SignedInUserError::Cookie)?;

    let authed_user = fetch_user(state, id).await?;

    Ok(SignedInUser(authed_user))
}

async fn fetch_user(state: AppConfig, id: i32) -> Result<AuthenticatedUser, SignedInUserError> {
    Ok(FetchAuthenticatedUser(id).run(state.pool.clone()).await?)
}

#[derive(Clone, Debug, Fail)]
pub enum SignedInUserError {
    #[fail(display = "Error talking to db actor")]
    Canceled,
    #[fail(display = "Error in database")]
    Database,
    #[fail(display = "No user cookie present")]
    Cookie,
    #[fail(display = "User doesn't exist")]
    User,
}

impl From<DbActionError<FetchAuthenticatedUserFail>> for SignedInUserError {
    fn from(e: DbActionError<FetchAuthenticatedUserFail>) -> Self {
        match e {
            DbActionError::Pool(_) => SignedInUserError::Database,
            DbActionError::Canceled => SignedInUserError::Canceled,
            DbActionError::Error(e) => match e {
                FetchAuthenticatedUserFail::Database => SignedInUserError::Database,
                FetchAuthenticatedUserFail::NotFound => SignedInUserError::User,
            },
        }
    }
}

impl ResponseError for SignedInUserError {
    fn error_response(&self) -> HttpResponse {
        redirect_error("/auth/sign_in", Some(self.to_string()))
    }
}

#[derive(Clone, Debug, Fail)]
#[fail(display = "State is missing")]
pub struct MissingState;

impl ResponseError for MissingState {
    // Defaults to InternalServerError
}

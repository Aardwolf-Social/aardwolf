use aardwolf_models::user::AuthenticatedUser;
use aardwolf_types::{
    operations::fetch_authenticated_user::{FetchAuthenticatedUser, FetchAuthenticatedUserFail},
    traits::{DbAction, DbActionError},
};
use actix_http::Payload;
use actix_session::Session;
use actix_web::{error::ResponseError, FromRequest, HttpRequest, HttpResponse};
use failure::Fail;
use futures::future::{FutureExt, LocalBoxFuture, TryFutureExt};

use crate::{error::redirect_error, from_session, routes::personas::new, AppConfig};

pub struct SignedInUser(pub AuthenticatedUser);

impl FromRequest for SignedInUser {
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
                .map_ok(move |session| (state.clone(), session.clone()))
                .map_err(|_: _| SignedInUserError::Cookie.into())
                .boxed_local()
        }
        None => futures::future::err(MissingState.into()).boxed_local(),
    }
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

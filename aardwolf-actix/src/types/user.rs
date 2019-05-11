use aardwolf_models::user::AuthenticatedUser;
use aardwolf_types::operations::fetch_authenticated_user::{
    FetchAuthenticatedUser, FetchAuthenticatedUserFail,
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
pub enum SignedInUserError {
    #[fail(display = "Error talking to db actor")]
    Mailbox,
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
            DbActionError::Connection => SignedInUserError::Database,
            DbActionError::Mailbox => SignedInUserError::Mailbox,
            DbActionError::Action(e) => match e {
                FetchAuthenticatedUserFail::Database => SignedInUserError::Database,
                FetchAuthenticatedUserFail::NotFound => SignedInUserError::User,
            },
        }
    }
}

impl ResponseError for SignedInUserError {
    fn error_response(&self) -> HttpResponse {
        RedirectError::new("/auth/sign_in", &Some(self.to_string())).error_response()
    }
}

#[derive(Clone, Debug, Fail)]
#[fail(display = "State is missing")]
pub struct MissingState;

impl ResponseError for MissingState {
    // Defaults to InternalServerError
}

pub struct SignedInUser(pub AuthenticatedUser);

impl FromRequest for SignedInUser {
    type Config = ();
    type Error = actix_web::Error;
    type Future = Box<dyn Future<Item = Self, Error = Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let state = match req.app_data::<AppConfig>().ok_or(MissingState) {
            Ok(state) => state.clone(),
            Err(e) => return Box::new(futures::future::err(e.into())),
        };

        let session = match Session::extract(req) {
            Ok(session) => session,
            Err(_) => return Box::new(futures::future::err(SignedInUserError::Cookie.into())),
        };

        let id_res = from_session(&session, "user_id", SignedInUserError::Cookie);

        let res = id_res
            .into_future()
            .and_then(move |id| {
                perform!(state, SignedInUserError, [
                     (_ = FetchAuthenticatedUser(id)),
                ])
            })
            .map(SignedInUser)
            .map_err(From::from);

        Box::new(res)
    }
}

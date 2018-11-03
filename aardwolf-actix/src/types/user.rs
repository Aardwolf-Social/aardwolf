use aardwolf_models::user::{email::Email, AuthenticatedUser};
use aardwolf_types::forms::user::{GetUserAndEmailById, GetUserById, UserLookupFail};
use actix_web::{
    error::ResponseError, middleware::session::RequestSession, FromRequest, HttpRequest,
};
use futures::future::{Either, Future, IntoFuture};

use crate::{action::DbActionWrapper, error::ErrorWrapper, AppConfig};

#[derive(Clone, Debug, Fail)]
#[fail(display = "No user cookie present")]
pub struct NoUserCookie;

impl NoUserCookie {
    fn new() -> Self {
        NoUserCookie
    }
}

impl ResponseError for NoUserCookie {}

pub struct SignedInUser(pub AuthenticatedUser);
pub struct SignedInUserWithEmail(pub AuthenticatedUser, pub Email);

impl FromRequest<AppConfig> for SignedInUser {
    type Config = ();
    type Result = Box<dyn Future<Item = Self, Error = actix_web::Error>>;

    fn from_request(req: &HttpRequest<AppConfig>, _: &Self::Config) -> Self::Result {
        let state = req.state().clone();

        Box::new(
            req.session()
                .get::<i32>("user_id")
                .into_future()
                .and_then(move |maybe_id| match maybe_id {
                    Some(id) => {
                        let res = perform!(state, id, UserLookupFail, [(DbActionWrapper<_, _, _> => GetUserById::new()),]);

                        let fut = res
                            .map(SignedInUser)
                            .map_err(|e| ErrorWrapper::new(state, e))
                            .from_err();

                        Either::A(fut)
                    }
                    None => Either::B(Err(NoUserCookie::new().into()).into_future()),
                })
                .map_err(|e: actix_web::Error| e),
        )
    }
}

impl FromRequest<AppConfig> for SignedInUserWithEmail {
    type Config = ();
    type Result = Box<dyn Future<Item = Self, Error = actix_web::Error>>;

    fn from_request(req: &HttpRequest<AppConfig>, _: &Self::Config) -> Self::Result {
        let state = req.state().clone();

        Box::new(
            req.session()
                .get::<i32>("user_id")
                .into_future()
                .and_then(move |maybe_id| match maybe_id {
                    Some(id) => {
                        let res =
                            perform!(state, id, UserLookupFail, [(DbActionWrapper => GetUserAndEmailById::new()),]);
                        let fut = res
                            .map(|(user, email)| SignedInUserWithEmail(user, email))
                            .map_err(|e| ErrorWrapper::new(state, e))
                            .from_err();

                        Either::A(fut)
                    }
                    None => Either::B(Err(NoUserCookie::new().into()).into_future()),
                })
                .map_err(|e: actix_web::Error| e),
        )
    }
}

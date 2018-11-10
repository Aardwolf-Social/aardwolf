use aardwolf_models::user::{email::Email, AuthenticatedUser};
use aardwolf_types::forms::user::{FetchUser, FetchUserAndEmail, FetchUserFail};
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
use rocket::{
    http::Status,
    outcome::IntoOutcome,
    request::{self, FromRequest},
    {Outcome, Request, State},
};

use session::from_cookie;

struct CookieError;

impl From<FetchUserFail> for CookieError {
    fn from(_: FetchUserFail) -> Self {
        CookieError
    }
}

pub struct SignedInUser(pub AuthenticatedUser);
pub struct SignedInUserWithEmail(pub AuthenticatedUser, pub Email);

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

impl<'l, 'r> FromRequest<'l, 'r> for SignedInUser {
    type Error = ();

    fn from_request(request: &'l Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<Pool>>()?;
        let db = match pool.get() {
            Ok(db) => db,
            Err(_) => return Outcome::Failure((Status::ServiceUnavailable, ())),
        };

        from_cookie(&mut request.cookies(), "user_id", CookieError)
            .and_then(|user_id| {
                perform!(&db, CookieError, [
                    (_ = FetchUser(user_id)),
                ])
            })
            .map(SignedInUser)
            .ok()
            .or_forward(())
    }
}

impl<'l, 'r> FromRequest<'l, 'r> for SignedInUserWithEmail {
    type Error = ();

    fn from_request(request: &'l Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<Pool>>()?;
        let db = match pool.get() {
            Ok(db) => db,
            Err(_) => return Outcome::Failure((Status::ServiceUnavailable, ())),
        };

        from_cookie(&mut request.cookies(), "user_id", CookieError)
            .and_then(|user_id| {
                perform!(&db, CookieError, [
                    (_ = FetchUserAndEmail(user_id)),
                ])
            })
            .map(|(user, email)| SignedInUserWithEmail(user, email))
            .ok()
            .or_forward(())
    }
}

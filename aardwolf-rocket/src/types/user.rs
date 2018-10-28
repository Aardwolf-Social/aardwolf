use aardwolf_models::user::{email::Email, AuthenticatedUser, UserLike};
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
use rocket::{
    http::Status,
    outcome::IntoOutcome,
    request::{self, FromRequest},
    {Outcome, Request, State},
};

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

        request
            .cookies()
            .get_private("user_id")
            .and_then(|c| c.value().parse::<i32>().ok())
            .and_then(|user_id| AuthenticatedUser::get_authenticated_user_by_id(user_id, &db).ok())
            .map(SignedInUser)
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

        request
            .cookies()
            .get_private("user_id")
            .and_then(|c| c.value().parse::<i32>().ok())
            .and_then(|user_id| {
                AuthenticatedUser::get_authenticated_user_by_id(user_id, &db)
                    .ok()
                    .and_then(|user| {
                        user.primary_email()
                            .and_then(|primary_email| Email::by_id(primary_email, &db).ok())
                            .or_else(|| Email::first_by_user_id(user_id, &db).ok())
                            .map(|email| SignedInUserWithEmail(user, email))
                    })
            })
            .or_forward(())
    }
}

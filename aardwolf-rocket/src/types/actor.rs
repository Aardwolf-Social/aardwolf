use aardwolf_models::{
    base_actor::{persona::Persona, BaseActor},
    user::{AuthenticatedUser, UserLike},
};
use aardwolf_types::operations::{
    fetch_authenticated_user::{FetchAuthenticatedUser, FetchAuthenticatedUserFail},
    fetch_base_actor::{FetchBaseActor, FetchBaseActorFail},
    fetch_persona::{FetchPersona, FetchPersonaFail},
};
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
use rocket::{
    http::Status,
    outcome::IntoOutcome,
    request::{self, FromRequest},
    {Outcome, Request, State},
};

use crate::session::from_cookie;

struct CookieError;

impl From<FetchAuthenticatedUserFail> for CookieError {
    fn from(_: FetchAuthenticatedUserFail) -> Self {
        CookieError
    }
}

impl From<FetchPersonaFail> for CookieError {
    fn from(_: FetchPersonaFail) -> Self {
        CookieError
    }
}

impl From<FetchBaseActorFail> for CookieError {
    fn from(_: FetchBaseActorFail) -> Self {
        CookieError
    }
}

pub struct CurrentActor(pub AuthenticatedUser, pub BaseActor, pub Persona);

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

impl<'l, 'r> FromRequest<'l, 'r> for CurrentActor {
    type Error = ();

    fn from_request(request: &'l Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<Pool>>()?;
        let db = match pool.get() {
            Ok(db) => db,
            Err(_) => return Outcome::Failure((Status::ServiceUnavailable, ())),
        };

        let user_res =
            from_cookie(&mut request.cookies(), "user_id", CookieError).and_then(|user_id| {
                perform!(&db, CookieError, [
                    (_ = FetchAuthenticatedUser(user_id)),
                ])
            });

        let res = match from_cookie(&mut request.cookies(), "persona_id", CookieError) {
            Ok(id) => user_res.map(|user| (user, id)),
            Err(_) => user_res.and_then(|user| {
                user.primary_persona()
                    .map(|id| (user, id))
                    .ok_or(CookieError)
            }),
        };

        res.and_then(|(user, id)| {
            let res = perform!(&db, CookieError, [
                (_ = FetchPersona(id)),
            ]);

            res.map(|persona| (user, persona))
        })
        .and_then(|(user, persona)| {
            let res = perform!(&db, CookieError, [
                (_ = FetchBaseActor(persona.id())),
            ]);

            res.map(move |actor| CurrentActor(user, actor, persona))
        })
        .ok()
        .or_forward(())
    }
}

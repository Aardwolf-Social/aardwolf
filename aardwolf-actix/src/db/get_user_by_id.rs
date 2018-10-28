use aardwolf_models::user::{email::Email, AuthenticatedUser, UserLike};
use actix::prelude::{Handler, Message};
use actix_web::error::ResponseError;

use crate::{
    db::Db,
    types::user::{SignedInUser, SignedInUserWithEmail},
};

#[derive(Clone, Debug, Fail)]
#[fail(display = "user not found")]
pub struct NotFoundError;

impl ResponseError for NotFoundError {}

pub struct GetUserById(i32);

impl GetUserById {
    pub fn new(id: i32) -> Self {
        GetUserById(id)
    }
}

impl Message for GetUserById {
    type Result = Result<SignedInUser, NotFoundError>;
}

impl Handler<GetUserById> for Db {
    type Result = <GetUserById as Message>::Result;

    fn handle(&mut self, msg: GetUserById, _: &mut Self::Context) -> Self::Result {
        let conn = self.get_connection().map_err(|_| NotFoundError)?;

        AuthenticatedUser::get_authenticated_user_by_id(msg.0, &conn)
            .map(SignedInUser)
            .map_err(|_| NotFoundError)
    }
}

pub struct GetUserAndEmailById(i32);

impl GetUserAndEmailById {
    pub fn new(id: i32) -> Self {
        GetUserAndEmailById(id)
    }
}

impl Message for GetUserAndEmailById {
    type Result = Result<SignedInUserWithEmail, NotFoundError>;
}

impl Handler<GetUserAndEmailById> for Db {
    type Result = <GetUserAndEmailById as Message>::Result;

    fn handle(&mut self, msg: GetUserAndEmailById, _: &mut Self::Context) -> Self::Result {
        let conn = self.get_connection().map_err(|_| NotFoundError)?;

        AuthenticatedUser::get_authenticated_user_by_id(msg.0, &conn)
            .map_err(|_| NotFoundError)
            .and_then(|user| {
                user.primary_email()
                    .and_then(|primary_email| Email::by_id(primary_email, &conn).ok())
                    .or_else(|| Email::first_by_user_id(msg.0, &conn).ok())
                    .map(|email| SignedInUserWithEmail(user, email))
                    .ok_or(NotFoundError)
            })
    }
}

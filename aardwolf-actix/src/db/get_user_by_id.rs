use actix::prelude::{Handler, Message};
use aardwolf_models::user::AuthenticatedUser;

use super::Db;

pub struct GetUserById(i32);

impl GetUserById {
    pub fn new(id: i32) -> Self {
        GetUserById(id)
    }
}

impl Message for GetUserById {
    type Result = Result<AuthenticatedUser, ()>;
}

impl Handler<GetUserById> for Db {
    type Result = <GetUserById as Message>::Result;

    fn handle(&mut self, msg: GetUserById, _: &mut Self::Context) -> Self::Result {
        let conn = self.get_connection().map_err(|_| ())?;

        AuthenticatedUser::get_authenticated_user_by_id(msg.0, &conn)
            .map_err(|_| ())
    }
}

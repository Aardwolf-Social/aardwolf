use ::actix::prelude::*;
use diesel::pg::PgConnection;
use r2d2::PooledConnection;
use r2d2_diesel::ConnectionManager;
use aardwolf_models::user::AuthenticatedUser;

pub type ManagedPgConn = ConnectionManager<PgConnection>;
pub type Pool = r2d2::Pool<ManagedPgConn>;

pub struct Db(Pool);

impl Db {
    pub fn new(pool: Pool) -> Self {
        Db(pool)
    }

    fn get_connection(&self) -> Result<PooledConnection<ManagedPgConn>, r2d2::Error> {
        self.0.get()
    }
}

impl Actor for Db {
    type Context = SyncContext<Self>;
}

impl Handler<GetUserById> for Db {
    type Result = <GetUserById as Message>::Result;

    fn handle(&mut self, msg: GetUserById, _: &mut Self::Context) -> Self::Result {
        let conn = self.get_connection().map_err(|_| ())?;

        AuthenticatedUser::get_authenticated_user_by_id(msg.0, &conn)
            .map_err(|_| ())
    }
}

pub struct GetUserById(i32);

impl GetUserById {
    pub fn new(id: i32) -> Self {
        GetUserById(id)
    }
}

impl Message for GetUserById {
    type Result = Result<AuthenticatedUser, ()>;
}

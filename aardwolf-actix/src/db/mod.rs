use ::actix::prelude::*;
use diesel::pg::PgConnection;
use r2d2::PooledConnection;
use r2d2_diesel::ConnectionManager;

mod get_user_by_id;
pub use self::get_user_by_id::GetUserById;

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


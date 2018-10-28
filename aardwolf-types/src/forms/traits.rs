use diesel::pg::PgConnection;
use failure::Fail;

pub trait Validate<T, E>
where
    E: Fail,
{
    fn validate(self) -> Result<T, E>;
}

pub trait DbAction<T, E>
where
    E: Fail,
{
    fn db_action(self, conn: &PgConnection) -> Result<T, E>;
}

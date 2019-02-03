use diesel::pg::PgConnection;
use failure::Fail;

pub trait Validate {
    type Item;
    type Error: Fail;

    fn validate(self) -> Result<Self::Item, Self::Error>;
}

pub trait DbAction {
    type Item;
    type Error: Fail;

    fn db_action(self, conn: &PgConnection) -> Result<Self::Item, Self::Error>;
}

pub trait Export {
    type Item;

    fn export(self) -> Self::Item;
}

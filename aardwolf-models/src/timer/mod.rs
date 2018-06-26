<<<<<<< HEAD
use chrono::offset::Utc;
use chrono::DateTime;
use diesel;
use diesel::pg::PgConnection;
=======
use chrono::{offset::Utc, DateTime};
use diesel::{self, pg::PgConnection};
>>>>>>> origin/master

pub mod event;
pub mod event_notification;

use schema::timers;

#[derive(Debug, Identifiable, Queryable, QueryableByName)]
#[table_name = "timers"]
pub struct Timer {
    id: i32,
    fire_time: DateTime<Utc>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Timer {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn fire_time(&self) -> DateTime<Utc> {
        self.fire_time
    }
}

#[derive(Insertable)]
#[table_name = "timers"]
pub struct NewTimer {
    fire_time: DateTime<Utc>,
}

impl NewTimer {
    pub fn insert(self, conn: &PgConnection) -> Result<Timer, diesel::result::Error> {
        use diesel::prelude::*;

        diesel::insert_into(timers::table)
            .values(&self)
            .get_result(conn)
    }

    pub fn new(fire_time: DateTime<Utc>) -> Self {
        NewTimer { fire_time }
    }
}

#[cfg(test)]
mod tests {
    use test_helper::*;

    #[test]
    fn create_timer() {
        with_connection(|conn| with_timer(conn, |_| Ok(())))
    }
}

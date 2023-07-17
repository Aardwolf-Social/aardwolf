use chrono::{offset::Utc, DateTime};
use diesel::{self, pg::PgConnection};

pub mod event;
pub mod event_notification;

use crate::schema::timers;

#[derive(Debug, Identifiable, Queryable, QueryableByName)]
#[diesel(table_name = timers)]
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

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

#[derive(Insertable)]
#[diesel(table_name = timers)]
pub struct NewTimer {
    fire_time: DateTime<Utc>,
}

impl NewTimer {
    pub fn insert(self, conn: &mut PgConnection) -> Result<Timer, diesel::result::Error> {
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
    use crate::test_helper::*;

    #[test]
    fn create_timer() {
        with_connection(|conn| {
            let _ = make_timer(conn);

            Ok(())
        })
    }
}

use chrono::{offset::Utc, DateTime};
use diesel::{self, pg::PgConnection};

use crate::{
    schema::event_notifications,
    timer::{event::Event, Timer},
};

#[derive(Debug, Identifiable, Queryable, QueryableByName)]
#[diesel(table_name = event_notifications)]
pub struct EventNotification {
    id: i32,
    event_id: i32, // foreign key to Event
    timer_id: i32, // foreign key to Timer
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl EventNotification {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn event_id(&self) -> i32 {
        self.event_id
    }

    pub fn timer_id(&self) -> i32 {
        self.timer_id
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

#[derive(Insertable)]
#[diesel(table_name = event_notifications)]
pub struct NewEventNotification {
    event_id: i32,
    timer_id: i32,
}

impl NewEventNotification {
    pub fn insert(
        self,
        conn: &mut PgConnection,
    ) -> Result<EventNotification, diesel::result::Error> {
        use diesel::prelude::*;

        diesel::insert_into(event_notifications::table)
            .values(&self)
            .get_result(conn)
    }

    /// TODO: Maybe fail if notification is scheduled after event starts
    pub fn new(event: &Event, timer: &Timer) -> Self {
        NewEventNotification {
            event_id: event.id(),
            timer_id: timer.id(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_helper::*;

    #[test]
    fn create_event() {
        with_connection(|conn| {
            let t1 = make_timer(conn)?;
            let t2 = make_timer(conn)?;
            let t3 = make_timer(conn)?;

            let (start, end) = if t1.fire_time() < t2.fire_time() {
                (t1, t2)
            } else {
                (t2, t1)
            };

            let (notif, start, end) = if t3.fire_time() < start.fire_time() {
                (t3, start, end)
            } else if t3.fire_time() < end.fire_time() {
                (start, t3, end)
            } else {
                (start, end, t3)
            };

            let owner_base = make_base_actor(conn)?;
            let owner = make_persona(conn, &owner_base)?;
            let event = make_event(conn, &owner, &start, &end)?;

            let _ = make_event_notification(conn, &event, &notif)?;

            Ok(())
        })
    }
}
